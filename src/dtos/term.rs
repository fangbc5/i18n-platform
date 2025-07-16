use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateTermDto {
    pub project_id: u64,
    pub source_term: String,
    pub target_term: String,
    pub language_code: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TermDto {
    pub id: u64,
    pub project_id: u64,
    pub source_term: String,
    pub target_term: String,
    pub language_code: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTermDto {
    pub source_term: Option<String>,
    pub target_term: Option<String>,
    pub description: Option<String>,
}
