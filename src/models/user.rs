use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: u64,
    pub tenant_id: u64,
    pub username: String,
    pub password: String,
    pub email: String,
    pub realname: String,
    pub avatar: Option<String>,
    pub status: i8,
    pub last_login: Option<NaiveDateTime>,
    pub crt_by: String,
    pub crt_at: NaiveDateTime,
    pub upt_by: Option<String>,
    pub upt_at: NaiveDateTime,
}
