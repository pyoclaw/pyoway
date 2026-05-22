//! Custom error types and HTTP response conversions.

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

/// Application-level errors.
///
/// Currently unused in v1; will be used for API error responses in future phases.
#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    /// A 404 Not Found error.
    #[error("Resource not found")]
    NotFound,

    /// An internal server error.
    #[error("Internal server error: {0}")]
    Internal(#[source] Box<dyn std::error::Error + Send + Sync>),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::NotFound => (StatusCode::NOT_FOUND, "Resource not found".to_string()),
            Self::Internal(ref e) => {
                tracing::error!("Internal error: {e}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

/// Shorthand for returning an internal server error response.
#[allow(dead_code)]
fn internal_error(msg: impl Into<String>) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({ "error": msg.into() })),
    )
}

/// Shorthand for returning a not found response.
#[allow(dead_code)]
fn not_found() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(json!({ "error": "Resource not found" })),
    )
}
