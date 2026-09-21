//! Object storage for event banners.

use crate::shared::AppError;
use base64::Engine;
use chrono::Utc;
use hmac::{Hmac, KeyInit, Mac};
use http::{HeaderMap, HeaderName, HeaderValue, header::CACHE_CONTROL};
use image::GenericImageView;
use rand::Rng;
use s3::{Bucket, Region, creds::Credentials};
use sha2::Sha256;
use std::env;

/// Maximum accepted banner upload size in bytes.
pub const MAX_BANNER_BYTES: usize = 8 * 1024 * 1024;

/// Target banner aspect ratio (width / height).
const BANNER_ASPECT_RATIO: f64 = 16.0 / 9.0;
const BANNER_ASPECT_TOLERANCE: f64 = 0.02;

const KEY_PREFIX: &str = "event-banners";

/// A valid banner.
pub struct DecodedBanner {
    bytes: Vec<u8>,
    extension: &'static str,
    content_type: &'static str,
}

/// Validate an uploaded banner.
///
/// Order matters: size is checked before any decode work is attempted so an
/// oversized upload is rejected cheaply rather than after decompression. The
/// bytes must then decode as a real PNG/JPEG/GIF/WebP image — this is the
/// control that stops a stored-XSS payload (e.g. SVG/HTML) disguised with an
/// image extension or `Content-Type`, so it isn't optional hardening.
/// Finally, its dimensions must fall within `BANNER_ASPECT_TOLERANCE` of
/// `BANNER_ASPECT_RATIO`.
pub fn validate_banner_image(bytes: Vec<u8>) -> Result<DecodedBanner, AppError> {
    if bytes.is_empty() {
        return Err(AppError::BadRequest("banner image must not be empty"));
    }
    if bytes.len() > MAX_BANNER_BYTES {
        return Err(AppError::BadRequest("banner image exceeds the 8MB limit"));
    }

    let format = image::guess_format(&bytes)
        .map_err(|_| AppError::BadRequest("unrecognized image format"))?;
    let (extension, content_type) = match format {
        image::ImageFormat::Png => ("png", "image/png"),
        image::ImageFormat::Jpeg => ("jpg", "image/jpeg"),
        image::ImageFormat::Gif => ("gif", "image/gif"),
        image::ImageFormat::WebP => ("webp", "image/webp"),
        _ => {
            return Err(AppError::BadRequest(
                "unsupported image format; use PNG, JPEG, GIF, or WebP",
            ));
        }
    };

    let decoded = image::load_from_memory_with_format(&bytes, format)
        .map_err(|_| AppError::BadRequest("could not decode banner image"))?;
    let (width, height) = decoded.dimensions();
    if width == 0 || height == 0 {
        return Err(AppError::BadRequest("banner image has invalid dimensions"));
    }
    let ratio = f64::from(width) / f64::from(height);
    if (ratio - BANNER_ASPECT_RATIO).abs() > BANNER_ASPECT_TOLERANCE {
        return Err(AppError::BadRequest(
            "banner image must be approximately 16:9",
        ));
    }

    Ok(DecodedBanner {
        bytes,
        extension,
        content_type,
    })
}

/// Create a random key.
fn banner_key(facility: &str, extension: &str) -> String {
    let mut random_bytes = [0u8; 16];
    rand::rng().fill_bytes(&mut random_bytes);
    format!(
        "{KEY_PREFIX}/{}/{}.{extension}",
        facility.to_lowercase(),
        hex::encode(random_bytes)
    )
}

/// Which object storage service to talk to. The same built image is
/// deployed to both DOKS (Spaces) and AKS (Azure Blob) at once during the
/// migration coexistence window (see `gitops`' `docs/migration-steps.md`
/// §3), so this has to be an explicit runtime switch rather than a
/// compile-time choice — mirrors cobalt's `config.StorageProvider()`.
enum Backend {
    Spaces(Box<Bucket>),
    AzureBlob {
        endpoint: String,
        account: String,
        account_key: String,
        container: String,
        http: reqwest::Client,
    },
}

/// Object storage client for event banner uploads — DigitalOcean Spaces or
/// Azure Blob Storage, selected by `STORAGE_PROVIDER`.
pub struct EventBannerStorage {
    backend: Backend,
    public_base_url: String,
}

