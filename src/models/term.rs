use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::schema::i18n_terms;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = i18n_terms)]
pub struct Term {
    pub id: String,
    pub project_id: String,
    pub source_term: String,
    pub target_term: String,
    pub language: String,
    pub description: Option<String>,
    pub platforms: Value,
    pub crt_by: String,
    pub crt_at: NaiveDateTime,
    pub upt_by: Option<String>,
    pub upt_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateTerm {
    pub project_id: String,
    pub source_term: String,
    pub target_term: String,
    pub language: String,
    pub description: Option<String>,
    pub platforms: Value,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = i18n_terms)]
pub struct UpdateTerm {
    pub source_term: Option<String>,
    pub target_term: Option<String>,
    pub language: Option<String>,
    pub description: Option<String>,
    pub platforms: Option<Value>,
}
