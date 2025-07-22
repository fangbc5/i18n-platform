use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::language::Language;

#[derive(Debug, Deserialize)]
pub struct CreateLanguageDto {
    pub name: String,
    pub code: String,
    pub is_active: Option<bool>,
    pub is_native: Option<bool>,
    pub crt_by: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateLanguageDto {
    pub name: Option<String>,
    pub code: Option<String>,
    pub is_active: Option<bool>,
    pub is_native: Option<bool>,
    pub upt_by: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LanguageVo {
    pub id: u64,
    pub code: String,
    pub name: String,
    pub is_active: bool,
    pub is_native: bool,
    pub crt_at: DateTime<Utc>,
    pub upt_at: DateTime<Utc>,
}

impl From<&Language> for LanguageVo {
    fn from(language: &Language) -> Self {
        Self {
            id: language.id,
            code: language.code.clone(),
            name: language.name.clone(),
            is_active: language.is_active,
            is_native: language.is_native,
            crt_at: language.crt_at,
            upt_at: language.upt_at,
        }
    }
}
