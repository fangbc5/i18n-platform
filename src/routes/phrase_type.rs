use crate::{
    dtos::phrase_type::{CreatePhraseTypeDto, UpdatePhraseTypeDto},
    errors::AppError,
    middleware::auth::Authentication,
    services::{phrase_type_service::PhraseTypeService, BaseService},
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

pub fn phrase_type_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .wrap(Authentication::default())
            .service(create_phrase_type)
            .service(get_phrase_types)
            .service(get_phrase_type)
            .service(update_phrase_type)
            .service(delete_phrase_type),
    );
}

#[post("")]
async fn create_phrase_type(
    phrase_type_service: web::Data<PhraseTypeService>,
    phrase_type: web::Json<CreatePhraseTypeDto>,
) -> Result<impl Responder, AppError> {
    let phrase_type = phrase_type_service.insert(&phrase_type.into_inner()).await?;
    Ok(HttpResponse::Created().json(phrase_type))
}

#[get("")]
async fn get_phrase_types(
    phrase_type_service: web::Data<PhraseTypeService>,
) -> Result<impl Responder, AppError> {
    let phrase_types = phrase_type_service.select_all().await?;
    Ok(HttpResponse::Ok().json(phrase_types))
}

#[get("/{id}")]
async fn get_phrase_type(
    phrase_type_service: web::Data<PhraseTypeService>,
    id: web::Path<u64>,
) -> Result<impl Responder, AppError> {
    let phrase_type = phrase_type_service.select_by_id(id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(phrase_type))
}

#[put("/{id}")]
async fn update_phrase_type(
    phrase_type_service: web::Data<PhraseTypeService>,
    id: web::Path<u64>,
    phrase_type: web::Json<UpdatePhraseTypeDto>,
) -> Result<impl Responder, AppError> {
    let phrase_type = phrase_type_service.update_by_id(id.into_inner(), &phrase_type.into_inner()).await?;
    Ok(HttpResponse::Ok().json(phrase_type))
}

#[delete("/{id}")]
async fn delete_phrase_type(
    phrase_type_service: web::Data<PhraseTypeService>,
    id: web::Path<u64>,
) -> Result<impl Responder, AppError> {
    phrase_type_service.delete_by_id(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
