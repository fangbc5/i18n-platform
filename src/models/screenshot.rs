use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::dtos::screenshot::{CreateScreenshotDto, UpdateScreenshotDto};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Screenshot {
    pub id: u64,
    pub phrase_id: u64,
    pub image_url: String,
    pub description: Option<String>,
    pub crt_by: String,
    pub crt_at: NaiveDateTime,
    pub upt_by: Option<String>,
    pub upt_at: NaiveDateTime,
}

impl From<&CreateScreenshotDto> for Screenshot {
    fn from(dto: &CreateScreenshotDto) -> Self {
        Screenshot {
            id: 0,
            phrase_id: dto.phrase_id,
            image_url: dto.image_url.clone(),
            description: dto.description.clone(),
            crt_by: "".to_string(),
            crt_at: Local::now().naive_local(),
            upt_by: None,
            upt_at: Local::now().naive_local(),
        }
    }
}

impl From<&UpdateScreenshotDto> for Screenshot {
    fn from(dto: &UpdateScreenshotDto) -> Self {
        Screenshot {
            id: 0,
            phrase_id: 0,
            image_url: dto.image_url.clone().unwrap_or_default(),
            description: dto.description.clone(),
            crt_by: "".to_string(),
            crt_at: Local::now().naive_local(),
            upt_by: None,
            upt_at: Local::now().naive_local(),
        }
    }
}