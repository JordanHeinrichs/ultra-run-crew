use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error occurred")]
    Database(#[from] sqlx::Error),

    #[error("Authentication failed: {0}")]
    Auth(String),

    #[error("Internal server error")]
    Internal,
}

#[derive(Serialize)]
pub struct ApiErrorResponse {
    pub error: String,
}

// This trait implementation tells Axum how to convert your error into HTML/JSON
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Log the actual error internally for terminal debugging
        tracing::error!("Application Error: {:?}", self);

        let (status, error_message) = match self {
            AppError::Database(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "A database error occurred.".to_string(),
            ),
            AppError::Auth(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong internally.".to_string(),
            ),
        };

        // 2. Wrap in axum::Json and return with the HTTP status code
        (
            status,
            Json(ApiErrorResponse {
                error: error_message,
            }),
        )
            .into_response()
    }
}
