use crate::{
    dtos::term::{CreateTermDto, UpdateTermDto},
    errors::AppError,
    middleware::auth::Authentication,
    services::{term_service::TermService, BaseService},
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

pub fn term_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .wrap(Authentication::default())
            .service(create_term)
            .service(get_terms)
            .service(get_term)
            .service(update_term)
            .service(delete_term),
    );
}

#[post("")]
async fn create_term(
    term_service: web::Data<TermService>,
    term: web::Json<CreateTermDto>,
) -> Result<impl Responder, AppError> {
    let term = term_service.insert(&term.into_inner()).await?;
    Ok(HttpResponse::Created().json(term))
}

#[get("")]
async fn get_terms(
    term_service: web::Data<TermService>,
) -> Result<impl Responder, AppError> {
    let terms = term_service.select_all().await?;
    Ok(HttpResponse::Ok().json(terms))
}

#[get("/{id}")]
async fn get_term(
    term_service: web::Data<TermService>,
    id: web::Path<u64>,
) -> Result<impl Responder, AppError> {
    let term = term_service.select_by_id(id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(term))
}

#[put("/{id}")]
async fn update_term(
    term_service: web::Data<TermService>,
    id: web::Path<u64>,
    term: web::Json<UpdateTermDto>,
) -> Result<impl Responder, AppError> {
    let term = term_service.update_by_id(id.into_inner(), &term.into_inner()).await?;
    Ok(HttpResponse::Ok().json(term))
}

#[delete("/{id}")]
async fn delete_term(
    term_service: web::Data<TermService>,
    id: web::Path<u64>,
) -> Result<impl Responder, AppError> {
    term_service.delete_by_id(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
