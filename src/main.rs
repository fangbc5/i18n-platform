mod config;
mod dtos;
mod errors;
mod middleware;
mod models;
mod repositories;
mod routes;
mod schema;
mod services;
mod utils;

use crate::{
    config::SETTINGS,
    routes::{
        auth::auth_routes, phrase::phrase_routes, project::project_routes, term::term_routes,
        translation::translation_routes,
    },
};
use axum::{extract::Extension, middleware::from_fn, routing::get, serve::serve, Router};
use std::{net::SocketAddr, sync::Arc};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

async fn healthcheck() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载配置
    dotenv::dotenv().ok();
    config::init();

    // 初始化日志
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 初始化数据库连接池
    let pool = repositories::init_pool()?;
    let pool = Arc::new(pool);

    // 初始化Redis连接池
    let redis_pool = utils::redis::init_pool()?;
    let redis_pool = Arc::new(redis_pool);

    // 初始化MinIO客户端
    let s3_client = utils::storage::init_s3_client().await?;
    let s3_client = Arc::new(s3_client);

    // 初始化Casbin执行器
    let casbin_enforcer = utils::casbin::init_enforcer().await?;
    let casbin_enforcer = Arc::new(casbin_enforcer);

    // 构建共享状态
    let state = Arc::new(AppState {
        db: pool.clone(),
        redis: redis_pool.clone(),
        s3: s3_client.clone(),
        casbin: casbin_enforcer.clone(),
    });

    // CORS配置
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 构建路由
    let app = Router::new()
        .route("/health", get(healthcheck))
        .nest("/api/auth", auth_routes())
        .nest("/api/projects", project_routes())
        .nest("/api/phrases", phrase_routes())
        .nest("/api/terms", term_routes())
        .nest("/api/translations", translation_routes())
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .layer(Extension(state));

    // 添加全局中间件
    let app = app.layer(from_fn(middleware::auth::auth_middleware));

    // 获取服务地址
    let addr = SocketAddr::from(([0, 0, 0, 0], SETTINGS.server.port));
    tracing::info!("服务启动于 {}", addr);

    // 启动服务
    serve(app.into_make_service(), addr).await?;

    Ok(())
}

// 共享状态结构
pub struct AppState {
    pub db: Arc<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::MysqlConnection>>>,
    pub redis: Arc<redis::Client>,
    pub s3: Arc<aws_sdk_s3::Client>,
    pub casbin: Arc<casbin::Enforcer>,
}
