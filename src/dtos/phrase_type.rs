use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PhraseTypeResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PhraseTypeRequest {
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePhraseTypeRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
}
