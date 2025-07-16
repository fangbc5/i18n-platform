use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::constants::{USER_EMAIL_REGEX, USER_NAME_MAX_LENGTH, USER_NAME_MIN_LENGTH, USER_PASSWORD_MAX_LENGTH, USER_PASSWORD_MIN_LENGTH};
use crate::errors::AppError;
use crate::middleware::auth::Authentication;
use crate::services::user_service::UserService;
use crate::utils::{jwt, password};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .wrap(Authentication::default())
            .service(login)
            .service(register),
    );
}

#[post("/login")]
pub async fn login(
    user_service: web::Data<UserService>,
    login_req: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    // 密码加密
    let password = password::hash_password(&login_req.password)?;
    let user = user_service
        .verify_password(&login_req.username, &password)
        .await?;

    let token = jwt::create_token(user.id, user.tenant_id)?;
    Ok(HttpResponse::Ok().json(LoginResponse { token }))
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[post("/register")]
pub async fn register(
    user_service: web::Data<UserService>,
    register_req: web::Json<RegisterRequest>,
) -> Result<HttpResponse, AppError> {
    // 检查用户名是否符合要求, 长度至少为5
    if register_req.username.len() < USER_NAME_MIN_LENGTH {
        return Err(AppError::BadRequest("Username must be at least 5 characters long".into()));
    }

    // 检查用户名是否符合要求, 长度不超过32
    if register_req.username.len() > USER_NAME_MAX_LENGTH {
        return Err(AppError::BadRequest("Username must be less than 32 characters long".into()));
    }

    // 检查用户名是否符合要求, 只能包含字母、数字、下划线
    if !register_req.username.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(AppError::BadRequest("Username can only contain letters, numbers, and underscores".into()));
    }
    // 检查用户名是否已存在
    if user_service.find_by_username(&register_req.username).await?.is_some() {
        return Err(AppError::BadRequest("Username already exists".into()));
    }

    // 检查邮箱是否符合要求, 使用正则表达式
    if !regex::Regex::new(USER_EMAIL_REGEX)
        .unwrap()
        .is_match(&register_req.email)
    {
        return Err(AppError::BadRequest("Invalid email".into()));
    }

    // 检查邮箱是否已存在
    if user_service.find_by_email(&register_req.email).await?.is_some() {
        return Err(AppError::BadRequest("Email already exists".into()));
    }
    // 检查密码是否符合要求, 长度至少为8
    if register_req.password.len() < USER_PASSWORD_MIN_LENGTH {
        return Err(AppError::BadRequest("Password must be at least 8 characters long".into()));
    }

    // 检查密码是否符合要求, 长度不超过32
    if register_req.password.len() > USER_PASSWORD_MAX_LENGTH{
        return Err(AppError::BadRequest("Password must be less than 32 characters long".into()));
    }

    // 密码加密
    let password = password::hash_password(&register_req.password)?;
    let user = user_service
        .create_user(
            &register_req.username,
            &password,
            &register_req.email,
        )
        .await?;

    Ok(HttpResponse::Created().json(user))
}
