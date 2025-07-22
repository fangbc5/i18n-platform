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
        self.select_by_column_optional("username", username).await
    }

    /// 根据邮箱查找用户
    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        self.select_by_column_optional("email", email).await
    }

    /// 根据手机号查找用户
    pub async fn find_by_phone(&self, phone: &str) -> Result<Option<User>, AppError> {
        self.select_by_column_optional("phone", phone).await
    }

    /// 根据用户名、手机号、邮箱、昵称、真实姓名搜索用户
    pub async fn select_page_by_key(&self,page: u32, size: u32, key: &str) -> Result<(Vec<User>, i64), AppError> {
        let query_count = format!("SELECT count(*) FROM {} WHERE username LIKE '%{}%' OR email LIKE '%{}%' OR phone LIKE '%{}%' OR realname LIKE '%{}%' OR nickname LIKE '%{}%'", self.get_table_name(), key, key, key, key, key);
        let query = format!("SELECT * FROM {} WHERE username LIKE '%{}%' OR email LIKE '%{}%' OR phone LIKE '%{}%' OR realname LIKE '%{}%' OR nickname LIKE '%{}%' LIMIT ?, ?", self.get_table_name(), key, key, key, key, key);
        let count = self.select_count(query_count).await?;
        let users = sqlx::query_as::<_, User>(&query)
            .bind((page - 1) * size)
            .bind(size)
            .fetch_all(self.get_pool())
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok((users, count))
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
