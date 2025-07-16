use crate::errors::AppError;
use async_trait::async_trait;
use sqlx::MySqlPool;

#[async_trait]
pub trait BaseRepository<T> {
    fn get_pool(&self) -> &MySqlPool;
    fn get_id_column_name(&self) -> &str {
        "id"
    }
    fn get_table_name(&self) -> &str;

    async fn select_by_id(&self, id: u64) -> Result<Option<T>, AppError>
    where
        T: for<'r> sqlx::FromRow<'r, sqlx::mysql::MySqlRow> + Send + Unpin,
    {
        let query = format!(
            "SELECT * FROM {} WHERE {} = ?",
            self.get_table_name(),
            self.get_id_column_name()
        );
        let result = sqlx::query_as::<_, T>(&query)
            .bind(id)
            .fetch_optional(self.get_pool())
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(result)
    }

    async fn select_all(&self) -> Result<Vec<T>, AppError>
    where
        T: for<'r> sqlx::FromRow<'r, sqlx::mysql::MySqlRow> + Send + Unpin,
    {
        let query = format!("SELECT * FROM {}", self.get_table_name());
        let result = sqlx::query_as::<_, T>(&query)
            .fetch_all(self.get_pool())
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(result)
    }

    async fn select_by_page(&self, page: u32, page_size: u32) -> Result<(Vec<T>, u64), AppError>
    where
        T: for<'r> sqlx::FromRow<'r, sqlx::mysql::MySqlRow> + Send + Unpin,
    {
        let query = format!("SELECT COUNT(*) FROM {}", self.get_table_name());
        let count = sqlx::query_scalar::<_, u64>(&query)
            .fetch_one(self.get_pool())
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let query = format!("SELECT * FROM {} LIMIT ?, ?", self.get_table_name());
        let result = sqlx::query_as::<_, T>(&query)
            .bind((page - 1) * page_size)
            .bind(page_size)
            .fetch_all(self.get_pool())
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok((result, count))
    }

    async fn select_by_column(&self, column: &str, value: &str) -> Result<Vec<T>, AppError>
    where
        T: for<'r> sqlx::FromRow<'r, sqlx::mysql::MySqlRow> + Send + Unpin,
    {
        let query = format!(
            "SELECT * FROM {} WHERE {} = ?",
            self.get_table_name(),
            column
        );
        let result = sqlx::query_as::<_, T>(&query)
            .bind(value)
            .fetch_all(self.get_pool())
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(result)
    }

    async fn select_by_column_optional(
        &self,
        column: &str,
        value: &str,
    ) -> Result<Option<T>, AppError>
    where
        T: for<'r> sqlx::FromRow<'r, sqlx::mysql::MySqlRow> + Send + Unpin,
    {
        let query = format!(
            "SELECT * FROM {} WHERE {} = ?",
            self.get_table_name(),
            column
        );
        let result = sqlx::query_as::<_, T>(&query)
            .bind(value)
            .fetch_optional(self.get_pool())
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(result)
    }

    async fn insert(&self, entity: &T) -> Result<u64, AppError>
    where
        T: serde::Serialize + Send + Sync,
    {
        let query = format!("INSERT INTO {} SET ?", self.get_table_name());
        let result = sqlx::query(&query)
            .bind(serde_json::to_value(entity).unwrap())
            .execute(self.get_pool())
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(result.last_insert_id())
    }

    async fn update_by_id(&self, id: u64, entity: &T) -> Result<bool, AppError>
    where
        T: serde::Serialize + Send + Sync,
    {
        let query = format!(
            "UPDATE {} SET ? WHERE {} = ?",
            self.get_table_name(),
            self.get_id_column_name()
        );
        let result = sqlx::query(&query)
            .bind(serde_json::to_value(entity).unwrap())
            .bind(id)
            .execute(self.get_pool())
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(result.rows_affected() > 0)
    }

    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        let query = format!(
            "DELETE FROM {} WHERE {} = ?",
            self.get_table_name(),
            self.get_id_column_name()
        );
        let result = sqlx::query(&query)
            .bind(id)
            .execute(self.get_pool())
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(result.rows_affected() > 0)
    }

    async fn delete_by_ids(&self, ids: &[u64]) -> Result<u64, AppError> {
        let query = format!(
            "DELETE FROM {} WHERE {} IN (?)",
            self.get_table_name(),
            self.get_id_column_name()
        );
        let result = sqlx::query(&query)
            .bind(
                ids.iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            )
            .execute(self.get_pool())
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(result.rows_affected())
    }
}
