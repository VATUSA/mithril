//! Middleware.

#![allow(clippy::unused_async_trait_impl)]

use crate::shared::{AppError, AppState, Auth};
use axum::{
    extract::{FromRequestParts, Request, State},
    middleware::Next,
    response::Response,
};
use http::header::HeaderName;
use std::sync::Arc;

const X_API_KEY: HeaderName = HeaderName::from_static("x-api-key");

/// Axum middleware to resolve the optional `X-API-Key` header.
pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // attempt to resolve the API key if provided
    let auth = match request.headers().get(&X_API_KEY) {
        None => Auth::Anonymous,
        Some(header) => {
            // If a header is provided, it must be valid. Invalid headers
            // will result in an error, even if the endpoint does not
            // require any authorization. This is to enforce good API
            // client behavior.
            let key = header
                .to_str()
                .map_err(|_| AppError::BadRequest("invalid X-API-Key header"))?;

            let api_key = crate::queries::get_api_key(&state.cobalt_db, key).await?;

            match api_key {
                Some(api_key) => Auth::Key {
                    key_id: api_key.id,
                    facility: api_key.facility,
                    testing: api_key.testing,
                },
                None => return Err(AppError::BadRequest("invalid X-API-Key header")),
            }
        }
    };

    // insert into the request
    request.extensions_mut().insert(auth);

    // continue in the call chain
    Ok(next.run(request).await)
}

/// Get the request's authentication context.
///
/// This extractor is suitable for endpoints that may be called
/// anonymously but still want to inspect whether a valid API key
/// was supplied, like for controlling behavior of returned data.
///
/// This does not need to be included if no key-specific behavior
/// needs to be implemented on a route; the base-level
/// authentication checks already happen in the middleware.
///
/// # Example
///
/// ```rust,ignore
/// use crate::{
///     db::NewsPost,
///     middleware::AuthExtractor,
///     shared::{AppError, AppState, Auth},
/// };
/// use axum::{Json, extract::State};
/// use std::sync::Arc;
///
/// async fn get_news(
///     AuthExtractor(auth): AuthExtractor,
///     State(state): State<Arc<AppState>>,
/// ) -> Result<Json<Vec<NewsPost>>, AppError> {
///     todo!()
/// }
/// ```
#[allow(unused)]
pub struct AuthExtractor(pub Auth);

impl<S> FromRequestParts<S> for AuthExtractor
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let auth = parts
            .extensions
            .get::<Auth>()
            .cloned()
            .unwrap_or(Auth::Anonymous);
        Ok(AuthExtractor(auth))
    }
}

/// Require a valid API key for protected routes.
///
/// This extractor rejects anonymous requests before the handler runs,
/// making it appropriate for endpoints that must always be called
/// with a valid `X-API-Key` header.
///
/// # Example
///
/// ```rust,ignore
/// use crate::{
///     middleware::RequireAuth,
///     shared::{AppError, AppState},
/// };
/// use axum::{Json, extract::State};
/// use serde_json::json;
/// use std::sync::Arc;
///
/// async fn create_event(
///     auth: RequireAuth,
///     State(state): State<Arc<AppState>>,
/// ) -> Result<Json<serde_json::Value>, AppError> {
///    todo!()
/// }
/// ```
pub struct RequireAuth {
    pub key_id: i64,
    pub facility: Option<String>,
    pub testing: bool,
}

impl<S> FromRequestParts<S> for RequireAuth
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        match parts
            .extensions
            .get::<Auth>()
            .cloned()
            .unwrap_or(Auth::Anonymous)
        {
            Auth::Anonymous => Err(AppError::ApiKeyRequired),
            Auth::Key {
                key_id,
                facility,
                testing,
            } => Ok(Self {
                key_id,
                facility,
                testing,
            }),
        }
    }
}
