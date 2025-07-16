use std::sync::Arc;

use async_trait::async_trait;
use sqlx::MySqlPool;

use crate::{
    dtos::project::{CreateProjectDto, ProjectVo, UpdateProjectDto},
    errors::AppError,
    models::project::Project,
    repositories::{base_repo::BaseRepository, project_repo::ProjectRepository}, services::BaseService,
};

pub struct ProjectService {
    repo: Arc<ProjectRepository>,
}

impl ProjectService {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Self {
            repo: Arc::new(ProjectRepository::new(pool)),
        }
    }

    pub async fn insert(&self, project: &CreateProjectDto) -> Result<u64, AppError> {
        self.repo.insert(&Project::from(project)).await
    }

    pub async fn update_by_id(&self, id: u64, project: &UpdateProjectDto) -> Result<bool, AppError> {
        self.repo.update_by_id(id, &Project::from(project)).await
    }
}

#[async_trait]
impl BaseService<Project> for ProjectService {
    type Repository = ProjectRepository;

    fn get_repository(&self) -> &Self::Repository {
        &self.repo
    }
}