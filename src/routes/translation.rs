use crate::{
    dtos::translation::{CreateTranslationDto, UpdateTranslationDto},
    errors::AppError,
    middleware::auth::Authentication,
    services::{translation_service::TranslationService, BaseService},
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

pub fn translation_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .wrap(Authentication::default())
            .service(create_translation)
            .service(get_translations)
            .service(get_translation)
            .service(update_translation)
            .service(delete_translation),
    );
}

#[post("")]
async fn create_translation(
    translation_service: web::Data<TranslationService>,
    translation: web::Json<CreateTranslationDto>,
) -> Result<impl Responder, AppError> {
    let translation = translation_service.insert(&translation.into_inner()).await?;
    Ok(HttpResponse::Created().json(translation))
}

#[get("")]
async fn get_translations(
    translation_service: web::Data<TranslationService>,
) -> Result<impl Responder, AppError> {
    let translations = translation_service.select_all().await?;
    Ok(HttpResponse::Ok().json(translations))
}

#[get("/{id}")]
async fn get_translation(
    translation_service: web::Data<TranslationService>,
    id: web::Path<u64>,
) -> Result<impl Responder, AppError> {
    let translation = translation_service.select_by_id(id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(translation))
}

#[put("/{id}")]
async fn update_translation(
    translation_service: web::Data<TranslationService>,
    id: web::Path<u64>,
    translation: web::Json<UpdateTranslationDto>,
) -> Result<impl Responder, AppError> {
    let translation = translation_service.update_by_id(id.into_inner(), &translation.into_inner()).await?;
    Ok(HttpResponse::Ok().json(translation))
}

#[delete("/{id}")]
async fn delete_translation(
    translation_service: web::Data<TranslationService>,
    id: web::Path<u64>,
) -> Result<impl Responder, AppError> {
    translation_service.delete_by_id(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
