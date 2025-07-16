use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::models::language::Language;

#[derive(Debug, Deserialize)]
pub struct CreateLanguageDto {
    pub name: String,
    pub code: String,
    pub native_name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateLanguageDto {
    pub name: Option<String>,
    pub code: Option<String>,
    pub native_name: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct LanguageVo {
    pub code: String,
    pub name: String,
    pub native_name: String,
    pub is_active: bool,
    pub crt_at: NaiveDateTime,
    pub upt_at: NaiveDateTime,
}

impl From<Language> for LanguageVo {
    fn from(language: Language) -> Self {
        Self {
            code: language.code,
            name: language.name,
            native_name: language.native_name,
            is_active: language.is_active,
            crt_at: language.crt_at,
            upt_at: language.upt_at,
        }
    }
}
