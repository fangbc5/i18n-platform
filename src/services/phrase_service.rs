use std::sync::Arc;

use async_trait::async_trait;
use sqlx::MySqlPool;

use crate::{
    dtos::phrase::{CreatePhraseDto, UpdatePhraseDto}, errors::AppError, models::phrase::Phrase, repositories::{base_repo::BaseRepository, phrase_repo::PhraseRepository}, services::BaseService
};

pub struct PhraseService {
    repo: Arc<PhraseRepository>,
}

impl PhraseService {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Self {
            repo: Arc::new(PhraseRepository::new(pool)),
        }
    }

    pub async fn insert(&self, phrase: &CreatePhraseDto) -> Result<u64, AppError> {
        self.repo.insert(&Phrase::from(phrase)).await
    }

    pub async fn update_by_id(&self, id: u64, phrase: &UpdatePhraseDto) -> Result<bool, AppError> {
        self.repo.update_by_id(id, &Phrase::from(phrase)).await
    }
}

#[async_trait]
impl BaseService<Phrase> for PhraseService {
    type Repository = PhraseRepository;

    fn get_repository(&self) -> &Self::Repository {
        &self.repo
    }
}