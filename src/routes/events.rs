//! Events routes.

use crate::{
    db::Event,
    queries,
    shared::{AppError, AppState},
};
use axum::{
    Json,
    extract::{Path, State},
};
use std::sync::Arc;
use utoipa_axum::{router::OpenApiRouter, routes};

/// Register routes.
pub fn router(state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(get_events))
        .routes(routes!(get_event))
        .with_state(state)
}

/// Retrieve events.
#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Events", body = [Event])
    )
)]
async fn get_events(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Event>>, AppError> {
    let events = queries::get_events(&state.cobalt_db).await?;
    Ok(Json(events))
}

#[utoipa::path(
    get,
    path = "/{id}",
    responses(
        (status = 200, description = "Event", body = Event),
        (status = 404, description = "No matching event found")
    )
)]
async fn get_event(
    Path(id): Path<i32>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Event>, AppError> {
    let event = queries::get_event(&state.cobalt_db, id).await?;
    match event {
        Some(e) => Ok(Json(e)),
        None => Err(AppError::NotFound("No event with given id found")),
    }
}

async fn create_event() {}

async fn edit_event() {}

async fn delete_event() {}
