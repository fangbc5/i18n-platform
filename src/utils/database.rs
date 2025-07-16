use sqlx::{mysql::MySqlPoolOptions, MySqlPool};

use crate::config::SETTINGS;

pub async fn init() -> Result<MySqlPool, sqlx::Error> {
    MySqlPoolOptions::new()
        .max_connections(SETTINGS.database.pool_size)
        .connect(&SETTINGS.database.url)
        .await
}
