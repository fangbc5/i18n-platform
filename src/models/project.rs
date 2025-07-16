use chrono::{Local, NaiveDateTime};
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
    pub status: i8,
    pub crt_by: String,
    pub crt_at: NaiveDateTime,
    pub upt_by: Option<String>,
    pub upt_at: NaiveDateTime,
}

impl From<&CreateProjectDto> for Project {
    fn from(dto: &CreateProjectDto) -> Self {
        Project {
            id: 0,
            name: dto.name.clone(),
            code: dto.code.clone(),
            description: None,
            base_language: "en".to_string(),
            owner_id: 0,
            status: 0,
            crt_by: "".to_string(),
            crt_at: Local::now().naive_local(),
            upt_by: None,
            upt_at: Local::now().naive_local(),
        }
    }
}

impl From<&UpdateProjectDto> for Project {
    fn from(dto: &UpdateProjectDto) -> Self {
        Project {
            id: 0,
            name: dto.name.clone().unwrap_or_default(),
            code: dto.code.clone().unwrap_or_default(),
            description: dto.description.clone(),
            base_language: "en".to_string(),
            owner_id: 0,
            status: dto.status.unwrap_or(0),
            crt_by: "".to_string(),
            crt_at: Local::now().naive_local(),
            upt_by: None,
            upt_at: Local::now().naive_local(),
        }
    }
}