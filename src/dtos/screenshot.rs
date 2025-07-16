use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateScreenshotDto {
    pub phrase_id: u64,
    pub image_url: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateScreenshotDto {
    pub image_url: Option<String>,
    pub description: Option<String>,
}
