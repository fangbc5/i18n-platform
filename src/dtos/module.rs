use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleResponse {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub description: Option<String>,
    pub path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ModuleRequest {
    pub project_id: String,
    pub name: String,
    pub description: Option<String>,
    pub path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateModuleRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub path: Option<String>,
}
