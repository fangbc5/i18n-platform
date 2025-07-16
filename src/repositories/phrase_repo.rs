use std::sync::Arc;

use async_trait::async_trait;
use sqlx::MySqlPool;

use crate::{errors::AppError, models::phrase::Phrase};

use super::BaseRepository;

pub struct PhraseRepository {
    pool: Arc<MySqlPool>,
}

impl PhraseRepository {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Self { pool }
    }

    /// 根据项目ID查找短语列表
    pub async fn find_by_project_id(&self, project_id: u64) -> Result<Vec<Phrase>, AppError> {
        sqlx::query_as::<_, Phrase>(&format!(
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

    /// 根据模块ID查找短语列表
    pub async fn find_by_module_id(&self, module_id: u64) -> Result<Vec<Phrase>, AppError> {
        sqlx::query_as::<_, Phrase>(&format!(
            r#"
            SELECT * FROM {} WHERE module_id = ?
            "#,
            self.get_table_name()
        ))
        .bind(module_id)
        .fetch_all(self.get_pool())
        .await
        .map_err(AppError::from)
    }

    /// 根据项目ID和短语代码查找短语
    pub async fn find_by_project_and_key(
        &self,
        project_id: u64,
        key: &str,
    ) -> Result<Option<Phrase>, AppError> {
        sqlx::query_as::<_, Phrase>(&format!(
            r#"
            SELECT * FROM {} WHERE project_id = ? AND `key` = ?
            "#,
            self.get_table_name()
        ))
        .bind(project_id)
        .bind(key)
        .fetch_optional(self.get_pool())
        .await
        .map_err(AppError::from)
    }

    /// 根据类型ID查找短语列表
    pub async fn find_by_type_id(&self, type_id: u64) -> Result<Vec<Phrase>, AppError> {
        sqlx::query_as::<_, Phrase>(&format!(
            r#"
            SELECT * FROM {} WHERE type_id = ?
            "#,
            self.get_table_name()
        ))
        .bind(type_id)
        .fetch_all(self.get_pool())
        .await
        .map_err(AppError::from)
    }
}

#[async_trait]
impl BaseRepository<Phrase> for PhraseRepository {
    fn get_table_name(&self) -> &str {
        "i18n_phrases"
    }

    fn get_pool(&self) -> &MySqlPool {
        &self.pool
    }
}
