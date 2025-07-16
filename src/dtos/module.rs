use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateModuleDto {
    pub project_id: u64,
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateModuleDto {
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ModuleDto {
    pub id: u64,
    pub project_id: u64,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ModuleQuery {
    pub project_id: Option<u64>,
    pub name: Option<String>,
    pub page: u32,
    pub size: u32,
}
