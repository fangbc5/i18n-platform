use crate::{
    dtos::common::ApiResponse,
    middleware::auth::Claims,
    models::{CreatePhraseType, UpdatePhraseType},
    repositories::DbPool,
    services::PhraseTypeService,
};
use axum::{
    extract::{Path, State},
    routing::{delete, get, post, put},
    Json, Router,
};

pub fn phrase_type_routes() -> Router<DbPool> {
    Router::new()
        .route("/phrase-types", get(list_phrase_types))
        .route("/phrase-types", post(create_phrase_type))
        .route("/phrase-types/:id", get(get_phrase_type))
        .route("/phrase-types/:id", put(update_phrase_type))
        .route("/phrase-types/:id", delete(delete_phrase_type))
}

async fn list_phrase_types(State(pool): State<DbPool>) -> Result<Json<ApiResponse>, ApiResponse> {
    let types = PhraseTypeService::find_all(&pool)
        .await
        .map_err(|e| ApiResponse::error(&e.to_string()))?;
    Ok(Json(ApiResponse::success(types)))
}

async fn get_phrase_type(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse>, ApiResponse> {
    let phrase_type = PhraseTypeService::find_by_id(&pool, &id)
        .await
        .map_err(|e| ApiResponse::error(&e.to_string()))?
        .ok_or_else(|| ApiResponse::not_found("Phrase type not found"))?;
    Ok(Json(ApiResponse::success(phrase_type)))
}

async fn create_phrase_type(
    State(pool): State<DbPool>,
    claims: Claims,
    Json(phrase_type): Json<CreatePhraseType>,
) -> Result<Json<ApiResponse>, ApiResponse> {
    let phrase_type = PhraseTypeService::create(&pool, &phrase_type, &claims.user_id)
        .await
        .map_err(|e| ApiResponse::error(&e.to_string()))?;
    Ok(Json(ApiResponse::success(phrase_type)))
}

async fn update_phrase_type(
    State(pool): State<DbPool>,
    claims: Claims,
    Path(id): Path<String>,
    Json(phrase_type): Json<UpdatePhraseType>,
) -> Result<Json<ApiResponse>, ApiResponse> {
    let phrase_type = PhraseTypeService::update(&pool, &id, &phrase_type, &claims.user_id)
        .await
        .map_err(|e| ApiResponse::error(&e.to_string()))?;
    Ok(Json(ApiResponse::success(phrase_type)))
}

async fn delete_phrase_type(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse>, ApiResponse> {
    let deleted = PhraseTypeService::delete(&pool, &id)
        .await
        .map_err(|e| ApiResponse::error(&e.to_string()))?;
    if deleted {
        Ok(Json(ApiResponse::success(
            "Phrase type deleted successfully",
        )))
    } else {
        Err(ApiResponse::not_found("Phrase type not found"))
    }
}
