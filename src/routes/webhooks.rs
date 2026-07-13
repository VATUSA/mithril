//! Webhook routes.

use crate::{
    middleware::RequireAuth,
    queries::{self, CreateWebhook},
    shared::{AppError, AppState, determine_facility},
};
use axum::{
    Json,
    extract::{self, State},
};
use http::StatusCode;
use rand::Rng;
use serde::Serialize;
use std::sync::Arc;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

/// Register routes.
pub fn router(state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(create_webhook))
        .with_state(state)
}

/// Generate a random 32-byte, hex-encoded webhook signing secret.
fn generate_webhook_secret() -> String {
    let mut bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut bytes);
    hex::encode(bytes)
}

#[derive(Debug, Serialize, ToSchema)]
struct CreateWebhookResponse {
    secret: String,
}

/// Create a webhook for the calling facility.
///
/// The returned `secret` is used to verify the `X-Mithril-Signature` header
/// sent with each webhook delivery: it's an HMAC-SHA256 hex digest of the
/// request body, keyed with this secret. This secret is only returned once;
/// it is not retrievable again.
#[utoipa::path(
    post,
    path = "/",
    tag = "webhooks",
    request_body = CreateWebhook,
    responses(
        (status = 201, description = "Webhook created", body = CreateWebhookResponse),
        (status = 400, description = "Malformed request"),
        (status = 401, description = "Must be called with an API key"),
        (status = 405, description = "Must be called with POST"),
        (status = 422, description = "Malformed request body"),
        (status = 500, description = "Server error")
    ),
    security(
        ("api_key" = [])
    )
)]
async fn create_webhook(
    State(state): State<Arc<AppState>>,
    auth: RequireAuth,
    extract::Json(data): extract::Json<CreateWebhook>,
) -> Result<(StatusCode, Json<CreateWebhookResponse>), AppError> {
    if !data.url.starts_with("https://") {
        return Err(AppError::BadRequest("webhook url must use https"));
    }

    let facility = determine_facility(&auth, &data)?;
    let secret = generate_webhook_secret();

    if !auth.testing {
        let id = queries::create_webhook(&state.cobalt_db, &facility, &data.url, &secret).await?;
        tracing::info!(
            "key {} created webhook {} for facility {}",
            auth.key_id,
            id,
            facility
        );
        Ok((StatusCode::CREATED, Json(CreateWebhookResponse { secret })))
    } else {
        tracing::debug!(
            "testing key {} used on create webhook endpoint",
            auth.key_id
        );
        Ok((StatusCode::CREATED, Json(CreateWebhookResponse { secret })))
    }
}
