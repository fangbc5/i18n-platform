use crate::dtos::common::PageRequest;
use crate::{
    dtos::language::{CreateLanguageDto, UpdateLanguageDto},
    errors::AppError,
    middleware::auth::Authentication,
    services::{language_service::LanguageService, BaseService},
    AppState,
};
use actix_web::{delete, get, post, put, web, HttpResponse};
use crate::utils::{PageR, R};

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

#[get("/list")]
async fn get_languages(
    language_service: web::Data<LanguageService>,
    req: web::Query<PageRequest>,
) -> Result<HttpResponse, AppError> {
    let languages = language_service.select_by_page(&req).await?;
    Ok(PageR::ok(languages))
}

#[get("/detail/{id}")]
async fn get_language(
    language_service: web::Data<LanguageService>,
    id: web::Path<u64>,
) -> Result<HttpResponse, AppError> {
    let language = language_service.select_by_id(id.into_inner()).await?;
    Ok(R::ok(language))
}

#[post("")]
async fn create_language(
    language_service: web::Data<LanguageService>,
    language: web::Json<CreateLanguageDto>,
) -> Result<HttpResponse, AppError> {
    let language = language_service.insert(&language.into_inner()).await?;
    Ok(R::ok(language))
}

#[put("/{id}")]
async fn update_language(
    language_service: web::Data<LanguageService>,
    id: web::Path<u64>,
    language: web::Json<UpdateLanguageDto>,
) -> Result<HttpResponse, AppError> {
    let language = language_service
        .update_by_id(id.into_inner(), &language.into_inner())
        .await?;
    Ok(R::ok(language))
}

#[delete("/{id}")]
async fn delete_language(
    language_service: web::Data<LanguageService>,
    id: web::Path<u64>,
) -> Result<HttpResponse, AppError> {
    let result = language_service.delete_by_id(id.into_inner()).await?;
    Ok(R::ok(result))
}
