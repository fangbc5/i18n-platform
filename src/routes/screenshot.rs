use crate::{
    dtos::screenshot::{CreateScreenshotDto, UpdateScreenshotDto},
    errors::AppError,
    middleware::auth::Authentication,
    services::{screenshot_service::ScreenshotService, BaseService},
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

pub fn screenshot_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .wrap(Authentication::default())
            .service(create_screenshot)
            .service(get_screenshots)
            .service(get_screenshot)
            .service(update_screenshot)
            .service(delete_screenshot),
    );
}

#[post("")]
async fn create_screenshot(
    screenshot_service: web::Data<ScreenshotService>,
    screenshot: web::Json<CreateScreenshotDto>,
) -> Result<impl Responder, AppError> {
    let screenshot = screenshot_service.insert(&screenshot.into_inner()).await?;
    Ok(HttpResponse::Created().json(screenshot))
}

#[get("")]
async fn get_screenshots(
    screenshot_service: web::Data<ScreenshotService>,
) -> Result<impl Responder, AppError> {
    let screenshots = screenshot_service.select_all().await?;
    Ok(HttpResponse::Ok().json(screenshots))
}

#[get("/{id}")]
async fn get_screenshot(
    screenshot_service: web::Data<ScreenshotService>,
    id: web::Path<u64>,
) -> Result<impl Responder, AppError> {
    let screenshot = screenshot_service.select_by_id(id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(screenshot))
}

#[put("/{id}")]
async fn update_screenshot(
    screenshot_service: web::Data<ScreenshotService>,
    id: web::Path<u64>,
    screenshot: web::Json<UpdateScreenshotDto>,
) -> Result<impl Responder, AppError> {
    let screenshot = screenshot_service.update_by_id(id.into_inner(), &screenshot.into_inner()).await?;
    Ok(HttpResponse::Ok().json(screenshot))
}

#[delete("/{id}")]
async fn delete_screenshot(
    screenshot_service: web::Data<ScreenshotService>,
    id: web::Path<u64>,
) -> Result<impl Responder, AppError> {
    screenshot_service.delete_by_id(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
