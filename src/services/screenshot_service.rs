use std::sync::Arc;

use async_trait::async_trait;
use sqlx::MySqlPool;

use crate::{
    dtos::screenshot::{CreateScreenshotDto, UpdateScreenshotDto}, errors::AppError, models::screenshot::Screenshot, repositories::{base_repo::BaseRepository, screenshot_repo::ScreenshotRepository}, services::BaseService
};

pub struct ScreenshotService {
    repo: Arc<ScreenshotRepository>,
}

impl ScreenshotService {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Self {
            repo: Arc::new(ScreenshotRepository::new(pool)),
        }
    }

    pub async fn insert(&self, screenshot: &CreateScreenshotDto) -> Result<u64, AppError> {
        self.repo.insert(&Screenshot::from(screenshot)).await
    }

    pub async fn update_by_id(&self, id: u64, screenshot: &UpdateScreenshotDto) -> Result<bool, AppError> {
        self.repo.update_by_id(id, &Screenshot::from(screenshot)).await
    }
}

#[async_trait]
impl BaseService<Screenshot> for ScreenshotService {
    type Repository = ScreenshotRepository;

    fn get_repository(&self) -> &Self::Repository {
        &self.repo
    }
}
