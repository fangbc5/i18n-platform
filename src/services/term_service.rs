use std::sync::Arc;

use async_trait::async_trait;
use sqlx::MySqlPool;

use crate::{
    dtos::term::{CreateTermDto, UpdateTermDto}, errors::AppError, models::term::Term, repositories::{base_repo::BaseRepository, term_repo::TermRepository}, services::BaseService
};

pub struct TermService {
    repo: Arc<TermRepository>,
}

impl TermService {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Self {
            repo: Arc::new(TermRepository::new(pool)),
        }
    }

    pub async fn insert(&self, term: &CreateTermDto) -> Result<u64, AppError> {
        self.repo.insert(&Term::from(term)).await
    }

    pub async fn update_by_id(&self, id: u64, term: &UpdateTermDto) -> Result<bool, AppError> {
        self.repo.update_by_id(id, &Term::from(term)).await
    }
}

#[async_trait]
impl BaseService<Term> for TermService {
    type Repository = TermRepository;

    fn get_repository(&self) -> &Self::Repository {
        &self.repo
    }
}
