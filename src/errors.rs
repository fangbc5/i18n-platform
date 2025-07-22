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
    Base64Error(String),
    BusinessError(String),
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
            AppError::Base64Error(msg) => msg,
            AppError::BusinessError(msg) => msg,
        };
        write!(f, "{}", message)
    }
}

#[derive(Serialize)]
struct ErrorResponse<'a> {
    pub code: i32,
    pub message: &'a str,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (status, message) = match self {
            AppError::Auth(msg) => (StatusCode::OK, msg),
            AppError::Forbidden(msg) => (StatusCode::OK, msg),
            AppError::BadRequest(msg) => (StatusCode::OK, msg),
            AppError::NotFound(msg) => (StatusCode::OK, msg),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::Internal(msg) => (StatusCode::OK, msg),
            AppError::SerdeError(msg) => (StatusCode::OK, msg),
            AppError::Queue(msg) => (StatusCode::OK, msg),
            AppError::Storage(msg) => (StatusCode::OK, msg),
            AppError::Permission(msg) => (StatusCode::OK, msg),
            AppError::Database(msg) => (StatusCode::OK, msg),
            AppError::CasbinError(msg) => (StatusCode::OK, msg),
            AppError::Base64Error(msg) => (StatusCode::OK, msg),
            AppError::BusinessError(msg) => (StatusCode::OK, msg),
        };

        HttpResponse::build(status).json(ErrorResponse {
            code: 500,
            message,
        })
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

impl From<base64::DecodeError> for AppError {
    fn from(err: base64::DecodeError) -> Self {
        AppError::Base64Error(format!("Base64 error: {}", err))
    }
}

impl<E> From<SdkError<E>> for AppError {
    fn from(err: SdkError<E>) -> Self {
        AppError::Storage(err.to_string())
    }
}
