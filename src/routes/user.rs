use crate::{errors::AppError, middleware::auth::AuthenticatedUser, services::UserService};
use actix_web::{HttpResponse, Scope, web};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
}

async fn create_user(
    req: web::Json<CreateUserRequest>,
    user_service: web::Data<UserService>,
) -> Result<HttpResponse, AppError> {
    let user = user_service
        .create_user(&req.username, &req.email, &req.password)
        .await?;

    Ok(HttpResponse::Created().json(UserResponse {
        id: user.id,
        username: user.username,
        email: user.email,
    }))
}

async fn get_current_user(
    auth_user: AuthenticatedUser,
    user_service: web::Data<UserService>,
) -> Result<HttpResponse, AppError> {
    let user = user_service.get_user_by_id(auth_user.user_id).await?;

    Ok(HttpResponse::Ok().json(UserResponse {
        id: user.id,
        username: user.username,
        email: user.email,
    }))
}

pub fn user_routes() -> Scope {
    web::scope("/users")
        .route("", web::post().to(create_user))
        .route("/me", web::get().to(get_current_user))
}
