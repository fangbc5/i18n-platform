use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::i18n_modules;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = i18n_modules)]
pub struct Module {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub description: Option<String>,
    pub path: Option<String>,
    pub crt_by: String,
    pub crt_at: NaiveDateTime,
    pub upt_by: Option<String>,
    pub upt_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateModule {
    pub project_id: String,
    pub name: String,
    pub description: Option<String>,
    pub path: Option<String>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = i18n_modules)]
pub struct UpdateModule {
    pub name: Option<String>,
    pub description: Option<String>,
    pub path: Option<String>,
}
