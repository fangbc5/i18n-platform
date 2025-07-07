use crate::{errors::AppError, services::TermService};
use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateTermRequest {
    pub project_id: String,
    pub source_term: String,
    pub definition: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTermRequest {
    pub source_term: Option<String>,
    pub definition: Option<String>,
}

pub fn term_routes() -> Router {
    Router::new()
        .route("/", post(create_term))
        .route("/:id", get(get_term))
        .route("/project/:id", get(get_project_terms))
        .route("/:id", post(update_term))
        .route("/:id", delete(delete_term))
}

async fn create_term(
    State(term_service): State<TermService>,
    Json(req): Json<CreateTermRequest>,
) -> Result<Json<Term>, AppError> {
    let term = term_service
        .create_term(&req.project_id, &req.source_term, &req.definition)
        .await?;
    Ok(Json(term))
}

async fn get_term(
    State(term_service): State<TermService>,
    Path(id): Path<String>,
) -> Result<Json<Term>, AppError> {
    let term = term_service.get_term(&id).await?;
    Ok(Json(term))
}

async fn get_project_terms(
    State(term_service): State<TermService>,
    Path(project_id): Path<String>,
) -> Result<Json<Vec<Term>>, AppError> {
    let terms = term_service.get_project_terms(&project_id).await?;
    Ok(Json(terms))
}

async fn update_term(
    State(term_service): State<TermService>,
    Path(id): Path<String>,
    Json(req): Json<UpdateTermRequest>,
) -> Result<StatusCode, AppError> {
    term_service
        .update_term(&id, req.source_term.as_deref(), req.definition.as_deref())
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn delete_term(
    State(term_service): State<TermService>,
    Path(id): Path<String>,
) -> Result<StatusCode, AppError> {
    term_service.delete_term(&id).await?;
    Ok(StatusCode::NO_CONTENT)
}
