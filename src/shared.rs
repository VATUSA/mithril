//! Shared data and utilities.

use axum::{
    Json,
    response::{IntoResponse, Response},
};
use http::StatusCode;
use serde::Serialize;
use sqlx::MySqlPool;

/// Application state available to route functions.
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
    #[error("database error")]
    Database(#[from] sqlx::Error),

    #[error("API key required")]
    ApiKeyRequired,

    #[error("insufficient permissions")]
    InsufficientPermissions,

    #[error("invalid JSON")]
    JsonProcessingError(#[from] serde_json::Error),

    #[error("missing or invalid environment variable")]
    EnvVarError(#[from] std::env::VarError),

    #[error("route not found")]
    RouteNotFound,

    #[error("{0}")]
    BadRequest(&'static str),

    #[error("{0}")]
    NotFound(&'static str),

    #[error("{0}")]
    Internal(&'static str),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: ErrorBody,
}

#[derive(Serialize)]
struct ErrorBody {
    code: &'static str,
    message: String,
}

impl AppError {
    /// Generate the HTTP status code for the error.
    fn status_code(&self) -> StatusCode {
        match self {
            Self::ApiKeyRequired => StatusCode::UNAUTHORIZED,
            Self::InsufficientPermissions => StatusCode::FORBIDDEN,
            Self::BadRequest(_) | Self::JsonProcessingError(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) | Self::RouteNotFound => StatusCode::NOT_FOUND,
            Self::Database(_) | Self::EnvVarError(_) | Self::Internal(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    /// Generate the "code" string in the response body.
    fn code(&self) -> &'static str {
        match self {
            Self::ApiKeyRequired => "api_key_required",
            Self::InsufficientPermissions => "insufficient_permissions",
            Self::BadRequest(_) => "bad_request",
            Self::JsonProcessingError(_) => "invalid_json",
            Self::NotFound(_) | Self::RouteNotFound => "not_found",
            Self::Database(_) => "database_error",
            Self::EnvVarError(_) => "config_error",
            Self::Internal(_) => "internal_error",
        }
    }

    /// Generate the "message" string in the response body.
    fn client_message(&self) -> String {
        match self {
            Self::Database(_) => "A database error occurred".to_string(),
            Self::EnvVarError(_) => "Server configuration error".to_string(),
            _ => self.to_string(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        if status.is_server_error() {
            tracing::error!(error = ?self, "request failed");
        } else {
            tracing::warn!(error = ?self, "request failed");
        }
        let body = ErrorResponse {
            error: ErrorBody {
                code: self.code(),
                message: self.client_message(),
            },
        };
        (status, Json(body)).into_response()
    }
}

/// Authentication variants for incoming requests.
#[derive(Debug, Clone)]
pub enum Auth {
    /// No API key was supplied.
    Anonymous,
    /// A valid authenticated request
    Key {
        key_id: i64,
        /// Which facility, if any, the key allows for additional permissions.
        facility: Option<String>,
        testing: bool,
    },
}
