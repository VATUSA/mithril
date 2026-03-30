//! Events routes.

use crate::{
    db::{self, Event},
    shared::{AppError, AppState},
};
use axum::{Json, Router, extract::State, routing::get};
use std::sync::Arc;

/// Register routes.
pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get_events))
}

/// Retrieve events.
async fn get_events(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Event>>, AppError> {
    let events = db::get_events(&state.cobalt_db).await?;
    Ok(Json(events))
}
