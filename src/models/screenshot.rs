use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::i18n_phrase_screenshots;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = i18n_phrase_screenshots)]
pub struct Screenshot {
    pub id: String,
    pub phrase_id: String,
    pub image_url: String,
    pub description: Option<String>,
    pub crt_by: String,
    pub crt_at: NaiveDateTime,
    pub upt_by: Option<String>,
    pub upt_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateScreenshot {
    pub phrase_id: String,
    pub image_url: String,
    pub description: Option<String>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = i18n_phrase_screenshots)]
pub struct UpdateScreenshot {
    pub image_url: Option<String>,
    pub description: Option<String>,
}
