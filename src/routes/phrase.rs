use crate::{errors::AppError, services::PhraseService};
use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreatePhraseRequest {
    pub project_id: String,
    pub key: String,
    pub source_text: String,
    pub context: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePhraseRequest {
    pub key: Option<String>,
    pub source_text: Option<String>,
    pub context: Option<String>,
}

pub fn phrase_routes() -> Router {
    Router::new()
        .route("/", post(create_phrase))
        .route("/:id", get(get_phrase))
        .route("/project/:id", get(get_project_phrases))
        .route("/:id", post(update_phrase))
        .route("/:id", delete(delete_phrase))
}

async fn create_phrase(
    State(phrase_service): State<PhraseService>,
    Json(req): Json<CreatePhraseRequest>,
) -> Result<Json<Phrase>, AppError> {
    let phrase = phrase_service
        .create_phrase(
            &req.project_id,
            &req.key,
            &req.source_text,
            req.context.as_deref(),
        )
        .await?;
    Ok(Json(phrase))
}

async fn get_phrase(
    State(phrase_service): State<PhraseService>,
    Path(id): Path<String>,
) -> Result<Json<Phrase>, AppError> {
    let phrase = phrase_service.get_phrase(&id).await?;
    Ok(Json(phrase))
}

async fn get_project_phrases(
    State(phrase_service): State<PhraseService>,
    Path(project_id): Path<String>,
) -> Result<Json<Vec<Phrase>>, AppError> {
    let phrases = phrase_service.get_project_phrases(&project_id).await?;
    Ok(Json(phrases))
}

async fn update_phrase(
    State(phrase_service): State<PhraseService>,
    Path(id): Path<String>,
    Json(req): Json<UpdatePhraseRequest>,
) -> Result<StatusCode, AppError> {
    phrase_service
        .update_phrase(
            &id,
            req.key.as_deref(),
            req.source_text.as_deref(),
            req.context.as_deref(),
        )
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn delete_phrase(
    State(phrase_service): State<PhraseService>,
    Path(id): Path<String>,
) -> Result<StatusCode, AppError> {
    phrase_service.delete_phrase(&id).await?;
    Ok(StatusCode::NO_CONTENT)
}
