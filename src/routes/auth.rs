use crate::AppState;
use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub fn auth_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/logout", post(logout))
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct LoginResponse {
    token: String,
}

async fn login(
    State(_state): State<Arc<AppState>>,
    Json(_payload): Json<LoginRequest>,
) -> Json<LoginResponse> {
    // 临时返回，后续实现
    Json(LoginResponse {
        token: "temporary_token".to_string(),
    })
}

async fn register(
    State(_state): State<Arc<AppState>>,
    Json(_payload): Json<LoginRequest>,
) -> Json<LoginResponse> {
    // 临时返回，后续实现
    Json(LoginResponse {
        token: "temporary_token".to_string(),
    })
}

async fn logout() -> &'static str {
    "Logged out"
}
