use crate::{errors::AppError, services::TranslationService};
use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateTranslationRequest {
    pub phrase_id: String,
    pub language: String,
    pub translated_text: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTranslationRequest {
    pub translated_text: String,
    pub status: String,
}

pub fn translation_routes() -> Router {
    Router::new()
        .route("/", post(create_translation))
        .route("/:id", get(get_translation))
        .route("/phrase/:id", get(get_phrase_translations))
        .route("/:id", post(update_translation))
        .route("/:id", delete(delete_translation))
}

async fn create_translation(
    State(translation_service): State<TranslationService>,
    Path(translator_id): Path<String>,
    Json(req): Json<CreateTranslationRequest>,
) -> Result<Json<Translation>, AppError> {
    let translation = translation_service
        .create_translation(
            &req.phrase_id,
            &req.language,
            &req.translated_text,
            Some(&translator_id),
        )
        .await?;
    Ok(Json(translation))
}

async fn get_translation(
    State(translation_service): State<TranslationService>,
    Path(id): Path<String>,
) -> Result<Json<Translation>, AppError> {
    let translation = translation_service.get_translation(&id).await?;
    Ok(Json(translation))
}

async fn get_phrase_translations(
    State(translation_service): State<TranslationService>,
    Path(phrase_id): Path<String>,
) -> Result<Json<Vec<Translation>>, AppError> {
    let translations = translation_service
        .get_phrase_translations(&phrase_id)
        .await?;
    Ok(Json(translations))
}

async fn update_translation(
    State(translation_service): State<TranslationService>,
    Path(id): Path<String>,
    Json(req): Json<UpdateTranslationRequest>,
) -> Result<StatusCode, AppError> {
    translation_service
        .update_translation(&id, &req.translated_text, &req.status)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn delete_translation(
    State(translation_service): State<TranslationService>,
    Path(id): Path<String>,
) -> Result<StatusCode, AppError> {
    translation_service.delete_translation(&id).await?;
    Ok(StatusCode::NO_CONTENT)
}
