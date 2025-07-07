use crate::{errors::AppError, models::Project, repositories::ProjectRepository};

pub struct ProjectService {
    repo: ProjectRepository,
}

impl ProjectService {
    pub fn new(repo: ProjectRepository) -> Self {
        Self { repo }
    }

    pub async fn create_project(
        &mut self,
        name: &str,
        description: Option<&str>,
        source_language: &str,
        target_languages: Vec<&str>,
        owner_id: &str,
    ) -> Result<Project, AppError> {
        let project = Project {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            source_language: source_language.to_string(),
            target_languages: serde_json::to_string(&target_languages)?,
            owner_id: owner_id.to_string(),
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        };

        self.repo.create(&project)
    }

    pub async fn get_project(&mut self, project_id: &str) -> Result<Project, AppError> {
        self.repo.find_by_id(project_id)
    }

    pub async fn get_user_projects(&mut self, user_id: &str) -> Result<Vec<Project>, AppError> {
        self.repo.find_by_owner(user_id)
    }
}
