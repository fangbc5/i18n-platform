use diesel::result::Error;

use crate::models::{CreateModule, Module, UpdateModule};
use crate::repositories::{DbPool, ModuleRepository};

pub struct ModuleService;

impl ModuleService {
    pub async fn find_all(pool: &DbPool) -> Result<Vec<Module>, Error> {
        let mut conn = pool.get().unwrap();
        ModuleRepository::find_all(&mut conn)
    }

    pub async fn find_by_project(pool: &DbPool, project_id: &str) -> Result<Vec<Module>, Error> {
        let mut conn = pool.get().unwrap();
        ModuleRepository::find_by_project(&mut conn, project_id)
    }

    pub async fn find_by_id(pool: &DbPool, id: &str) -> Result<Option<Module>, Error> {
        let mut conn = pool.get().unwrap();
        ModuleRepository::find_by_id(&mut conn, id)
    }

    pub async fn create(
        pool: &DbPool,
        new_module: &CreateModule,
        user_id: &str,
    ) -> Result<Module, Error> {
        let mut conn = pool.get().unwrap();
        ModuleRepository::create(&mut conn, new_module, user_id)
    }

    pub async fn update(
        pool: &DbPool,
        id: &str,
        module: &UpdateModule,
        user_id: &str,
    ) -> Result<Module, Error> {
        let mut conn = pool.get().unwrap();
        ModuleRepository::update(&mut conn, id, module, user_id)
    }

    pub async fn delete(pool: &DbPool, id: &str) -> Result<bool, Error> {
        let mut conn = pool.get().unwrap();
        ModuleRepository::delete(&mut conn, id)
    }
}
