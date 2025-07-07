use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::i18n_languages;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = i18n_languages)]
pub struct Language {
    pub code: String,
    pub name: String,
    pub native_name: String,
    pub is_active: bool,
    pub crt_by: String,
    pub crt_at: NaiveDateTime,
    pub upt_by: Option<String>,
    pub upt_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateLanguage {
    pub code: String,
    pub name: String,
    pub native_name: String,
    pub is_active: Option<bool>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = i18n_languages)]
pub struct UpdateLanguage {
    pub name: Option<String>,
    pub native_name: Option<String>,
    pub is_active: Option<bool>,
}
