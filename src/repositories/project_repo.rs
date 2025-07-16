use std::sync::Arc;

use async_trait::async_trait;
use sqlx::MySqlPool;

use crate::{errors::AppError, models::project::Project};

use super::BaseRepository;

pub struct ProjectRepository {
    pool: Arc<MySqlPool>,
}

impl ProjectRepository {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Self { pool }
    }

    /// 根据项目代码查找项目
    pub async fn find_by_code(&self, code: &str) -> Result<Option<Project>, AppError> {
        sqlx::query_as::<_, Project>(&format!(
            r#"
            SELECT * FROM {} WHERE code = ?
            "#,
            self.get_table_name()
        ))
        .bind(code)
        .fetch_optional(self.get_pool())
        .await
        .map_err(AppError::from)
    }

    /// 根据项目名称查找项目
    pub async fn find_by_name(&self, name: &str) -> Result<Option<Project>, AppError> {
        sqlx::query_as::<_, Project>(&format!(
            r#"
            SELECT * FROM {} WHERE name = ?
            "#,
            self.get_table_name()
        ))
        .bind(name)
        .fetch_optional(self.get_pool())
        .await
        .map_err(AppError::from)
    }
}

#[async_trait]
impl BaseRepository<Project> for ProjectRepository {
    fn get_table_name(&self) -> &str {
        "i18n_projects"
    }

    fn get_pool(&self) -> &MySqlPool {
        &self.pool
    }
}
