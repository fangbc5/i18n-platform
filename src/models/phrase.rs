use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

use crate::dtos::phrase::{CreatePhraseDto, UpdatePhraseDto};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Phrase {
    pub id: u64,
    pub project_id: u64,
    pub module_id: Option<u64>,
    pub type_id: u64,
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

impl From<&CreatePhraseDto> for Phrase {
    fn from(dto: &CreatePhraseDto) -> Self {
        Phrase {
            id: 0,
            project_id: dto.project_id,
            module_id: None,
            type_id: 1,
            key: dto.key.clone(),
            base_content: dto.source_text.clone(),
            context: dto.context.clone(),
            variables: None,
            platforms: Value::Null,
            tags: None,
            max_length: None,
            is_plural: false,
            crt_by: "".to_string(),
            crt_at: Local::now().naive_local(),
            upt_by: None,
            upt_at: Local::now().naive_local(),
        }
    }
}

impl From<&UpdatePhraseDto> for Phrase {
    fn from(dto: &UpdatePhraseDto) -> Self {
        Phrase {
            id: 0,
            project_id: 0,
            module_id: None,
            type_id: 0,
            key: dto.key.clone().unwrap_or_default(),
            base_content: dto.source_text.clone().unwrap_or_default(),
            context: dto.context.clone(),
            variables: None,
            platforms: Value::Null,
            tags: None,
            max_length: None,
            is_plural: false,
            crt_by: "".to_string(),
            crt_at: Local::now().naive_local(),
            upt_by: None,
            upt_at: Local::now().naive_local(),
        }
    }
}