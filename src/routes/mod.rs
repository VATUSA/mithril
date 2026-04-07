//! API routes.

pub mod academy;
pub mod events;
pub mod facility;
pub mod news;
pub mod solo;
pub mod support;
pub mod tmu;
pub mod training;
pub mod user;

/// Generic route fallback.
pub async fn fallback() -> crate::shared::AppError {
    crate::shared::AppError::RouteNotFound
}
