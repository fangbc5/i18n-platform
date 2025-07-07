use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use diesel::result::Error as DieselError;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("认证错误: {0}")]
    Unauthorized(String),

    #[error("授权错误: {0}")]
    Authorization(String),

    #[error("禁止访问: {0}")]
    Forbidden(String),

    #[error("数据库错误: {0}")]
    Database(#[from] DieselError),

    #[error("缓存错误: {0}")]
    Cache(#[from] redis::RedisError),

    #[error("Token错误: {0}")]
    Token(String),

    #[error("Casbin错误: {0}")]
    Casbin(String),

    #[error("存储错误: {0}")]
    Storage(String),

    #[error("验证错误: {0}")]
    Validation(String),

    #[error("未找到: {0}")]
    NotFound(String),

    #[error("内部错误: {0}")]
    Internal(String),

    #[error("认证错误: {0}")]
    Authentication(String),

    #[error("消息队列错误: {0}")]
    Queue(String),

    #[error("序列化错误: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("密码错误: {0}")]
    Password(String),

    #[error("JWT错误: {0}")]
    Jwt(String),

    #[error("Redis错误: {0}")]
    Redis(String),

    #[error("权限错误: {0}")]
    Permission(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::Authorization(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
            AppError::Database(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::Cache(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::Token(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::Casbin(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::Storage(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::Authentication(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::Queue(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::Serialization(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::Password(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Jwt(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::Redis(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::Permission(msg) => (StatusCode::FORBIDDEN, msg),
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
