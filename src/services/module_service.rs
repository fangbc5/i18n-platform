use std::sync::Arc;

use sqlx::MySqlPool;

use crate::{
    dtos::module::{CreateModuleDto, UpdateModuleDto},
    errors::AppError,
    models::module::Module,
    repositories::{module_repo::ModuleRepository, BaseRepository}, services::BaseService,
};

pub struct ModuleService {
    repo: Arc<ModuleRepository>,
}

impl ModuleService {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Self {
            repo: Arc::new(ModuleRepository::new(pool)),
        }
    }

    pub async fn select_by_id(&self, id: u64) -> Result<Option<Module>, AppError> {
        self.repo.select_by_id(id).await
    }

    pub async fn select_all(&self) -> Result<Vec<Module>, AppError> {
        self.repo.select_all().await
    }

    pub async fn select_by_page(
        &self,
        page: u32,
        page_size: u32,
    ) -> Result<(Vec<Module>, u64), AppError> {
        self.repo.select_by_page(page, page_size).await
    }

    pub async fn insert(&self, module: &CreateModuleDto) -> Result<u64, AppError> {
        self.repo.insert(&Module::from(module)).await
    }

    pub async fn update_by_id(&self, id: u64, module: &UpdateModuleDto) -> Result<bool, AppError> {
        self.repo.update_by_id(id, &Module::from(module)).await
    }

    pub async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        self.repo.delete_by_id(id).await
    }

    pub async fn delete_by_ids(&self, ids: &[u64]) -> Result<u64, AppError> {
        self.repo.delete_by_ids(ids).await
    }
}

impl BaseService<Module> for ModuleService {
    type Repository = ModuleRepository;

    fn get_repository(&self) -> &Self::Repository {
        &self.repo
    }
}