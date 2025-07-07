use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateProjectDto {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProjectDto {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProjectDto {
    pub name: Option<String>,
    pub description: Option<String>,
}
