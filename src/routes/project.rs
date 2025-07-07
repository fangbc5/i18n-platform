use crate::{errors::AppError, services::ProjectService};
use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub description: Option<String>,
    pub source_language: String,
    pub target_languages: Vec<String>,
}

#[derive(Serialize)]
pub struct ProjectResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: i32,
}

pub fn project_routes() -> Router {
    Router::new()
        .route("/", post(create_project))
        .route("/:id", get(get_project))
        .route("/user/:id", get(get_user_projects))
}

async fn create_project(
    State(project_service): State<ProjectService>,
    Path(user_id): Path<String>,
    Json(req): Json<CreateProjectRequest>,
) -> Result<Json<ProjectResponse>, AppError> {
    let project = project_service
        .create_project(
            &req.name,
            req.description.as_deref(),
            &req.source_language,
            &req.target_languages,
            &user_id,
        )
        .await?;
    Ok(Json(ProjectResponse {
        id: project.id,
        name: project.name,
        description: project.description,
        owner_id: project.owner_id,
    }))
}

async fn get_project(
    State(project_service): State<ProjectService>,
    Path(id): Path<String>,
) -> Result<Json<ProjectResponse>, AppError> {
    let project = project_service.get_project(&id).await?;
    Ok(Json(ProjectResponse {
        id: project.id,
        name: project.name,
        description: project.description,
        owner_id: project.owner_id,
    }))
}

async fn get_user_projects(
    State(project_service): State<ProjectService>,
    Path(user_id): Path<String>,
) -> Result<Json<Vec<ProjectResponse>>, AppError> {
    let projects = project_service.get_user_projects(&user_id).await?;
    Ok(Json(
        projects
            .into_iter()
            .map(|p| ProjectResponse {
                id: p.id,
                name: p.name,
                description: p.description,
                owner_id: p.owner_id,
            })
            .collect::<Vec<_>>(),
    ))
}
