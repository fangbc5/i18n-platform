use crate::{
    dtos::common::ApiResponse,
    middleware::auth::Claims,
    models::{CreateModule, UpdateModule},
    repositories::DbPool,
    services::ModuleService,
};
use axum::{
    extract::{Path, State},
    routing::{delete, get, post, put},
    Json, Router,
};

pub fn module_routes() -> Router<DbPool> {
    Router::new()
        .route("/modules", get(list_modules))
        .route("/modules", post(create_module))
        .route("/modules/:id", get(get_module))
        .route("/modules/:id", put(update_module))
        .route("/modules/:id", delete(delete_module))
        .route("/projects/:project_id/modules", get(list_project_modules))
}

async fn list_modules(State(pool): State<DbPool>) -> Result<Json<ApiResponse>, ApiResponse> {
    let modules = ModuleService::find_all(&pool)
        .await
        .map_err(|e| ApiResponse::error(&e.to_string()))?;
    Ok(Json(ApiResponse::success(modules)))
}

async fn list_project_modules(
    State(pool): State<DbPool>,
    Path(project_id): Path<String>,
) -> Result<Json<ApiResponse>, ApiResponse> {
    let modules = ModuleService::find_by_project(&pool, &project_id)
        .await
        .map_err(|e| ApiResponse::error(&e.to_string()))?;
    Ok(Json(ApiResponse::success(modules)))
}

async fn get_module(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse>, ApiResponse> {
    let module = ModuleService::find_by_id(&pool, &id)
        .await
        .map_err(|e| ApiResponse::error(&e.to_string()))?
        .ok_or_else(|| ApiResponse::not_found("Module not found"))?;
    Ok(Json(ApiResponse::success(module)))
}

async fn create_module(
    State(pool): State<DbPool>,
    claims: Claims,
    Json(module): Json<CreateModule>,
) -> Result<Json<ApiResponse>, ApiResponse> {
    let module = ModuleService::create(&pool, &module, &claims.user_id)
        .await
        .map_err(|e| ApiResponse::error(&e.to_string()))?;
    Ok(Json(ApiResponse::success(module)))
}

async fn update_module(
    State(pool): State<DbPool>,
    claims: Claims,
    Path(id): Path<String>,
    Json(module): Json<UpdateModule>,
) -> Result<Json<ApiResponse>, ApiResponse> {
    let module = ModuleService::update(&pool, &id, &module, &claims.user_id)
        .await
        .map_err(|e| ApiResponse::error(&e.to_string()))?;
    Ok(Json(ApiResponse::success(module)))
}

async fn delete_module(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse>, ApiResponse> {
    let deleted = ModuleService::delete(&pool, &id)
        .await
        .map_err(|e| ApiResponse::error(&e.to_string()))?;
    if deleted {
        Ok(Json(ApiResponse::success("Module deleted successfully")))
    } else {
        Err(ApiResponse::not_found("Module not found"))
    }
}
