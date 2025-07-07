use crate::{
    errors::AppError,
    utils::jwt::{decode_token, Claims},
    AppState,
};
use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use casbin::{CoreApi, RbacApi};
use redis::Commands;
use std::sync::Arc;

// 不需要认证的路径
const PUBLIC_PATHS: [&str; 3] = ["/api/auth/login", "/api/auth/register", "/health"];

pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    // 跳过不需要认证的路径
    let path = req.uri().path();
    if path == "/health" || path.starts_with("/api/auth") {
        return Ok(next.run(req).await);
    }

    // 获取并验证Token
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|auth| auth.to_str().ok())
        .and_then(|auth| auth.strip_prefix("Bearer "))
        .ok_or(AppError::Unauthorized("Missing token".into()))?;

    let claims = decode_token(token)?;

    // 检查Token是否被加入黑名单
    let redis_key = format!("blacklist:{}", token);
    let is_blacklisted: bool = state
        .redis
        .get_connection()
        .map_err(AppError::Cache)?
        .exists(&redis_key)
        .map_err(AppError::Cache)?;

    if is_blacklisted {
        return Err(AppError::Unauthorized("Token is blacklisted".into()));
    }

    // 检查权限
    let can_access = state
        .casbin
        .enforce(&[&claims.sub, path, req.method().as_str()])
        .map_err(|e| AppError::Authorization(e.to_string()))?;

    if !can_access {
        return Err(AppError::Forbidden("Access denied".into()));
    }

    // 将用户信息添加到请求中
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}

// 用于测试的跳过认证中间件
#[cfg(test)]
pub async fn skip_auth<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    Ok(next.run(req).await)
}
