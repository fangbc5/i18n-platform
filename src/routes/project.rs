use crate::{
    dtos::project::{CreateProjectDto, UpdateProjectDto},
    errors::AppError,
    middleware::auth::Authentication,
    services::{project_service::ProjectService, BaseService},
};
use actix_web::{delete, get, post, put, web, HttpResponse};

pub fn project_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .wrap(Authentication::default())
            .service(create_project)
            .service(get_projects)
            .service(get_project)
            .service(update_project)
            .service(delete_project),
    );
}

#[post("")]
async fn create_project(
    project_service: web::Data<ProjectService>,
    project: web::Json<CreateProjectDto>,
) -> Result<HttpResponse, AppError> {
    let project = project_service.insert(&project.into_inner()).await?;
    Ok(HttpResponse::Created().json(project))
}

#[get("")]
async fn get_projects(
    project_service: web::Data<ProjectService>,
) -> Result<HttpResponse, AppError> {
    let projects = project_service.select_all().await?;
    Ok(HttpResponse::Ok().json(projects))
}

#[get("/{id}")]
async fn get_project(
    project_service: web::Data<ProjectService>,
    id: web::Path<u64>,
) -> Result<HttpResponse, AppError> {
    let project = project_service.select_by_id(id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(project))
}

#[put("/{id}")]
async fn update_project(
    project_service: web::Data<ProjectService>,
    id: web::Path<u64>,
    project: web::Json<UpdateProjectDto>,
) -> Result<HttpResponse, AppError> {
    let project = project_service.update_by_id(id.into_inner(), &project.into_inner()).await?;
    Ok(HttpResponse::Ok().json(project))
}

#[delete("/{id}")]
async fn delete_project(
    project_service: web::Data<ProjectService>,
    id: web::Path<u64>,
) -> Result<HttpResponse, AppError> {
    project_service.delete_by_id(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
