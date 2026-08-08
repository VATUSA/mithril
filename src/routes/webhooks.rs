//! Webhook routes.

use crate::{
    middleware::RequireAuth,
    queries::{self, CreateWebhook, WebhookInfo},
    shared::{AppError, AppState, determine_facility},
};
use axum::{
    Json,
    extract::{self, Path, State},
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
        .routes(routes!(get_webhooks, create_webhook))
        .routes(routes!(delete_webhook))
        .with_state(state)
}

/// Generate a random 32-byte, hex-encoded webhook signing secret.
fn generate_webhook_secret() -> String {
    let mut bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut bytes);
    hex::encode(bytes)
}

/// Get webhooks for the calling facility.
#[utoipa::path(
    get,
    path = "/",
    tag = "webhooks",
    responses(
        (status = 200, description = "Registered webhooks", body = [WebhookInfo]),
        (status = 400, description = "Malformed request"),
        (status = 401, description = "Must be called with an API key"),
        (status = 500, description = "Server error")
    ),
    security(
        ("api_key" = [])
    )
)]
async fn get_webhooks(
    State(state): State<Arc<AppState>>,
    auth: RequireAuth,
) -> Result<Json<Vec<WebhookInfo>>, AppError> {
    let facility = auth.facility.as_deref().unwrap_or_default();
    let webhooks = queries::get_webhooks_for_facility(&state.cobalt_db, facility).await?;
    Ok(Json(webhooks))
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

/// Delete an existing webhook for the calling facility.
#[utoipa::path(
    delete,
    path = "/{id}",
    tag = "webhooks",
    responses(
        (status = 204, description = "Webhook deleted"),
        (status = 401, description = "Must be called with an API key"),
        (status = 403, description = "Webhook not in your facility"),
        (status = 404, description = "No matching webhook found"),
        (status = 405, description = "Must be called with DELETE"),
        (status = 500, description = "Server error")
    ),
    params(
        ("id" = i32, Path, description = "Webhook ID")
    ),
    security(
        ("api_key" = [])
    )
)]
async fn delete_webhook(
    Path(id): Path<i32>,
    State(state): State<Arc<AppState>>,
    auth: RequireAuth,
) -> Result<StatusCode, AppError> {
    let webhook = queries::get_webhook(&state.cobalt_db, id).await?;
    let Some(webhook) = webhook else {
        return Err(AppError::NotFound("webhook not found"));
    };
    if webhook.facility.as_deref() != auth.facility.as_deref() {
        tracing::info!(
            "key {} tried to delete webhook for '{}'",
            auth.key_id,
            webhook.facility.unwrap_or_default()
        );
        return Err(AppError::InsufficientPermissions);
    }
    if !auth.testing {
        queries::delete_webhook(&state.cobalt_db, id).await?;
        tracing::info!("key {} used to delete webhook {}", auth.key_id, webhook.id);
    } else {
        tracing::debug!(
            "testing key {} called on webhook delete endpoint",
            auth.key_id
        );
    }
    Ok(StatusCode::NO_CONTENT)
}
