use std::sync::Arc;

use async_trait::async_trait;
use sqlx::MySqlPool;

use crate::{errors::AppError, models::language::Language};

use super::BaseRepository;

pub struct LanguageRepository {
    pool: Arc<MySqlPool>,
}

impl LanguageRepository {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Self { pool }
    }

    /// 根据语言代码查找语言
    pub async fn find_by_code(&self, code: &str) -> Result<Option<Language>, AppError> {
        sqlx::query_as::<_, Language>(&format!(
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
    /// 根据语言名称查找语言
    pub async fn find_by_name(&self, name: &str) -> Result<Option<Language>, AppError> {
        sqlx::query_as::<_, Language>(&format!(
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
impl BaseRepository<Language> for LanguageRepository {
    fn get_table_name(&self) -> &str {
        "i18n_languages"
    }

    fn get_pool(&self) -> &MySqlPool {
        &self.pool
    }
}
