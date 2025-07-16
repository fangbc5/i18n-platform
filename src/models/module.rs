use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::dtos::module::{CreateModuleDto, UpdateModuleDto};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Module {
    pub id: u64,
    pub project_id: u64,
    pub name: String,
    pub description: Option<String>,
    pub path: Option<String>,
    pub crt_by: String,
    pub crt_at: NaiveDateTime,
    pub upt_by: Option<String>,
    pub upt_at: NaiveDateTime,
}

impl From<&CreateModuleDto> for Module {
    fn from(dto: &CreateModuleDto) -> Self {
        Module {
            id: 0,
            project_id: dto.project_id,
            name: dto.name.clone(),
            description: dto.description.clone(),
            path: None,
            crt_by: "".to_string(),
            crt_at: Local::now().naive_local(),
            upt_by: None,
            upt_at: Local::now().naive_local(),
        }
    }
}

impl From<&UpdateModuleDto> for Module {
    fn from(dto: &UpdateModuleDto) -> Self {
        Module {
            id: 0,
            project_id: 0,
            name: dto.name.clone().unwrap_or_default(),
            description: dto.description.clone(),
            path: None,
            crt_by: "".to_string(),
            crt_at: Local::now().naive_local(),
            upt_by: None,
            upt_at: Local::now().naive_local(),
        }
    }
}
