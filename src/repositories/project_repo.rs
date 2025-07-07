use crate::{errors::AppError, models::Project, schema::projects};
use diesel::{prelude::*, MysqlConnection};

pub struct ProjectRepository {
    conn: MysqlConnection,
}

impl ProjectRepository {
    pub fn new(conn: MysqlConnection) -> Self {
        Self { conn }
    }

    pub fn find_by_id(&mut self, project_id: &str) -> Result<Project, AppError> {
        use crate::schema::projects::dsl::*;
        projects
            .find(project_id)
            .first(&mut self.conn)
            .map_err(AppError::Database)
    }

    pub fn find_by_owner(&mut self, owner_user_id: &str) -> Result<Vec<Project>, AppError> {
        use crate::schema::projects::dsl::*;
        projects
            .filter(owner_id.eq(owner_user_id))
            .load::<Project>(&mut self.conn)
            .map_err(AppError::Database)
    }

    pub fn create(&mut self, project: &Project) -> Result<Project, AppError> {
        use crate::schema::projects::dsl::*;
        diesel::insert_into(projects)
            .values(project)
            .execute(&mut self.conn)
            .map_err(AppError::Database)?;

        Ok(project.clone())
    }
}
