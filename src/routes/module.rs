use crate::{
    dtos::module::{CreateModuleDto, ModuleQuery, UpdateModuleDto},
    errors::AppError,
    middleware::auth::Authentication,
    services::module_service::ModuleService,
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

pub fn module_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .wrap(Authentication::default())
            .service(create_module)
            .service(get_modules)
            .service(get_module)
            .service(update_module)
            .service(delete_module),
    );
}

#[post("")]
async fn create_module(
    module_service: web::Data<ModuleService>,
    module: web::Json<CreateModuleDto>,
) -> Result<impl Responder, AppError> {
    let module = module_service.insert(&module.into_inner()).await?;
    Ok(HttpResponse::Created().json(module))
}

#[get("")]
async fn get_modules(
    module_service: web::Data<ModuleService>,
    query: web::Query<ModuleQuery>,
) -> Result<impl Responder, AppError> {
    let modules = module_service.select_by_page(query.page, query.size).await?;
    Ok(HttpResponse::Ok().json(modules))
}

#[get("/{id}")]
async fn get_module(
    module_service: web::Data<ModuleService>,
    id: web::Path<u64>,
) -> Result<impl Responder, AppError> {
    let module = module_service.select_by_id(id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(module))
}

#[put("/{id}")]
async fn update_module(
    module_service: web::Data<ModuleService>,
    id: web::Path<u64>,
    module: web::Json<UpdateModuleDto>,
) -> Result<impl Responder, AppError> {

    let module = module_service.update_by_id(id.into_inner(), &module.into_inner()).await?;
    Ok(HttpResponse::Ok().json(module))
}

#[delete("/{id}")]
async fn delete_module(
    module_service: web::Data<ModuleService>,
    id: web::Path<u64>,
) -> Result<impl Responder, AppError> {
    module_service.delete_by_id(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
