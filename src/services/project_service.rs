use std::sync::Arc;

use async_trait::async_trait;
use sqlx::MySqlPool;

use crate::dtos::common::PageRequest;
use crate::repositories::UserRepository;
use crate::{
    dtos::project::{CreateProjectDto, ProjectVo, UpdateProjectDto},
    errors::AppError,
    models::project::Project,
    repositories::{base_repo::BaseRepository, project_repo::ProjectRepository},
    services::BaseService,
};

pub struct ProjectService {
    repo: Arc<ProjectRepository>,
    user_repo: Arc<UserRepository>,
}

impl ProjectService {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Self {
            repo: Arc::new(ProjectRepository::new(pool.clone())),
            user_repo: Arc::new(UserRepository::new(pool)),
        }
    }

    pub async fn select_by_page(
        &self,
        req: &PageRequest,
    ) -> Result<(Vec<ProjectVo>, i64), AppError> {
        if let Some(search_key) = &req.search_key {
            let projects = self
                .repo
                .select_page_by_key(req.page, req.size, search_key)
                .await?;
            let mut list = Vec::with_capacity(projects.0.len());
            for project in &projects.0 {
                let mut vo = ProjectVo::from(project);
                if let Some(owner) = self.user_repo.select_by_id(project.owner_id).await? {
                    if let Some(username) = owner.username {
                        vo.owner = username;
                    }
                }
                list.push(vo);
            }
            Ok((list, projects.1))
        } else {
            let projects = self.repo.select_by_page(req.page, req.size).await?;
            let mut list = Vec::with_capacity(projects.0.len());
            for project in &projects.0 {
                let mut vo = ProjectVo::from(project);
                if let Some(owner) = self.user_repo.select_by_id(project.owner_id).await? {
                    if let Some(username) = owner.username {
                        vo.owner = username;
                    }
                }
                list.push(vo);
            }
            Ok((list, projects.1))
        }
    }

    pub async fn select_by_id(&self, id: u64) -> Result<ProjectVo, AppError> {
        let project = self.repo.select_by_id(id).await?;
        if let Some(project) = project {
            let mut vo = ProjectVo::from(&project);
            if let Some(owner) = self.user_repo.select_by_id(project.owner_id).await? {
                if let Some(username) = owner.username {
                    vo.owner = username;
                }
            }
            Ok(vo)
        } else {
            Err(AppError::NotFound("project not found".into()))
        }
    }

    pub async fn insert(&self, project: &CreateProjectDto) -> Result<u64, AppError> {
        // 判断名称和编码唯一性
        if let Some(_) = self.repo.find_by_name(&project.name, 0).await? {
            return Err(AppError::BusinessError("repeat project name".into()));
        }
        if let Some(_) = self.repo.find_by_code(&project.code, 0).await? {
            return Err(AppError::BusinessError("repeat project code".into()));
        }
        self.repo.insert(&Project::from(project)).await
    }

    pub async fn update_by_id(
        &self,
        id: u64,
        project: &UpdateProjectDto,
    ) -> Result<bool, AppError> {
        // 判断名称和编码唯一性
        if let Some(code) = &project.code {
            if let Some(_) = self.repo.find_by_code(code, id).await? {
                return Err(AppError::BusinessError("repeat project code".into()));
            }
        }
        if let Some(name) = &project.name {
            if let Some(_) = self.repo.find_by_name(name, id).await? {
                return Err(AppError::BusinessError("repeat project name".into()));
            }
        }
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
