//! News routes.

use crate::{
    db::NewsPost,
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
        .routes(routes!(get_news, get_single_news))
        .with_state(state)
}

/// Retrieve news posts.
#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "News", body = [NewsPost])
    )
)]
async fn get_news(State(state): State<Arc<AppState>>) -> Result<Json<Vec<NewsPost>>, AppError> {
    let news = queries::get_news_posts(&state.cobalt_db).await?;
    Ok(Json(news))
}

#[utoipa::path(
    get,
    path = "/{id}",
    responses(
        (status = 200, description = "News post", body = NewsPost)
    )
)]
async fn get_single_news(
    Path(id): Path<i32>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Option<NewsPost>>, AppError> {
    let news = queries::get_news_post(&state.cobalt_db, id).await?;
    Ok(Json(news))
}

async fn update_news() {}

async fn delete_news() {}
