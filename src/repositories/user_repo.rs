use std::sync::Arc;

use async_trait::async_trait;
use sqlx::MySqlPool;

use crate::{errors::AppError, models::user::User};

use super::BaseRepository;

pub struct UserRepository {
    pool: Arc<MySqlPool>,
}

impl UserRepository {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Self { pool }
    }

    /// 根据用户名查找用户
    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
        sqlx::query_as::<_, User>(&format!(
            r#"
            SELECT * FROM {} WHERE username = ?
            "#,
            self.get_table_name()
        ))
        .bind(username)
        .fetch_optional(self.get_pool())
        .await
        .map_err(AppError::from)
    }

    /// 根据邮箱查找用户
    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        sqlx::query_as::<_, User>(&format!(
            r#"
            SELECT * FROM {} WHERE email = ?
            "#,
            self.get_table_name()
        ))
        .bind(email)
        .fetch_optional(self.get_pool())
        .await
        .map_err(AppError::from)
    }
}

#[async_trait]
impl BaseRepository<User> for UserRepository {
    fn get_table_name(&self) -> &str {
        "i18n_users"
    }

    fn get_pool(&self) -> &MySqlPool {
        &self.pool
    }
}
