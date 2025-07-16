use crate::models::enums::TranslationStatus;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateTranslationDto {
    pub language_code: String,
    pub translated_text: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTranslationDto {
    pub language_code: Option<String>,
    pub content: Option<String>,
    pub status: Option<TranslationStatus>,
    pub translated_by: Option<u64>,
    pub reviewed_by: Option<u64>,
}
