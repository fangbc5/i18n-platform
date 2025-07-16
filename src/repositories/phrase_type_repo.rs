use std::sync::Arc;

use async_trait::async_trait;
use sqlx::MySqlPool;

use crate::{errors::AppError, models::phrase_type::PhraseType};

use super::BaseRepository;

pub struct PhraseTypeRepository {
    pool: Arc<MySqlPool>,
}

impl PhraseTypeRepository {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Self { pool }
    }

    /// 根据项目ID查找短语类型列表
    pub async fn find_by_project_id(&self, project_id: u64) -> Result<Vec<PhraseType>, AppError> {
        sqlx::query_as::<_, PhraseType>(&format!(
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

    /// 根据项目ID和类型代码查找短语类型
    pub async fn find_by_project_and_code(
        &self,
        project_id: u64,
        code: &str,
    ) -> Result<Option<PhraseType>, AppError> {
        sqlx::query_as::<_, PhraseType>(&format!(
            r#"
            SELECT * FROM {} WHERE project_id = ? AND code = ?
            "#,
            self.get_table_name()
        ))
        .bind(project_id)
        .bind(code)
        .fetch_optional(self.get_pool())
        .await
        .map_err(AppError::from)
    }
}

#[async_trait]
impl BaseRepository<PhraseType> for PhraseTypeRepository {
    fn get_table_name(&self) -> &str {
        "i18n_phrase_types"
    }

    fn get_pool(&self) -> &MySqlPool {
        &self.pool
    }
}
