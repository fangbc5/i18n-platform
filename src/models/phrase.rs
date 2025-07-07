use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::schema::i18n_phrases;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = i18n_phrases)]
pub struct Phrase {
    pub id: String,
    pub project_id: String,
    pub module_id: Option<String>,
    pub type_id: String,
    pub key: String,
    pub base_content: String,
    pub context: Option<String>,
    pub variables: Option<Value>,
    pub platforms: Value,
    pub tags: Option<Value>,
    pub max_length: Option<i32>,
    pub is_plural: bool,
    pub crt_by: String,
    pub crt_at: NaiveDateTime,
    pub upt_by: Option<String>,
    pub upt_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreatePhrase {
    pub project_id: String,
    pub module_id: Option<String>,
    pub type_id: String,
    pub key: String,
    pub base_content: String,
    pub context: Option<String>,
    pub variables: Option<Value>,
    pub platforms: Value,
    pub tags: Option<Value>,
    pub max_length: Option<i32>,
    pub is_plural: Option<bool>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = i18n_phrases)]
pub struct UpdatePhrase {
    pub module_id: Option<String>,
    pub type_id: Option<String>,
    pub key: Option<String>,
    pub base_content: Option<String>,
    pub context: Option<String>,
    pub variables: Option<Value>,
    pub platforms: Option<Value>,
    pub tags: Option<Value>,
    pub max_length: Option<i32>,
    pub is_plural: Option<bool>,
}
