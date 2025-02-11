use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Authentication failed")]
    AuthenticationError,
    #[error("Not found")]
    NotFound,
    #[error("Database error")]
    DatabaseError(#[from] diesel::result::Error),
    #[error("Internal server error")]
    InternalServerError,
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Invalid email format")]
    InvalidEmail,
    #[error("Invalid password format")]
    InvalidPassword,
    #[error("Email not verified")]
    EmailNotVerified,
    #[error("Insufficient permissions")]
    InsufficientPermissions,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::AuthenticationError => (StatusCode::UNAUTHORIZED, "Authentication failed"),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource not found"),
            AppError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            AppError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
            AppError::RateLimitExceeded => (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded"),
            AppError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AppError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials"),
            AppError::InvalidEmail => (StatusCode::BAD_REQUEST, "Invalid email format"),
            AppError::InvalidPassword => (StatusCode::BAD_REQUEST, "Password does not meet requirements"),
            AppError::EmailNotVerified => (StatusCode::FORBIDDEN, "Email not verified"),
            AppError::InsufficientPermissions => (StatusCode::FORBIDDEN, "Insufficient permissions"),
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
} 