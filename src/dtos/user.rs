use crate::models::enums::UserRole;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateUserDto {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: UserRole,
}

#[derive(Debug, Serialize)]
pub struct UserDto {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub role: UserRole,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserDto {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub role: Option<UserRole>,
}
