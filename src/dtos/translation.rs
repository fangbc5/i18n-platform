use crate::models::enums::TranslationStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateTranslationDto {
    pub phrase_id: i32,
    pub language_code: String,
    pub translated_text: String,
}

#[derive(Debug, Serialize)]
pub struct TranslationDto {
    pub id: i32,
    pub phrase_id: i32,
    pub language_code: String,
    pub translated_text: String,
    pub translator_id: i32,
    pub status: TranslationStatus,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTranslationDto {
    pub translated_text: Option<String>,
    pub status: Option<TranslationStatus>,
}
