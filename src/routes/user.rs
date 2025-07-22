use crate::dtos::user::{PageUserRequest, UserIdsRequest};
use crate::utils::PageR;
use crate::{
    dtos::user::{CreateUserDto, UpdateUserDto, UserVo},
    errors::AppError,
    middleware::auth::Authentication,
    services::{user_service::UserService, BaseService},
    utils::{
        jwt::{self},
        R,
    },
};
use actix_web::{delete, get, post, put, web, HttpRequest, Responder};

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .wrap(Authentication::default())
            .service(get_users)
            .service(get_user)
            .service(update_user)
            .service(delete_user)
            .service(create_user)
            .service(get_current_user)
            .service(batch_delete_users),
    );
}
#[get("/profile")]
async fn get_current_user(
    user_service: web::Data<UserService>,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let claims = jwt::get_claims(&req)?;
    let user = user_service.select_by_id(claims.sub).await?;
    Ok(R::ok(UserVo::from(&user)))
}

#[get("/list")]
async fn get_users(
    user_service: web::Data<UserService>,
    user: web::Query<PageUserRequest>,
) -> Result<impl Responder, AppError> {
    let users = user_service.select_by_page(&user).await?;
    Ok(PageR::ok(users))
}

#[get("/{id:\\d+}")]
async fn get_user(
    user_service: web::Data<UserService>,
    id: web::Path<u64>,
) -> Result<impl Responder, AppError> {
    let user = user_service.select_by_id(id.into_inner()).await?;
    Ok(R::ok(UserVo::from(&user)))
}

#[post("/add")]
async fn create_user(
    user_service: web::Data<UserService>,
    mut user: web::Json<CreateUserDto>,
    http_request: HttpRequest
) -> Result<impl Responder, AppError> {
    let claims = jwt::get_claims(&http_request)?;
    user.crt_by = claims.username;
    let user = user_service.create_user(&user.into_inner()).await?;
    Ok(R::ok(user))
}

#[put("/{id}")]
async fn update_user(
    user_service: web::Data<UserService>,
    id: web::Path<u64>,
    mut user: web::Json<UpdateUserDto>,
    http_request: HttpRequest
) -> Result<impl Responder, AppError> {
    let claims = jwt::get_claims(&http_request)?;
    user.upt_by = claims.username;
    let user = user_service
        .update_by_id(id.into_inner(), &user.into_inner())
        .await?;
    Ok(R::ok(user))
}

#[delete("/{id}")]
async fn delete_user(
    user_service: web::Data<UserService>,
    id: web::Path<u64>,
) -> Result<impl Responder, AppError> {
    user_service.delete_by_id(id.into_inner()).await?;
    Ok(R::ok(()))
}

#[delete("/batch/{ids}")]
async fn batch_delete_users(
    user_service: web::Data<UserService>,
    req: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let ids = req.into_inner().split(",").map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let affected = user_service.delete_by_ids(&ids).await?;
    Ok(R::ok(affected))
}
