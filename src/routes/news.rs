//! News routes.

// It may be advantageous to limiting updates and deletes to the facility
// that created them (with ZHQ override), but that data isn't stored into
// the underlying table.

use crate::{
    db::NewsPost,
    middleware::RequireAuth,
    queries::{self, CreateNewsPost, UpdateNewsPost},
    shared::{AppError, AppState},
};
use axum::{
    Json,
    extract::{self, Path, State},
};
use http::StatusCode;
use std::sync::Arc;
use utoipa_axum::{router::OpenApiRouter, routes};

/// Register routes.
pub fn router(state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(get_news, create_news))
        .routes(routes!(get_single_news, update_news, delete_news))
        .with_state(state)
}

/// Retrieve news posts
#[utoipa::path(
    get,
    path = "/",
    tag = "news",
    responses(
        (status = 200, description = "News", body = [NewsPost]),
        (status = 500, description = "Server error")
    )
)]
async fn get_news(State(state): State<Arc<AppState>>) -> Result<Json<Vec<NewsPost>>, AppError> {
    let news = queries::get_news_posts(&state.cobalt_db).await?;
    Ok(Json(news))
}

/// Retrieve a single news post
#[utoipa::path(
    get,
    path = "/{id}",
    tag = "news",
    responses(
        (status = 200, description = "News post", body = NewsPost),
        (status = 404, description = "No matching news post found"),
        (status = 500, description = "Server error")
    ),
    params(
        ("id" = i32, Path, description = "News post ID")
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

/// Create a new news posting
#[utoipa::path(
    post,
    path = "/",
    tag = "news",
    request_body = CreateNewsPost,
    responses(
        (status = 201, description = "News post created"),
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
async fn create_news(
    State(state): State<Arc<AppState>>,
    auth: RequireAuth,
    extract::Json(data): extract::Json<CreateNewsPost>,
) -> Result<StatusCode, AppError> {
    // validate data.author_cid in key's facility
    let facility = auth
        .facility
        .as_deref()
        .ok_or(AppError::InsufficientPermissions)?;
    if facility != "ZHQ" {
        let author_on_roster =
            queries::cid_in_facility(&state.vatusa_db, data.author_cid, facility).await?;
        if !author_on_roster {
            tracing::warn!(
                "key {} tried to create a news post for {} in another facility",
                auth.key_id,
                data.author_cid
            );
            return Err(AppError::InsufficientPermissions);
        }
    }
    // create
    if !auth.testing {
        let id = queries::create_news_post(&state.cobalt_db, &data).await?;
        tracing::info!(
            "key {} used to create news post {} by {}: '{}'",
            auth.key_id,
            id,
            data.author_cid,
            data.title
        );
    } else {
        tracing::debug!(
            "testing key {} used on create news post endpoint",
            auth.key_id
        )
    }
    Ok(StatusCode::CREATED)
}

/// Update an existing news posting
#[utoipa::path(
    patch,
    path = "/{id}",
    tag = "news",
    request_body = UpdateNewsPost,
    responses(
        (status = 200, description = "News post updated"),
        (status = 400, description = "Malformed request"),
        (status = 401, description = "Must be called with an API key"),
        (status = 404, description = "No matching news post found"),
        (status = 405, description = "Must be called with PATCH"),
        (status = 422, description = "Malformed request body"),
        (status = 500, description = "Server error")
    ),
    params(
        ("id" = i32, Path, description = "News post ID")
    ),
    security(
        ("api_key" = [])
    )
)]
async fn update_news(
    Path(id): Path<i32>,
    State(state): State<Arc<AppState>>,
    auth: RequireAuth,
    extract::Json(data): extract::Json<UpdateNewsPost>,
) -> Result<StatusCode, AppError> {
    // ensure news post exists
    let news = queries::get_news_post(&state.cobalt_db, id).await?;
    let news = match news {
        Some(n) => n,
        None => {
            return Err(AppError::NotFound("news post not found"));
        }
    };
    // validate data.author_cid in key's facility
    let facility = auth
        .facility
        .as_deref()
        .ok_or(AppError::InsufficientPermissions)?;
    if facility != "ZHQ" {
        let author_on_roster =
            queries::cid_in_facility(&state.vatusa_db, news.author_cid, facility).await?;
        if !author_on_roster {
            tracing::warn!(
                "key {} tried to update news post {} in another facility",
                auth.key_id,
                news.id,
            );
            return Err(AppError::InsufficientPermissions);
        }
    }
    // update
    if !auth.testing {
        queries::update_news_post(&state.cobalt_db, id, &data).await?;
        tracing::info!("key {} used to update news post {}", auth.key_id, news.id);
    } else {
        tracing::debug!("testing key {} called on news update endpoint", auth.key_id);
    }
    Ok(StatusCode::OK)
}

/// Delete an existing news posting
#[utoipa::path(
    delete,
    path = "/{id}",
    tag = "news",
    responses(
        (status = 204, description = "News post deleted"),
        (status = 400, description = "Malformed request"),
        (status = 401, description = "Must be called with an API key"),
        (status = 404, description = "No matching news post found"),
        (status = 405, description = "Must be called with DELETE"),
        (status = 422, description = "Malformed request body"),
        (status = 500, description = "Server error")
    ),
    params(
        ("id" = i32, Path, description = "News post ID")
    ),
    security(
        ("api_key" = [])
    )
)]
async fn delete_news(
    Path(id): Path<i32>,
    State(state): State<Arc<AppState>>,
    auth: RequireAuth,
) -> Result<StatusCode, AppError> {
    // ensure news post exists
    let news = queries::get_news_post(&state.cobalt_db, id).await?;
    let news = match news {
        Some(n) => n,
        None => {
            return Err(AppError::NotFound("news post not found"));
        }
    };
    // validate data.author_cid in key's facility
    let facility = auth
        .facility
        .as_deref()
        .ok_or(AppError::InsufficientPermissions)?;
    if facility != "ZHQ" {
        let author_on_roster =
            queries::cid_in_facility(&state.vatusa_db, news.author_cid, facility).await?;
        if !author_on_roster {
            tracing::warn!(
                "key {} tried to delete news post {} in another facility",
                auth.key_id,
                news.id,
            );
            return Err(AppError::InsufficientPermissions);
        }
    }
    // delete
    if !auth.testing {
        queries::delete_news_post(&state.cobalt_db, id).await?;
        tracing::info!(
            "key {} used to delete news post {}, title was '{}'",
            auth.key_id,
            id,
            news.title,
        );
    } else {
        tracing::debug!("testing key {} called on news delete endpoint", auth.key_id);
    }
    Ok(StatusCode::NO_CONTENT)
}
