use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ScreenshotResponse {
    pub id: String,
    pub phrase_id: String,
    pub image_url: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ScreenshotRequest {
    pub phrase_id: String,
    pub image_url: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateScreenshotRequest {
    pub image_url: Option<String>,
    pub description: Option<String>,
}