impl EventBannerStorage {
    /// Build the storage client. `STORAGE_PROVIDER` selects the backend —
    /// `spaces` (the default, for existing DOKS deployments) or
    /// `azure_blob`.
    ///
    /// **Spaces** (`DO_SPACES_*`): `DO_SPACES_KEY`, `DO_SPACES_SECRET`,
    /// `DO_SPACES_BUCKET` (required), `DO_SPACES_REGION` (defaults to
    /// `sfo3`, matching cobalt's default), `DO_SPACES_ENDPOINT` (defaults to
    /// `https://<region>.digitaloceanspaces.com`), and
    /// `DO_SPACES_PUBLIC_BASE_URL` (defaults to
    /// `https://<bucket>.<region>.digitaloceanspaces.com`).
    /// `DO_SPACES_PATH_STYLE=true` switches to path-style addressing
    /// (`<endpoint>/<bucket>/...` instead of `<bucket>.<endpoint>/...`),
    /// which real Spaces doesn't need but S3-compatible test servers like
    /// MinIO require.
    ///
    /// **Azure Blob** (`AZURE_STORAGE_*`): `AZURE_STORAGE_ACCOUNT`,
    /// `AZURE_STORAGE_KEY`, `AZURE_STORAGE_CONTAINER` (required),
    /// `AZURE_STORAGE_ENDPOINT` (defaults to
    /// `https://<account>.blob.core.windows.net`), and
    /// `AZURE_STORAGE_PUBLIC_BASE_URL` (defaults to `<endpoint>/<container>`).
    pub fn connect() -> Result<Self, AppError> {
        let provider = env::var("STORAGE_PROVIDER").unwrap_or_else(|_| "spaces".to_string());
        if provider == "azure_blob" {
            Self::connect_azure_blob()
        } else {
            Self::connect_spaces()
        }
    }

    fn connect_spaces() -> Result<Self, AppError> {
        let key = env::var("DO_SPACES_KEY")?;
        let secret = env::var("DO_SPACES_SECRET")?;
        let bucket_name = env::var("DO_SPACES_BUCKET")?;
        let region = env::var("DO_SPACES_REGION").unwrap_or_else(|_| "sfo3".to_string());
        let endpoint = env::var("DO_SPACES_ENDPOINT")
            .unwrap_or_else(|_| format!("https://{region}.digitaloceanspaces.com"));
        let public_base_url = env::var("DO_SPACES_PUBLIC_BASE_URL")
            .unwrap_or_else(|_| format!("https://{bucket_name}.{region}.digitaloceanspaces.com"));
        let path_style = env::var("DO_SPACES_PATH_STYLE").is_ok_and(|v| v == "true");

        let credentials = Credentials::new(Some(&key), Some(&secret), None, None, None)
            .map_err(s3::error::S3Error::from)?;
        let bucket = Bucket::new(
            &bucket_name,
            Region::Custom { region, endpoint },
            credentials,
        )?;
        let bucket = if path_style {
            bucket.with_path_style()
        } else {
            bucket
        };

        Ok(Self {
            backend: Backend::Spaces(bucket),
            public_base_url,
        })
    }

    fn connect_azure_blob() -> Result<Self, AppError> {
        let account = env::var("AZURE_STORAGE_ACCOUNT")?;
        let account_key = env::var("AZURE_STORAGE_KEY")?;
        let container = env::var("AZURE_STORAGE_CONTAINER")?;
        let endpoint = env::var("AZURE_STORAGE_ENDPOINT")
            .unwrap_or_else(|_| format!("https://{account}.blob.core.windows.net"));
        let public_base_url = env::var("AZURE_STORAGE_PUBLIC_BASE_URL")
            .unwrap_or_else(|_| format!("{endpoint}/{container}"));

        Ok(Self {
            backend: Backend::AzureBlob {
                endpoint,
                account,
                account_key,
                container,
                http: reqwest::Client::new(),
            },
            public_base_url,
        })
    }

    /// Upload a validated banner image and return its public URL.
    pub async fn upload(&self, facility: &str, banner: &DecodedBanner) -> Result<String, AppError> {
        let key = banner_key(facility, banner.extension);

        match &self.backend {
            Backend::Spaces(bucket) => {
                let mut headers = HeaderMap::new();
                headers.insert(
                    HeaderName::from_static("x-amz-acl"),
                    HeaderValue::from_static("public-read"),
                );
                headers.insert(
                    CACHE_CONTROL,
                    HeaderValue::from_static("public, max-age=31536000, immutable"),
                );

                let response = bucket
                    .put_object_with_content_type_and_headers(
                        &key,
                        &banner.bytes,
                        banner.content_type,
                        Some(headers),
                    )
                    .await?;
                if response.status_code() >= 300 {
                    tracing::error!(status = response.status_code(), key, "banner upload failed");
                    return Err(AppError::Internal("failed to upload banner image"));
                }
            }
            Backend::AzureBlob {
                endpoint,
                account,
                account_key,
                container,
                http,
            } => {
                upload_azure_blob(
                    http,
                    endpoint,
                    account,
                    account_key,
                    container,
                    &key,
                    banner,
                )
                .await?;
            }
        }

        Ok(format!(
            "{}/{key}",
            self.public_base_url.trim_end_matches('/')
        ))
    }
}

