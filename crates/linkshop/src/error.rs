use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid slug: {0}")]
    InvalidSlug(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Internal server error: {0}")]
    Internal(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::InvalidSlug(_) => StatusCode::NOT_FOUND,
            Self::Database(_) | Self::Config(_) | Self::Internal(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        let body = match self {
            Self::NotFound(msg) | Self::InvalidSlug(msg) => ErrorResponse {
                error: "not_found".to_string(),
                message: msg.clone(),
            },
            Self::Database(err) => {
                tracing::error!(error = %err, "Database error encountered during request");
                ErrorResponse {
                    error: "internal_error".to_string(),
                    message: "An internal server error occurred".to_string(),
                }
            }
            Self::Config(msg) | Self::Internal(msg) => {
                tracing::error!(error = %msg, "Internal error encountered during request");
                ErrorResponse {
                    error: "internal_error".to_string(),
                    message: "An internal server error occurred".to_string(),
                }
            }
        };

        HttpResponse::build(status).json(body)
    }
}
