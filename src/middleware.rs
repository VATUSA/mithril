use crate::shared::{AppState, Auth};
use axum::{
    extract::{FromRequestParts, Request, State},
    middleware::Next,
    response::Response,
};
use std::{convert::Infallible, sync::Arc};

/// Axum middleware to extract headers for presence of valid API key.
pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Response {
    // attempt to match the auth header to a valid API key from the DB
    let auth = if let Some(header) = request.headers().get("authorization") {
        let as_str = match header.to_str() {
            Ok(s) => s,
            Err(e) => {
                tracing::warn!("Auth header value could not be used as str: {e}");
                return next.run(request).await;
            }
        };
        match crate::queries::get_api_key(&state.vatusa_db, as_str).await {
            Ok(Some(api_key)) => Auth::Key {
                facility: api_key.facility,
            },
            Ok(None) => {
                tracing::info!("Auth header '{as_str}' not found");
                Auth::Anonymous
            }
            Err(e) => {
                tracing::error!("Error accessing DB to check auth header: {e}");
                return next.run(request).await;
            }
        }
    } else {
        Auth::Anonymous
    };

    // insert into the request
    request.extensions_mut().insert(auth);

    // continue in the call chain
    next.run(request).await
}

/// Get the API key authentication from the request;
pub struct AuthExtractor(pub Auth);

impl<S> FromRequestParts<S> for AuthExtractor
where
    S: Send + Sync,
{
    type Rejection = Infallible;

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
