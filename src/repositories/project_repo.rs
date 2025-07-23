use std::sync::Arc;

use async_trait::async_trait;
use sqlx::MySqlPool;

use crate::{errors::AppError, models::project::Project};
use crate::models::language::Language;
use super::BaseRepository;

pub struct ProjectRepository {
    pool: Arc<MySqlPool>,
}

impl ProjectRepository {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Self { pool }
    }

    /// 根据项目代码查找项目
    pub async fn find_by_code(&self, code: &str, exclude_id: u64) -> Result<Option<Project>, AppError> {
        let sql = if exclude_id > 0 {
            format!(
                r#"
            SELECT * FROM {} WHERE code = ? and id != {}
            "#,
                self.get_table_name(),
                exclude_id
            )
        } else {
            format!(
                r#"
            SELECT * FROM {} WHERE code = ?
            "#,
                self.get_table_name()
            )
        };
        sqlx::query_as::<_, Project>(&sql)
        .bind(code)
        .fetch_optional(self.get_pool())
        .await
        .map_err(AppError::from)
    }

    /// 根据项目名称查找项目
    pub async fn find_by_name(&self, name: &str, exclude_id: u64) -> Result<Option<Project>, AppError> {
        let sql = if exclude_id > 0 {
            format!(
                r#"
            SELECT * FROM {} WHERE name = ? and id != {}
            "#,
                self.get_table_name(),
                exclude_id
            )
        } else {
            format!(
                r#"
            SELECT * FROM {} WHERE name = ?
            "#,
                self.get_table_name()
            )
        };
        sqlx::query_as::<_, Project>(&sql)
        .bind(name)
        .fetch_optional(self.get_pool())
        .await
        .map_err(AppError::from)
    }

    /// 根据编码或名称搜索项目
    pub async fn select_page_by_key(
        &self,
        page: u32,
        size: u32,
        key: &str,
    ) -> Result<(Vec<Project>, i64), AppError> {
        let query_count = format!(
            "SELECT count(*) FROM {} WHERE code LIKE '%{}%' OR name LIKE '%{}%'",
            self.get_table_name(),
            key,
            key
        );
        let query = format!(
            "SELECT * FROM {} WHERE code LIKE '%{}%' OR name LIKE '%{}%' LIMIT ?, ?",
            self.get_table_name(),
            key,
            key
        );
        let count = self.select_count(query_count).await?;
        let projects = sqlx::query_as::<_, Project>(&query)
            .bind((page - 1) * size)
            .bind(size)
            .fetch_all(self.get_pool())
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok((projects, count))
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
