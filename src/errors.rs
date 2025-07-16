use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use aws_sdk_s3::error::SdkError;
use serde::Serialize;
use std::fmt;
use validator::ValidationErrors;

#[derive(Debug)]
pub enum AppError {
    Auth(String),
    Forbidden(String),
    BadRequest(String),
    NotFound(String),
    Unauthorized(String),
    Internal(String),
    SerdeError(String),
    Queue(String),
    Storage(String),
    Permission(String),
    Database(String),
    CasbinError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            AppError::Auth(msg) => msg,
            AppError::Forbidden(msg) => msg,
            AppError::BadRequest(msg) => msg,
            AppError::NotFound(msg) => msg,
            AppError::Unauthorized(msg) => msg,
            AppError::Internal(msg) => msg,
            AppError::SerdeError(msg) => msg,
            AppError::Queue(msg) => msg,
            AppError::Storage(msg) => msg,
            AppError::Permission(msg) => msg,
            AppError::Database(msg) => msg,
            AppError::CasbinError(msg) => msg,
        };
        write!(f, "{}", message)
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (status, message) = match self {
            AppError::Auth(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::SerdeError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::Queue(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::Storage(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::Permission(msg) => (StatusCode::FORBIDDEN, msg),
            AppError::Database(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::CasbinError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        HttpResponse::build(status).json(ErrorResponse {
            message: message.to_string(),
        })
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(err: bcrypt::BcryptError) -> Self {
        AppError::Internal(format!("Bcrypt error: {}", err))
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        AppError::Unauthorized(format!("JWT error: {}", err))
    }
}

impl From<ValidationErrors> for AppError {
    fn from(err: ValidationErrors) -> Self {
        AppError::BadRequest(format!("Validation error: {}", err))
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound("Record not found".to_string()),
            _ => AppError::Database(format!("Database error: {}", err)),
        }
    }
}

impl<E> From<SdkError<E>> for AppError {
    fn from(err: SdkError<E>) -> Self {
        AppError::Storage(err.to_string())
    }
}
