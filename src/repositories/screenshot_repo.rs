use std::sync::Arc;

use async_trait::async_trait;
use sqlx::MySqlPool;

use crate::{errors::AppError, models::screenshot::Screenshot};

use super::BaseRepository;

pub struct ScreenshotRepository {
    pool: Arc<MySqlPool>,
}

impl ScreenshotRepository {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Self { pool }
    }

    /// 根据项目ID查找截图列表
    pub async fn find_by_project_id(&self, project_id: u64) -> Result<Vec<Screenshot>, AppError> {
        sqlx::query_as::<_, Screenshot>(&format!(
            r#"
            SELECT * FROM {} WHERE project_id = ?
            "#,
            self.get_table_name()
        ))
        .bind(project_id)
        .fetch_all(self.get_pool())
        .await
        .map_err(AppError::from)
    }

    /// 根据短语ID查找截图列表
    pub async fn find_by_phrase_id(&self, phrase_id: u64) -> Result<Vec<Screenshot>, AppError> {
        sqlx::query_as::<_, Screenshot>(&format!(
            r#"
            SELECT * FROM {} WHERE phrase_id = ?
            "#,
            self.get_table_name()
        ))
        .bind(phrase_id)
        .fetch_all(self.get_pool())
        .await
        .map_err(AppError::from)
    }
}

#[async_trait]
impl BaseRepository<Screenshot> for ScreenshotRepository {
    fn get_table_name(&self) -> &str {
        "i18n_screenshots"
    }

    fn get_pool(&self) -> &MySqlPool {
        &self.pool
    }
}
