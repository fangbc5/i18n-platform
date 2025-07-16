use std::sync::Arc;

use async_trait::async_trait;
use sqlx::MySqlPool;

use crate::{errors::AppError, models::module::Module};

use super::BaseRepository;

pub struct ModuleRepository {
    pool: Arc<MySqlPool>,
}

impl ModuleRepository {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Self { pool }
    }

    /// 根据项目ID查找模块列表
    pub async fn find_by_project_id(&self, project_id: u64) -> Result<Vec<Module>, AppError> {
        sqlx::query_as::<_, Module>(&format!(
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

    /// 根据项目ID和模块代码查找模块
    pub async fn find_by_project_and_code(
        &self,
        project_id: u64,
        code: &str,
    ) -> Result<Option<Module>, AppError> {
        sqlx::query_as::<_, Module>(&format!(
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
impl BaseRepository<Module> for ModuleRepository {
    fn get_table_name(&self) -> &str {
        "i18n_modules"
    }

    fn get_pool(&self) -> &MySqlPool {
        &self.pool
    }
}
