use std::sync::Arc;

use async_trait::async_trait;
use sqlx::MySqlPool;

use crate::{errors::AppError, models::term::Term};

use super::BaseRepository;

pub struct TermRepository {
    pool: Arc<MySqlPool>,
}

impl TermRepository {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Self { pool }
    }

    /// 根据项目ID查找术语列表
    pub async fn find_by_project_id(&self, project_id: u64) -> Result<Vec<Term>, AppError> {
        sqlx::query_as::<_, Term>(&format!(
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

    /// 根据项目ID和术语代码查找术语
    pub async fn find_by_project_and_code(
        &self,
        project_id: u64,
        code: &str,
    ) -> Result<Vec<Term>, AppError> {
        sqlx::query_as::<_, Term>(&format!(
            r#"
            SELECT * FROM {} WHERE project_id = ? AND code = ?
            "#,
            self.get_table_name()
        ))
        .bind(project_id)
        .bind(code)
        .fetch_all(self.get_pool())
        .await
        .map_err(AppError::from)
    }
}

#[async_trait]
impl BaseRepository<Term> for TermRepository {
    fn get_table_name(&self) -> &str {
        "i18n_terms"
    }

    fn get_pool(&self) -> &MySqlPool {
        &self.pool
    }
}
