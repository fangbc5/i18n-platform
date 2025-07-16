use std::sync::Arc;

use async_trait::async_trait;
use sqlx::MySqlPool;

use crate::{errors::AppError, models::translation::Translation};

use super::BaseRepository;

pub struct TranslationRepository {
    pool: Arc<MySqlPool>,
}

impl TranslationRepository {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Self { pool }
    }

    /// 根据项目ID和语言代码查找翻译列表
    pub async fn find_by_project_and_language(
        &self,
        project_id: u64,
        language_code: &str,
    ) -> Result<Vec<Translation>, AppError> {
        sqlx::query_as::<_, Translation>(&format!(
            r#"
            SELECT * FROM {} WHERE project_id = ? AND language_code = ?
            "#,
            self.get_table_name()
        ))
        .bind(project_id)
        .bind(language_code)
        .fetch_all(self.get_pool())
        .await
        .map_err(AppError::from)
    }

    /// 根据短语ID和语言代码查找翻译
    pub async fn find_by_phrase_and_language(
        &self,
        phrase_id: u64,
        language_code: &str,
    ) -> Result<Option<Translation>, AppError> {
        sqlx::query_as::<_, Translation>(&format!(
            r#"
            SELECT * FROM {} WHERE phrase_id = ? AND language_code = ?
            "#,
            self.get_table_name()
        ))
        .bind(phrase_id)
        .bind(language_code)
        .fetch_optional(self.get_pool())
        .await
        .map_err(AppError::from)
    }

    /// 根据项目ID查找翻译列表
    pub async fn find_by_project_id(&self, project_id: u64) -> Result<Vec<Translation>, AppError> {
        sqlx::query_as::<_, Translation>(&format!(
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
}

#[async_trait]
impl BaseRepository<Translation> for TranslationRepository {
    fn get_table_name(&self) -> &str {
        "i18n_translations"
    }

    fn get_pool(&self) -> &MySqlPool {
        &self.pool
    }
}
