use crate::{
    dtos::project::{CreateProjectDto, UpdateProjectDto},
    errors::AppError,
    middleware::auth::Authentication,
    services::{project_service::ProjectService, BaseService},
};
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use crate::dtos::common::PageRequest;
use crate::dtos::project::ProjectVo;
use crate::utils::{jwt, PageR, R};

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

#[get("/list")]
async fn get_projects(
    project_service: web::Data<ProjectService>,
    req: web::Query<PageRequest>
) -> Result<HttpResponse, AppError> {
    let projects = project_service.select_by_page(&req).await?;
    Ok(PageR::ok(projects))
}

#[get("/detail/{id}")]
async fn get_project(
    project_service: web::Data<ProjectService>,
    id: web::Path<u64>,
) -> Result<HttpResponse, AppError> {
    let project = project_service.select_by_id(id.into_inner()).await?;
    Ok(R::ok(project))
}

#[post("")]
async fn create_project(
    project_service: web::Data<ProjectService>,
    mut project: web::Json<CreateProjectDto>,
    http_request: HttpRequest
) -> Result<HttpResponse, AppError> {
    let claims = jwt::get_claims(&http_request)?;
    project.crt_by = claims.username;
    let project = project_service.insert(&project.into_inner()).await?;
    Ok(R::ok(project))
}

#[put("/{id}")]
async fn update_project(
    project_service: web::Data<ProjectService>,
    id: web::Path<u64>,
    mut project: web::Json<UpdateProjectDto>,
    http_request: HttpRequest
) -> Result<HttpResponse, AppError> {
    let claims = jwt::get_claims(&http_request)?;
    project.upt_by = claims.username;
    let project = project_service.update_by_id(id.into_inner(), &project.into_inner()).await?;
    Ok(R::ok(project))
}

#[delete("/{id}")]
async fn delete_project(
    project_service: web::Data<ProjectService>,
    id: web::Path<u64>,
) -> Result<HttpResponse, AppError> {
    let result = project_service.delete_by_id(id.into_inner()).await?;
    Ok(R::ok(result))
}
