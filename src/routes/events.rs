//! Events routes.

use crate::{
    db::{self, Event},
    shared::{AppError, AppState},
};
use axum::{Json, extract::State};
use std::sync::Arc;
use utoipa_axum::{router::OpenApiRouter, routes};

/// Register routes.
pub fn router(state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(get_events))
        .with_state(state)
}

/// Retrieve events.
#[utoipa::path(
    get,
    path = "/events/",
    responses(
        (status = 200, description = "Events", body = [Event])
    )
)]
async fn get_events(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Event>>, AppError> {
    let events = db::get_events(&state.cobalt_db).await?;
    Ok(Json(events))
}
