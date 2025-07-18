use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use sqlx::MySqlPool;
use tracing::info;

mod config;
mod constants;
mod dtos;
mod errors;
mod middleware;
mod models;
mod repositories;
mod routes;
mod services;
mod utils;

use crate::config::SETTINGS;
use crate::middleware::ErrorHandler;
use crate::services::language_service::LanguageService;
use crate::services::module_service::ModuleService;
use crate::services::phrase_service::PhraseService;
use crate::services::phrase_type_service::PhraseTypeService;
use crate::services::project_service::ProjectService;
use crate::services::screenshot_service::ScreenshotService;
use crate::services::term_service::TermService;
use crate::services::translation_service::TranslationService;
use crate::services::user_service::UserService;

#[derive(Clone)]
pub struct AppState {
    mysql_pool: Arc<MySqlPool>,
    redis_client: redis::Client,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 加载环境变量
    dotenv::dotenv().ok();

    // 初始化日志
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    tracing_subscriber::fmt::init();

    // 加载配置
    config::init();
    info!("配置加载完成");

    // 初始化数据库连接池
    let db_pool = Arc::new(
        utils::database::init()
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?,
    );
    info!("数据库连接池初始化完成");

    // 初始化Redis客户端
    let redis_client = utils::redis::init().await;
    info!("Redis客户端初始化完成");

    // 创建应用状态
    let state = Arc::new(AppState {
        mysql_pool: db_pool,
        redis_client,
    });

    // 启动HTTP服务器
    info!("正在启动服务器...");
    HttpServer::new(move || {
        // CORS配置
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(ErrorHandler::default())
            .app_data(web::Data::new(state.clone()))
            .app_data(web::Data::new(LanguageService::new(
                state.mysql_pool.clone(),
            )))
            .app_data(web::Data::new(ModuleService::new(state.mysql_pool.clone())))
            .app_data(web::Data::new(PhraseService::new(state.mysql_pool.clone())))
            .app_data(web::Data::new(PhraseTypeService::new(
                state.mysql_pool.clone(),
            )))
            .app_data(web::Data::new(ProjectService::new(
                state.mysql_pool.clone(),
            )))
            .app_data(web::Data::new(ScreenshotService::new(
                state.mysql_pool.clone(),
            )))
            .app_data(web::Data::new(TermService::new(state.mysql_pool.clone())))
            .app_data(web::Data::new(TranslationService::new(
                state.mysql_pool.clone(),
            )))
            .app_data(web::Data::new(UserService::new(state.mysql_pool.clone())))
            // API路由
            .service(
                web::scope("/api")
                    .service(web::scope("/auth").configure(routes::auth_routes))
                    .service(web::scope("/user").configure(routes::user_routes))
                    .service(web::scope("/project").configure(routes::project_routes))
                    .service(web::scope("/language").configure(routes::language_routes))
                    .service(web::scope("/module").configure(routes::module_routes))
                    .service(web::scope("/phrase").configure(routes::phrase_routes))
                    .service(web::scope("/translation").configure(routes::translation_routes))
                    .service(web::scope("/term").configure(routes::term_routes))
                    .service(web::scope("/screenshot").configure(routes::screenshot_routes)),
            )
            // 健康检查
            .route("/health", web::get().to(health_check))
    })
    .bind((SETTINGS.server.host.as_str(), SETTINGS.server.port))?
    .run()
    .await
}
async fn health_check() -> &'static str {
    "OK"
}
