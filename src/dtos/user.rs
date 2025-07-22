use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::user::User;

#[derive(Debug, Deserialize)]
pub struct CreateUserDto {
    pub username: Option<String>,
    pub password: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub nickname: Option<String>,
    pub status: Option<bool>,
    pub crt_by: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub password: String,
    pub nickname: Option<String>,
    pub status: Option<bool>,
    pub verify_code: Option<String>,
}

impl From<&RegisterRequest> for CreateUserDto {
    fn from(req: &RegisterRequest) -> Self {
        Self {
            username: req.username.clone(),
            password: req.password.clone(),
            email: req.email.clone(),
            phone: req.phone.clone(),
            nickname: req.nickname.clone(),
            status: req.status.clone(),
            crt_by: Some("register".to_owned())
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LoginDto {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserDto {
    pub username: Option<String>,
    pub email: Option<String>,
    pub realname: Option<String>,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub status: Option<bool>,
    pub upt_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64, // 过期时间（秒）
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i32,
}

#[derive(Debug, Serialize)]
pub struct UserVo {
    pub id: u64,
    pub username: Option<String>,
    pub email: Option<String>,
    pub realname: Option<String>,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub status: bool,
    pub last_login: Option<NaiveDateTime>,
    pub crt_at: DateTime<Utc>,
    pub upt_at: DateTime<Utc>,
}

impl From<&User> for UserVo {
    fn from(user: &User) -> Self {
        Self {
            id: user.id,
            username: user.username.clone(),
            email: user.email.clone(),
            realname: user.realname.clone(),
            nickname: user.nickname.clone(),
            avatar: user.avatar.clone(),
            status: user.status,
            last_login: user.last_login,
            crt_at: user.crt_at,
            upt_at: user.upt_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageUserRequest {
    #[serde(default = "crate::utils::default_page")]
    pub page: u32,
    #[serde(default = "crate::utils::default_size")]
    pub size: u32,
    // 查询key支持用户名、手机号、邮箱、昵称、真实姓名搜索
    pub search_key: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserIdsRequest {
    pub ids: Vec<u64>,
}