use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::models::project::Project;

#[derive(Debug, Deserialize)]
pub struct CreateProjectDto {
    pub name: String,
    pub code: String,
    pub base_language: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProjectDto {
    pub name: Option<String>,
    pub code: Option<String>,
    pub description: Option<String>,
    pub status: Option<i8>,
}

#[derive(Debug, Serialize)]
pub struct ProjectVo {
    pub id: u64,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub base_language: String,
    pub owner_id: u64,
    pub status: i8,
    pub crt_at: NaiveDateTime,
    pub upt_at: NaiveDateTime,
}

impl From<Project> for ProjectVo {
    fn from(project: Project) -> Self {
        Self {
            id: project.id,
            name: project.name,
            code: project.code,
            description: project.description,
            base_language: project.base_language,
            owner_id: project.owner_id,
            status: project.status,
            crt_at: project.crt_at,
            upt_at: project.upt_at,
        }
    }
}
