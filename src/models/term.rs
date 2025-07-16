use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

use crate::dtos::term::{CreateTermDto, UpdateTermDto};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Term {
    pub id: u64,
    pub project_id: u64,
    pub source_term: String,
    pub target_term: String,
    pub language: String,
    pub description: Option<String>,
    pub platforms: Value,
    pub crt_by: String,
    pub crt_at: NaiveDateTime,
    pub upt_by: Option<String>,
    pub upt_at: NaiveDateTime,
}

impl From<&CreateTermDto> for Term {
    fn from(dto: &CreateTermDto) -> Self {
        Term {
            id: 0,
            project_id: dto.project_id,
            source_term: dto.source_term.clone(),
            target_term: dto.target_term.clone(),
            language: dto.language_code.clone(),
            description: dto.description.clone(),
            platforms: Value::Null,
            crt_by: "".to_string(),
            crt_at: Local::now().naive_local(),
            upt_by: None,
            upt_at: Local::now().naive_local(),
        }
    }
}

impl From<&UpdateTermDto> for Term {
    fn from(dto: &UpdateTermDto) -> Self {
        Term {
            id: 0,
            project_id: 0,
            source_term: dto.source_term.clone().unwrap_or_default(),
            target_term: dto.target_term.clone().unwrap_or_default(),
            language: "".to_string(),
            description: dto.description.clone(),
            platforms: Value::Null,
            crt_by: "".to_string(),
            crt_at: Local::now().naive_local(),
            upt_by: None,
            upt_at: Local::now().naive_local(),
        }
    }
}