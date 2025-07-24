use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use application::errors::AppError;
use log::error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Authentication error: {0}")]
    AuthenticationError(String),
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let (status, error_type, message) = match self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, "not_found", msg.clone()),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "bad_request", msg.clone()),
            ApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, "unauthorized", msg.clone()),
            ApiError::AuthenticationError(msg) => (
                StatusCode::UNAUTHORIZED,
                "authentication_error",
                msg.clone(),
            ),
            ApiError::InternalServerError(msg) => {
                error!("Internal server error: {msg}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_server_error",
                    "internal server error".to_string(),
                )
            }
            ApiError::DatabaseError(msg) => {
                error!("Database error: {msg}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_server_error",
                    "internal server error".to_string(),
                )
            }
        };

        HttpResponse::build(status).json(serde_json::json!({
            "error": {
                "type": error_type,
                "message": message,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            }
        }))
    }
}

impl From<AppError> for ApiError {
    fn from(value: AppError) -> Self {
        match value {
            AppError::NotFound(msg) => ApiError::NotFound(msg),
            AppError::BadRequest(msg) => ApiError::BadRequest(msg),
            AppError::AlreadyExists(msg) => ApiError::BadRequest(msg),
            AppError::DatabaseError(msg) => ApiError::DatabaseError(msg),
        }
    }
}
