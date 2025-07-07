use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{i18n_translation_history, i18n_translations};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = i18n_translations)]
pub struct Translation {
    pub id: String,
    pub phrase_id: String,
    pub language: String,
    pub content: String,
    pub status: String,
    pub translated_by: Option<String>,
    pub reviewed_by: Option<String>,
    pub crt_by: String,
    pub crt_at: NaiveDateTime,
    pub upt_by: Option<String>,
    pub upt_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateTranslation {
    pub phrase_id: String,
    pub language: String,
    pub content: String,
    pub status: Option<String>,
    pub translated_by: Option<String>,
    pub reviewed_by: Option<String>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = i18n_translations)]
pub struct UpdateTranslation {
    pub content: Option<String>,
    pub status: Option<String>,
    pub translated_by: Option<String>,
    pub reviewed_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = i18n_translation_history)]
pub struct TranslationHistory {
    pub id: String,
    pub translation_id: String,
    pub content: String,
    pub version: i32,
    pub modified_by: String,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateTranslationHistory {
    pub translation_id: String,
    pub content: String,
    pub version: i32,
}
