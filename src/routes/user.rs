use crate::{
    dtos::user::UpdateUserDto,
    errors::AppError,
    middleware::auth::Authentication,
    services::{
        user_service::UserService,
        BaseService,
    },
    utils::jwt::{self},
};
use actix_web::{delete, get, put, web, HttpRequest, HttpResponse, Responder};

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .wrap(Authentication::default())
            .service(get_users)
            .service(get_user)
            .service(update_user)
            .service(delete_user)
            .service(get_current_user),
    );
}

#[get("")]
async fn get_users(user_service: web::Data<UserService>) -> Result<impl Responder, AppError> {
    let users = user_service.select_all().await?;
    Ok(HttpResponse::Ok().json(users))
}

#[get("/me")]
async fn get_current_user(
    user_service: web::Data<UserService>,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let claims = jwt::get_claims(&req)?;
    let user = user_service.select_by_id(claims.sub).await?;
    Ok(HttpResponse::Ok().json(user))
}

#[get("/{id}")]
async fn get_user(
    user_service: web::Data<UserService>,
    id: web::Path<u64>,
) -> Result<impl Responder, AppError> {
    let user = user_service.select_by_id(id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(user))
}

#[put("/{id}")]
async fn update_user(
    user_service: web::Data<UserService>,
    id: web::Path<u64>,
    user: web::Json<UpdateUserDto>,
) -> Result<impl Responder, AppError> {
    let user = user_service
        .update_by_id(id.into_inner(), &user.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(user))
}

#[delete("/{id}")]
async fn delete_user(
    user_service: web::Data<UserService>,
    id: web::Path<u64>,
) -> Result<impl Responder, AppError> {
    user_service.delete_by_id(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
