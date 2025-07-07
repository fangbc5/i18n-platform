use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageResponse {
    pub code: String,
    pub name: String,
    pub native_name: String,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct LanguageRequest {
    pub code: String,
    pub name: String,
    pub native_name: String,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateLanguageRequest {
    pub name: Option<String>,
    pub native_name: Option<String>,
    pub is_active: Option<bool>,
}
