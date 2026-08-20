//! Object storage for event banners.

use crate::shared::AppError;
use http::{HeaderMap, HeaderName, HeaderValue, header::CACHE_CONTROL};
use image::GenericImageView;
use rand::Rng;
use s3::{Bucket, Region, creds::Credentials};
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

/// DigitalOcean Spaces client for event banner uploads.
pub struct EventBannerStorage {
    bucket: Box<Bucket>,
    public_base_url: String,
}

impl EventBannerStorage {
    /// Build the storage client from `DO_SPACES_*` environment variables:
    ///
    /// `DO_SPACES_KEY`, `DO_SPACES_SECRET`, `DO_SPACES_BUCKET` (required),
    /// `DO_SPACES_REGION` (defaults to `sfo3`, matching cobalt's default),
    /// `DO_SPACES_ENDPOINT` (defaults to `https://<region>.digitaloceanspaces.com`), and
    /// `DO_SPACES_PUBLIC_BASE_URL` (defaults to `https://<bucket>.<region>.digitaloceanspaces.com`).
    ///
    /// `DO_SPACES_PATH_STYLE=true` switches to path-style addressing
    /// (`<endpoint>/<bucket>/...` instead of `<bucket>.<endpoint>/...`),
    /// which real Spaces doesn't need but S3-compatible test servers like
    /// MinIO require.
    pub fn connect() -> Result<Self, AppError> {
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
            bucket,
            public_base_url,
        })
    }

    /// Upload a validated banner image and return its public URL.
    pub async fn upload(&self, facility: &str, banner: &DecodedBanner) -> Result<String, AppError> {
        let key = banner_key(facility, banner.extension);

        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("x-amz-acl"),
            HeaderValue::from_static("public-read"),
        );
        headers.insert(
            CACHE_CONTROL,
            HeaderValue::from_static("public, max-age=31536000, immutable"),
        );

        let response = self
            .bucket
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

        Ok(format!(
            "{}/{key}",
            self.public_base_url.trim_end_matches('/')
        ))
    }
}
