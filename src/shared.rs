//! Shared data and utilities.

use axum::response::{IntoResponse, Response};
use http::StatusCode;
use sqlx::MySqlPool;

/// Application state available to route functions.
#[allow(unused)]
pub struct AppState {
    /// Connection to the "old" VATUSA DB
    pub vatusa_db: MySqlPool,
    /// Connection to the new VATUSA backend DB
    pub cobalt_db: MySqlPool,
}

/// App errors, broken down into enum variants for the various
/// things that can go wrong.
#[allow(unused)]
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Database(#[from] sqlx::Error),
    #[error(transparent)]
    JsonProcessingError(#[from] serde_json::Error),
    #[error(transparent)]
    EnvVarError(#[from] std::env::VarError),
    #[error("generic error {0}: {1}")]
    GenericFallback(&'static str, anyhow::Error),
}

impl IntoResponse for AppError {
    /// Generate an Axum-compatible response from an error.
    fn into_response(self) -> Response {
        // TODO improve
        (StatusCode::INTERNAL_SERVER_ERROR, "TODO").into_response()
    }
}
