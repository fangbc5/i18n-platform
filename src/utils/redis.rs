use crate::config::SETTINGS;

pub async fn init() -> redis::Client {
    redis::Client::open(format!(
        "redis://{}:{}",
        SETTINGS.redis.host, SETTINGS.redis.port
    ))
    .expect("Failed to create Redis client")
}
