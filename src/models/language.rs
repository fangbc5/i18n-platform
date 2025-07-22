use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Language {
    pub id: u64,
    pub code: String,
    pub name: String,
    pub is_active: bool,
    pub is_native: bool,
    pub crt_by: String,
    pub crt_at: DateTime<Utc>,
    pub upt_by: Option<String>,
    pub upt_at: DateTime<Utc>,
}
