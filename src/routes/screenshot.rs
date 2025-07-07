use crate::{
    dtos::common::ApiResponse,
    middleware::auth::Claims,
    models::{CreateScreenshot, UpdateScreenshot},
    repositories::DbPool,
    services::ScreenshotService,
};
use axum::{
    extract::{Path, State},
    routing::{delete, get, post, put},
    Json, Router,
};

pub fn screenshot_routes() -> Router<DbPool> {
    Router::new()
        .route("/screenshots", get(list_screenshots))
        .route("/screenshots", post(create_screenshot))
        .route("/screenshots/:id", get(get_screenshot))
        .route("/screenshots/:id", put(update_screenshot))
        .route("/screenshots/:id", delete(delete_screenshot))
        .route(
            "/phrases/:phrase_id/screenshots",
            get(list_phrase_screenshots),
        )
}

async fn list_screenshots(State(pool): State<DbPool>) -> Result<Json<ApiResponse>, ApiResponse> {
    let screenshots = ScreenshotService::find_all(&pool)
        .await
        .map_err(|e| ApiResponse::error(&e.to_string()))?;
    Ok(Json(ApiResponse::success(screenshots)))
}

async fn list_phrase_screenshots(
    State(pool): State<DbPool>,
    Path(phrase_id): Path<String>,
) -> Result<Json<ApiResponse>, ApiResponse> {
    let screenshots = ScreenshotService::find_by_phrase(&pool, &phrase_id)
        .await
        .map_err(|e| ApiResponse::error(&e.to_string()))?;
    Ok(Json(ApiResponse::success(screenshots)))
}

async fn get_screenshot(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse>, ApiResponse> {
    let screenshot = ScreenshotService::find_by_id(&pool, &id)
        .await
        .map_err(|e| ApiResponse::error(&e.to_string()))?
        .ok_or_else(|| ApiResponse::not_found("Screenshot not found"))?;
    Ok(Json(ApiResponse::success(screenshot)))
}

async fn create_screenshot(
    State(pool): State<DbPool>,
    claims: Claims,
    Json(screenshot): Json<CreateScreenshot>,
) -> Result<Json<ApiResponse>, ApiResponse> {
    let screenshot = ScreenshotService::create(&pool, &screenshot, &claims.user_id)
        .await
        .map_err(|e| ApiResponse::error(&e.to_string()))?;
    Ok(Json(ApiResponse::success(screenshot)))
}

async fn update_screenshot(
    State(pool): State<DbPool>,
    claims: Claims,
    Path(id): Path<String>,
    Json(screenshot): Json<UpdateScreenshot>,
) -> Result<Json<ApiResponse>, ApiResponse> {
    let screenshot = ScreenshotService::update(&pool, &id, &screenshot, &claims.user_id)
        .await
        .map_err(|e| ApiResponse::error(&e.to_string()))?;
    Ok(Json(ApiResponse::success(screenshot)))
}

async fn delete_screenshot(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse>, ApiResponse> {
    let deleted = ScreenshotService::delete(&pool, &id)
        .await
        .map_err(|e| ApiResponse::error(&e.to_string()))?;
    if deleted {
        Ok(Json(ApiResponse::success(
            "Screenshot deleted successfully",
        )))
    } else {
        Err(ApiResponse::not_found("Screenshot not found"))
    }
}
