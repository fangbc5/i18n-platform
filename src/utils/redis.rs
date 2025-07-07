use crate::{config::SETTINGS, errors::AppError};
use redis::{Client, Commands};

pub fn init_pool() -> Result<Client, AppError> {
    Client::open(SETTINGS.redis.url.as_str()).map_err(AppError::Cache)
}

pub async fn set_with_expiry(
    client: &Client,
    key: &str,
    value: &str,
    expiry_secs: u64,
) -> Result<(), AppError> {
    let mut conn = client.get_connection().map_err(AppError::Cache)?;

    redis::pipe()
        .set(key, value)
        .expire(key, expiry_secs as i64)
        .query::<()>(&mut conn)
        .map_err(AppError::Cache)
}

pub async fn get(client: &Client, key: &str) -> Result<Option<String>, AppError> {
    let mut conn = client.get_connection().map_err(AppError::Cache)?;
    conn.get(key).map_err(AppError::Cache)
}

pub async fn delete(client: &Client, key: &str) -> Result<(), AppError> {
    let mut conn = client.get_connection().map_err(AppError::Cache)?;
    redis::pipe()
        .del(key)
        .query::<()>(&mut conn)
        .map_err(AppError::Cache)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_redis_operations() {
        let client = init_pool().unwrap();

        // 测试设置值
        set_with_expiry(&client, "test_key", "test_value", 60)
            .await
            .unwrap();

        // 测试获取值
        let value = get(&client, "test_key").await.unwrap();
        assert_eq!(value, Some("test_value".to_string()));

        // 测试删除值
        delete(&client, "test_key").await.unwrap();
        let value = get(&client, "test_key").await.unwrap();
        assert_eq!(value, None);
    }
}
