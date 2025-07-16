use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::{dtos::translation::{CreateTranslationDto, UpdateTranslationDto}, models::enums::TranslationStatus};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Translation {
    pub id: u64,
    pub phrase_id: u64,
    pub language: String,
    pub content: String,
    pub status: TranslationStatus,
    pub translated_by: Option<u64>,
    pub reviewed_by: Option<u64>,
    pub crt_by: String,
    pub crt_at: NaiveDateTime,
    pub upt_by: Option<String>,
    pub upt_at: NaiveDateTime,
}

impl From<&CreateTranslationDto> for Translation {
    fn from(dto: &CreateTranslationDto) -> Self {
        Translation {
            id: 0,
            phrase_id: 0,
            language: dto.language_code.clone(),
            content: dto.translated_text.clone(),
            status: TranslationStatus::Pending,
            translated_by: None,
            reviewed_by: None,
            crt_by: "".to_string(),
            crt_at: Local::now().naive_local(),
            upt_by: None,
            upt_at: Local::now().naive_local(),
        }
    }
}

impl From<&UpdateTranslationDto> for Translation {
    fn from(dto: &UpdateTranslationDto) -> Self {
        Translation {
            id: 0,
            phrase_id: 0,
            language: dto.language_code.clone().unwrap_or_default(),
            content: dto.content.clone().unwrap_or_default(),
            status: dto.status.clone().unwrap_or(TranslationStatus::Pending),
            translated_by: dto.translated_by,
            reviewed_by: dto.reviewed_by,
            crt_by: "".to_string(),
            crt_at: Local::now().naive_local(),
            upt_by: None,
            upt_at: Local::now().naive_local(),
        }
    }
}
