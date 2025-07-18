use async_trait::async_trait;
use std::sync::Arc;
use sqlx::MySqlPool;
use crate::{
    dtos::phrase_type::{CreatePhraseTypeDto, UpdatePhraseTypeDto}, errors::AppError, models::phrase_type::PhraseType, repositories::{phrase_type_repo::PhraseTypeRepository, BaseRepository}, services::BaseService
};

pub struct PhraseTypeService {
    repository: Arc<PhraseTypeRepository>,
}

impl PhraseTypeService {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Self { repository: Arc::new(PhraseTypeRepository::new(pool)) }
    }

    pub async fn insert(&self, phrase_type: &CreatePhraseTypeDto) -> Result<u64, AppError> {
        self.repository.insert(&PhraseType::from(phrase_type)).await
    }

    pub async fn update_by_id(&self, id: u64, phrase_type: &UpdatePhraseTypeDto) -> Result<bool, AppError> {
        self.repository.update_by_id(id, &PhraseType::from(phrase_type)).await
    }
}

#[async_trait]
impl BaseService<PhraseType> for PhraseTypeService {
    type Repository = PhraseTypeRepository;

    fn get_repository(&self) -> &Self::Repository {
        &self.repository
    }
}