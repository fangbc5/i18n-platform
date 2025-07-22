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
    
    async fn select_count(&self, sql: String) -> Result<i64, AppError> {
        let count = sqlx::query_scalar::<_, i64>(&sql)
            .fetch_optional(self.get_pool())
            .await
            .map_err(|e| AppError::Database(e.to_string()))?.unwrap_or(0);
        Ok(count)
    }

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

    async fn select_by_page(&self, page: u32, page_size: u32) -> Result<(Vec<T>, i64), AppError>
    where
        T: for<'r> sqlx::FromRow<'r, sqlx::mysql::MySqlRow> + Send + Unpin,
    {
        let query = format!("SELECT COUNT(*) FROM {}", self.get_table_name());
        let count = sqlx::query_scalar::<_, i64>(&query)
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
        let value = serde_json::to_value(entity)
            .map_err(|e| AppError::Database(format!("Failed to serialize entity: {}", e)))?;

        let obj = value
            .as_object()
            .ok_or_else(|| AppError::Database("Entity must be an object".into()))?;

        let columns: Vec<&str> = obj.keys().map(String::as_str).collect();
        let values: Vec<_> = obj.values().collect();

        let placeholders = (0..columns.len())
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(", ");

        let query = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            self.get_table_name(),
            columns.join(", "),
            placeholders
        );

        let mut query_builder = sqlx::query(&query);
        for value in values {
            match value {
                serde_json::Value::Null => query_builder = query_builder.bind(None::<String>),
                serde_json::Value::Bool(b) => query_builder = query_builder.bind(*b),
                serde_json::Value::Number(n) => {
                    if let Some(i) = n.as_i64() {
                        query_builder = query_builder.bind(i);
                    } else if let Some(f) = n.as_f64() {
                        query_builder = query_builder.bind(f);
                    }
                }
                serde_json::Value::String(s) => {
                    // 尝试解析日期时间字符串
                    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(s) {
                        query_builder = query_builder.bind(dt.naive_utc());
                    } else {
                        query_builder = query_builder.bind(s);
                    }
                }
                _ => return Err(AppError::Database("Unsupported value type".into())),
            }
        }

        let result = query_builder
            .execute(self.get_pool())
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.last_insert_id())
    }

    async fn update_by_id(&self, id: u64, entity: &T) -> Result<bool, AppError>
    where
        T: serde::Serialize + Send + Sync,
    {
        let value = serde_json::to_value(entity)
            .map_err(|e| AppError::Database(format!("Failed to serialize entity: {}", e)))?;

        let obj = value
            .as_object()
            .ok_or_else(|| AppError::Database("Entity must be an object".into()))?;

        // 过滤掉 null 值的字段
        let mut update_fields = Vec::new();
        let mut values = Vec::new();

        for (key, value) in obj {
            if !value.is_null() {
                update_fields.push(format!("{} = ?", key));
                values.push(value);
            }
        }

        if update_fields.is_empty() {
            return Ok(false);
        }

        let query = format!(
            "UPDATE {} SET {} WHERE {} = ?",
            self.get_table_name(),
            update_fields.join(", "),
            self.get_id_column_name()
        );

        let mut query_builder = sqlx::query(&query);

        for value in values {
            match value {
                serde_json::Value::Bool(b) => query_builder = query_builder.bind(b),
                serde_json::Value::Number(n) => {
                    if let Some(i) = n.as_i64() {
                        query_builder = query_builder.bind(i);
                    } else if let Some(f) = n.as_f64() {
                        query_builder = query_builder.bind(f);
                    }
                }
                serde_json::Value::String(s) => {
                    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&s) {
                        query_builder = query_builder.bind(dt.naive_utc());
                    } else {
                        query_builder = query_builder.bind(s);
                    }
                }
                _ => return Err(AppError::Database("Unsupported value type".into())),
            }
        }

        query_builder = query_builder.bind(id);

        let result = query_builder
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
        if ids.is_empty() {
            return Ok(0);
        }

        let placeholders = std::iter::repeat("?")
            .take(ids.len())
            .collect::<Vec<_>>()
            .join(",");

        let query = format!(
            "DELETE FROM {} WHERE {} IN ({})",
            self.get_table_name(),
            self.get_id_column_name(),
            placeholders
        );

        let mut query_builder = sqlx::query(&query);
        for id in ids {
            query_builder = query_builder.bind(*id);
        }

        let result = query_builder
            .execute(self.get_pool())
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.rows_affected())
    }
}
