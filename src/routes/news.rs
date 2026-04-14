//! News routes.

use crate::{
    db::NewsPost,
    queries::{self, CreateNewsPost},
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
        .routes(routes!(get_news, create_news))
        .routes(routes!(get_single_news))
        .with_state(state)
}

/// Retrieve news posts.
///
/// Public route.
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

/// Retrieve a single news post.
///
/// Public route.
#[utoipa::path(
    get,
    path = "/{id}",
    responses(
        (status = 200, description = "News post", body = NewsPost),
        (status = 404, description = "No matching news post found")
    )
)]
async fn get_single_news(
    Path(id): Path<i32>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<NewsPost>, AppError> {
    let news = queries::get_news_post(&state.cobalt_db, id).await?;
    match news {
        Some(n) => Ok(Json(n)),
        None => Err(AppError::NotFound("No news post with given id found")),
    }
}

#[utoipa::path(
    post,
    path = "/",
    request_body = CreateNewsPost,
    responses(
        (status = 204, description = "News post created"),
        (status = 400, description = "Malformed request"),
        (status = 401, description = "Must be called with an API key"),
        (status = 405, description = "Must be called with POST"),
        (status = 500, description = "Server error")
    )
)]
async fn create_news() {}

async fn update_news() {}

async fn delete_news() {}
