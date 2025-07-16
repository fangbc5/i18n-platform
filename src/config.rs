use dotenv::dotenv;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
    pub pool_size: u32,
}

#[derive(Debug, Deserialize)]
pub struct Redis {
    pub host: String,
    pub port: u16,
    pub password: Option<String>,
    pub database: Option<u8>,
}

#[derive(Debug, Deserialize)]
pub struct Minio {
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
    pub bucket: String,
    #[serde(default = "default_region")]
    pub region: String,
}

fn default_region() -> String {
    "us-east-1".to_string()
}

#[derive(Debug, Deserialize)]
pub struct Kafka {
    pub brokers: String,
    pub group_id: String,
}

#[derive(Debug, Deserialize)]
pub struct Jwt {
    pub secret: String,
    pub expiration: i64,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

#[derive(Debug)]
pub struct Settings {
    pub server: Server,
    pub database: Database,
    pub redis: Redis,
    pub minio: Minio,
    pub kafka: Kafka,
    pub jwt: Jwt,
    pub environment: String,
}

impl Settings {
    pub fn new() -> Result<Self, env::VarError> {
        dotenv().ok();

        Ok(Self {
            server: Server {
                host: env::var("SERVER_HOST")?,
                port: env::var("SERVER_PORT")?.parse().unwrap_or(8080),
            },
            database: Database {
                url: env::var("DATABASE_URL")?,
                pool_size: env::var("DATABASE_POOL_SIZE")?.parse().unwrap_or(10),
            },
            redis: Redis {
                host: env::var("REDIS_HOST").unwrap_or("localhost".to_string()),
                port: env::var("REDIS_PORT")
                    .unwrap_or("6379".to_string())
                    .parse()
                    .unwrap_or(6379),
                password: env::var("REDIS_PASSWORD").ok(),
                database: env::var("REDIS_DATABASE")
                    .ok()
                    .map(|s| s.parse().unwrap_or(0)),
            },
            minio: Minio {
                endpoint: env::var("MINIO_ENDPOINT")?,
                access_key: env::var("MINIO_ACCESS_KEY")?,
                secret_key: env::var("MINIO_SECRET_KEY")?,
                bucket: env::var("MINIO_BUCKET")?,
                region: env::var("MINIO_REGION").unwrap_or(default_region()),
            },
            kafka: Kafka {
                brokers: env::var("KAFKA_BROKERS")?,
                group_id: env::var("KAFKA_GROUP_ID")?,
            },
            jwt: Jwt {
                secret: env::var("JWT_SECRET")?,
                expiration: env::var("JWT_EXPIRATION")?.parse().unwrap_or(86400),
            },
            environment: env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
        })
    }
}

// 全局配置实例
lazy_static::lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new().expect("配置加载失败");
}

// 辅助函数
pub fn init() {
    lazy_static::initialize(&SETTINGS);
    tracing::info!("配置加载完成: {:?}", *SETTINGS);
}
