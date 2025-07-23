use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::project::Project;

#[derive(Debug, Deserialize)]
pub struct CreateProjectDto {
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub base_language: String,
    pub crt_by: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProjectDto {
    pub name: Option<String>,
    pub code: Option<String>,
    pub description: Option<String>,
    pub base_language: Option<String>,
    pub owner_id: Option<u64>,
    pub status: Option<bool>,
    pub upt_by: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProjectVo {
    pub id: u64,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub base_language: String,
    pub owner: String,
    pub status: bool,
    pub crt_at: DateTime<Utc>,
    pub upt_at: DateTime<Utc>,
}

impl From<&Project> for ProjectVo {
    fn from(project: &Project) -> Self {
        Self {
            id: project.id,
            name: project.name.clone(),
            code: project.code.clone(),
            description: project.description.clone(),
            base_language: project.base_language.clone(),
            owner: String::new(),
            status: project.status,
            crt_at: project.crt_at,
            upt_at: project.upt_at,
        }
    }
}
