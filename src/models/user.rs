use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::i18n_users;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = i18n_users)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
    pub email: String,
    pub realname: String,
    pub avatar: Option<String>,
    pub status: bool,
    pub last_login: Option<NaiveDateTime>,
    pub crt_by: Option<String>,
    pub crt_at: NaiveDateTime,
    pub upt_by: Option<String>,
    pub upt_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
    pub email: String,
    pub realname: String,
    pub avatar: Option<String>,
    pub status: Option<bool>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = i18n_users)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub realname: Option<String>,
    pub avatar: Option<String>,
    pub status: Option<bool>,
    pub last_login: Option<NaiveDateTime>,
}