/// Upload to Azure Blob Storage using the Shared Key authorization scheme —
/// the Blob-service equivalent of Spaces' SigV4 signing, hand-rolled rather
/// than pulling in the Azure SDK for the same reason this file doesn't pull
/// in the AWS SDK for Spaces: exactly one kind of call is made here (put a
/// whole block blob in one shot), and Shared Key's string-to-sign is simple
/// enough that a small dependency (`hmac`/`sha2`/`base64`, all already
/// in the dependency tree) covers it.
///
/// https://learn.microsoft.com/en-us/rest/api/storageservices/authorize-with-shared-key
async fn upload_azure_blob(
    http: &reqwest::Client,
    endpoint: &str,
    account: &str,
    account_key: &str,
    container: &str,
    key: &str,
    banner: &DecodedBanner,
) -> Result<(), AppError> {
    const X_MS_VERSION: &str = "2021-08-06";

    let blob_path = format!("{container}/{key}");
    let url = format!("{endpoint}/{blob_path}");
    let content_length = banner.bytes.len();
    let x_ms_date = Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string();

    let authorization = sign_azure_blob_request(
        account,
        account_key,
        &blob_path,
        banner.content_type,
        content_length,
        &x_ms_date,
        X_MS_VERSION,
    )
    .map_err(|_| AppError::Internal("failed to sign azure blob request"))?;

    let response = http
        .put(&url)
        .header("Content-Type", banner.content_type)
        .header("x-ms-blob-type", "BlockBlob")
        .header("x-ms-date", &x_ms_date)
        .header("x-ms-version", X_MS_VERSION)
        // Same rationale as the Spaces path: arbitrary user-uploaded file
        // types and never-rewritten, unguessable keys.
        .header("X-Content-Type-Options", "nosniff")
        .header("Cache-Control", "public, max-age=31536000, immutable")
        .header("Authorization", authorization)
        .body(banner.bytes.clone())
        .send()
        .await
        .map_err(|err| {
            tracing::error!(error = %err, key, "banner upload to azure blob failed");
            AppError::Internal("failed to upload banner image")
        })?;

    if !response.status().is_success() {
        tracing::error!(status = %response.status(), key, "banner upload to azure blob failed");
        return Err(AppError::Internal("failed to upload banner image"));
    }

    Ok(())
}

type HmacSha256 = Hmac<Sha256>;

/// Build the `Authorization: SharedKey ...` header value for a single PUT
/// Blob request. `blob_path` is `<container>/<key>` (no leading slash).
fn sign_azure_blob_request(
    account: &str,
    account_key: &str,
    blob_path: &str,
    content_type: &str,
    content_length: usize,
    x_ms_date: &str,
    x_ms_version: &str,
) -> Result<String, AppError> {
    let key_bytes = base64::engine::general_purpose::STANDARD
        .decode(account_key)
        .map_err(|_| AppError::Internal("invalid azure storage account key"))?;

    // CanonicalizedHeaders: every x-ms-* header, lowercase name, sorted,
    // "name:value\n" each — blob-type < date < version alphabetically, so
    // this fixed order matches what sorting them would produce.
    let canonicalized_headers =
        format!("x-ms-blob-type:BlockBlob\nx-ms-date:{x_ms_date}\nx-ms-version:{x_ms_version}\n");
    // CanonicalizedResource: account + the resource path, no query string
    // (this request has none).
    let canonicalized_resource = format!("/{account}/{blob_path}");

    let content_length_field = if content_length > 0 {
        content_length.to_string()
    } else {
        String::new()
    };

    let string_to_sign = [
        "PUT",                 // Verb
        "",                    // Content-Encoding
        "",                    // Content-Language
        &content_length_field, // Content-Length
        "",                    // Content-MD5
        content_type,          // Content-Type
        "",                    // Date (x-ms-date is used instead)
        "",                    // If-Modified-Since
        "",                    // If-Match
        "",                    // If-None-Match
        "",                    // If-Unmodified-Since
        "",                    // Range
    ]
    .join("\n")
        + "\n"
        + &canonicalized_headers
        + &canonicalized_resource;

    let mut mac = HmacSha256::new_from_slice(&key_bytes)
        .map_err(|_| AppError::Internal("invalid azure storage account key"))?;
    mac.update(string_to_sign.as_bytes());
    let signature = base64::engine::general_purpose::STANDARD.encode(mac.finalize().into_bytes());

    Ok(format!("SharedKey {account}:{signature}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    // End-to-end vector for Azure Blob's Shared Key authorization scheme,
    // independently computed via openssl/python against Azurite's
    // well-known emulator account key (never used against real Azure —
    // picked only because it's a public, well-known key, so this test has
    // no real secret in it). Mirrors cobalt's storage/azure_blob_test.go.
    #[test]
    fn sign_azure_blob_request_matches_known_vector() {
        let account = "devstoreaccount1";
        let account_key = "Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==";
        let blob_path = "vatusa-events/event-banners/zdv/deadbeefdeadbeefdeadbeefdeadbeef.png";

        let authorization = sign_azure_blob_request(
            account,
            account_key,
            blob_path,
            "image/png",
            4,
            "Wed, 09 Sep 2026 12:00:00 GMT",
            "2021-08-06",
        )
        .unwrap();

        assert_eq!(
            authorization,
            "SharedKey devstoreaccount1:QG91JR0xNMn4RSyFpSJSSmWYJ7VE+vrEKE2MXmrmbZE="
        );
    }

    #[test]
    fn sign_azure_blob_request_rejects_bad_key() {
        let result = sign_azure_blob_request(
            "account",
            "not-valid-base64!!",
            "container/key",
            "application/octet-stream",
            0,
            "Wed, 09 Sep 2026 12:00:00 GMT",
            "2021-08-06",
        );
        assert!(result.is_err());
    }
}
