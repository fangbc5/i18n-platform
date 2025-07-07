use crate::{errors::AppError, models::Term, schema::terms};
use diesel::{prelude::*, MysqlConnection};

pub struct TermRepository {
    conn: MysqlConnection,
}

impl TermRepository {
    pub fn new(conn: MysqlConnection) -> Self {
        Self { conn }
    }

    pub fn find_by_id(&mut self, term_id: &str) -> Result<Term, AppError> {
        use crate::schema::terms::dsl::*;
        terms
            .find(term_id)
            .first(&mut self.conn)
            .map_err(AppError::Database)
    }

    pub fn find_by_project(&mut self, project_id_param: &str) -> Result<Vec<Term>, AppError> {
        use crate::schema::terms::dsl::*;
        terms
            .filter(project_id.eq(project_id_param))
            .load::<Term>(&mut self.conn)
            .map_err(AppError::Database)
    }

    pub fn create(&mut self, term: &Term) -> Result<Term, AppError> {
        use crate::schema::terms::dsl::*;
        diesel::insert_into(terms)
            .values(term)
            .execute(&mut self.conn)
            .map_err(AppError::Database)?;

        Ok(term.clone())
    }

    pub fn update(&mut self, term_id_param: &str, term: &Term) -> Result<(), AppError> {
        use crate::schema::terms::dsl::*;
        diesel::update(terms.find(term_id_param))
            .set(term)
            .execute(&mut self.conn)
            .map_err(AppError::Database)?;

        Ok(())
    }

    pub fn delete(&mut self, term_id_param: &str) -> Result<(), AppError> {
        use crate::schema::terms::dsl::*;
        diesel::delete(terms.find(term_id_param))
            .execute(&mut self.conn)
            .map_err(AppError::Database)?;

        Ok(())
    }
}
