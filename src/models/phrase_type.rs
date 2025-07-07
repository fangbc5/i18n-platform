use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::i18n_phrase_types;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = i18n_phrase_types)]
pub struct PhraseType {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub crt_by: String,
    pub crt_at: NaiveDateTime,
    pub upt_by: Option<String>,
    pub upt_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreatePhraseType {
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = i18n_phrase_types)]
pub struct UpdatePhraseType {
    pub name: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
}
