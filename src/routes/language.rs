use crate::{
    dtos::language::{CreateLanguageDto, UpdateLanguageDto},
    errors::AppError,
    middleware::auth::Authentication,
    services::{language_service::LanguageService, BaseService},
    AppState,
};
use actix_web::{delete, get, post, put, web, HttpResponse};

pub fn language_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .wrap(Authentication::default())
            .service(create_language)
            .service(get_languages)
            .service(get_language)
            .service(update_language)
            .service(delete_language),
    );
}

#[post("")]
async fn create_language(
    language: web::Json<CreateLanguageDto>,
    language_service: web::Data<LanguageService>,
) -> Result<HttpResponse, AppError> {
    let language = language_service.insert(language.into_inner()).await?;
    Ok(HttpResponse::Created().json(language))
}

#[get("")]
async fn get_languages(language_service: web::Data<LanguageService>) -> Result<HttpResponse, AppError> {
    let languages = language_service.select_all().await?;
    Ok(HttpResponse::Ok().json(languages))
}

#[get("/{id}")]
async fn get_language(
    state: web::Data<AppState>,
    id: web::Path<u64>,
) -> Result<HttpResponse, AppError> {
    let service = LanguageService::new(state.mysql_pool.clone());
    let language = service.select_by_id(id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(language))
}

#[put("/{id}")]
async fn update_language(
    language_service: web::Data<LanguageService>,
    id: web::Path<u64>,
    language: web::Json<UpdateLanguageDto>,
) -> Result<HttpResponse, AppError> {
    let language = language_service
        .update_by_id(id.into_inner(), language.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(language))
}

#[delete("/{id}")]
async fn delete_language(
    state: web::Data<AppState>,
    id: web::Path<u64>,
) -> Result<HttpResponse, AppError> {
    let service = LanguageService::new(state.mysql_pool.clone());
    service.delete_by_id(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
