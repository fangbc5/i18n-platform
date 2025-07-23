use chrono::{DateTime, Local, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::dtos::project::{CreateProjectDto, UpdateProjectDto};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub id: u64,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub base_language: String,
    pub owner_id: u64,
    pub status: bool,
    pub crt_by: String,
    pub crt_at: DateTime<Utc>,
    pub upt_by: Option<String>,
    pub upt_at: DateTime<Utc>,
}

impl From<&CreateProjectDto> for Project {
    fn from(dto: &CreateProjectDto) -> Self {
        Project {
            id: 1,
            name: dto.name.clone(),
            code: dto.code.clone(),
            description: dto.description.clone(),
            base_language: dto.base_language.clone(),
            owner_id: 0,
            status: true,
            crt_by: dto.crt_by.clone().unwrap_or("admin".to_owned()),
            crt_at: Utc::now(),
            upt_by: None,
            upt_at: Utc::now(),
        }
    }
}

impl From<&UpdateProjectDto> for Project {
    fn from(dto: &UpdateProjectDto) -> Self {
        Project {
            id: 1,
            name: dto.name.clone().unwrap_or_default(),
            code: dto.code.clone().unwrap_or_default(),
            description: dto.description.clone(),
            base_language: dto.base_language.clone().unwrap_or("zh_CN".to_owned()),
            owner_id: dto.owner_id.unwrap_or(0),
            status: dto.status.unwrap_or(true),
            crt_by: "".to_string(),
            crt_at: Utc::now(),
            upt_by: dto.upt_by.clone(),
            upt_at: Utc::now(),
        }
    }
}