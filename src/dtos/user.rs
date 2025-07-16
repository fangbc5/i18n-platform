use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::models::user::User;

#[derive(Debug, Deserialize)]
pub struct CreateUserDto {
    pub username: String,
    pub password: String,
    pub email: String,
    pub realname: String,
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
    pub avatar: Option<String>,
    pub status: Option<i8>,
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
    pub username: String,
    pub email: String,
    pub realname: String,
    pub avatar: Option<String>,
    pub status: i8,
    pub last_login: Option<NaiveDateTime>,
    pub crt_at: NaiveDateTime,
    pub upt_at: NaiveDateTime,
}

impl From<User> for UserVo {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            realname: user.realname,
            avatar: user.avatar,
            status: user.status,
            last_login: user.last_login,
            crt_at: user.crt_at,
            upt_at: user.upt_at,
        }
    }
}
