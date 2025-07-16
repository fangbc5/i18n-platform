use crate::{
    dtos::phrase::{CreatePhraseDto, UpdatePhraseDto},
    errors::AppError,
    middleware::auth::Authentication,
    services::{phrase_service::PhraseService, BaseService},
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

pub fn phrase_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .wrap(Authentication::default())
            .service(create_phrase)
            .service(get_phrases)
            .service(get_phrase)
            .service(update_phrase)
            .service(delete_phrase),
    );
}

#[post("")]
async fn create_phrase(
    phrase_service: web::Data<PhraseService>,
    phrase: web::Json<CreatePhraseDto>,
) -> Result<impl Responder, AppError> {
    let phrase = phrase_service.insert(&phrase.into_inner()).await?;
    Ok(HttpResponse::Created().json(phrase))
}

#[get("")]
async fn get_phrases(
    phrase_service: web::Data<PhraseService>,
) -> Result<impl Responder, AppError> {
    let phrases = phrase_service.select_all().await?;
    Ok(HttpResponse::Ok().json(phrases))
}

#[get("/{id}")]
async fn get_phrase(
    phrase_service: web::Data<PhraseService>,
    id: web::Path<u64>,
) -> Result<impl Responder, AppError> {
    let phrase = phrase_service.select_by_id(id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(phrase))
}

#[put("/{id}")]
async fn update_phrase(
    phrase_service: web::Data<PhraseService>,
    id: web::Path<u64>,
    phrase: web::Json<UpdatePhraseDto>,
) -> Result<impl Responder, AppError> {
    let phrase = phrase_service.update_by_id(id.into_inner(), &phrase.into_inner()).await?;
    Ok(HttpResponse::Ok().json(phrase))
}

#[delete("/{id}")]
async fn delete_phrase(
    phrase_service: web::Data<PhraseService>,
    id: web::Path<u64>,
) -> Result<impl Responder, AppError> {
    phrase_service.delete_by_id(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
