//! News routes.

use crate::{
    db::{self, NewsPost},
    shared::{AppError, AppState},
};
use axum::{Json, Router, extract::State, routing::get};
use std::sync::Arc;

/// Register routes.
pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get_news))
}

/// Retrieve news posts.
async fn get_news(State(state): State<Arc<AppState>>) -> Result<Json<Vec<NewsPost>>, AppError> {
    let news = db::get_news(&state.cobalt_db).await?;
    Ok(Json(news))
}
