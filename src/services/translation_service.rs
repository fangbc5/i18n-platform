use std::sync::Arc;

use async_trait::async_trait;
use sqlx::MySqlPool;

use crate::{
    dtos::translation::{CreateTranslationDto, UpdateTranslationDto}, errors::AppError, models::translation::Translation, repositories::{base_repo::BaseRepository, translation_repo::TranslationRepository}, services::BaseService
};

pub struct TranslationService {
    repo: Arc<TranslationRepository>,
}

impl TranslationService {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Self {
            repo: Arc::new(TranslationRepository::new(pool)),
        }
    }

    pub async fn insert(&self, translation: &CreateTranslationDto) -> Result<u64, AppError> {
        self.repo.insert(&Translation::from(translation)).await
    }

    pub async fn update_by_id(&self, id: u64, translation: &UpdateTranslationDto) -> Result<bool, AppError> {
        self.repo.update_by_id(id, &Translation::from(translation)).await
    }
}

#[async_trait]
impl BaseService<Translation> for TranslationService {
    type Repository = TranslationRepository;

    fn get_repository(&self) -> &Self::Repository {
        &self.repo
    }
}
