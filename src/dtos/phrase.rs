use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreatePhraseDto {
    pub project_id: i32,
    pub key: String,
    pub source_text: String,
    pub context: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PhraseDto {
    pub id: i32,
    pub project_id: i32,
    pub key: String,
    pub source_text: String,
    pub context: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePhraseDto {
    pub key: Option<String>,
    pub source_text: Option<String>,
    pub context: Option<String>,
}
