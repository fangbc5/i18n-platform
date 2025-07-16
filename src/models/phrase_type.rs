use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::dtos::phrase_type::{CreatePhraseTypeDto, UpdatePhraseTypeDto};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PhraseType {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub crt_by: String,
    pub crt_at: NaiveDateTime,
    pub upt_by: Option<String>,
    pub upt_at: NaiveDateTime,
}

impl From<&CreatePhraseTypeDto> for PhraseType {
    fn from(dto: &CreatePhraseTypeDto) -> Self {
        PhraseType {
            id: 0,
            name: dto.name.clone(),
            description: dto.description.clone(),
            icon: dto.icon.clone(),
            crt_by: "".to_string(),
            crt_at: Local::now().naive_local(),
            upt_by: None,
            upt_at: Local::now().naive_local(),
        }
    }
}

impl From<&UpdatePhraseTypeDto> for PhraseType {
    fn from(dto: &UpdatePhraseTypeDto) -> Self {
        PhraseType {
            id: 0,
            name: dto.name.clone().unwrap_or_default(),
            description: dto.description.clone(),
            icon: dto.icon.clone(),
            crt_by: "".to_string(),
            crt_at: Local::now().naive_local(),
            upt_by: None,
            upt_at: Local::now().naive_local(),
        }
    }
}