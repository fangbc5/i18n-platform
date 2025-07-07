use crate::{errors::AppError, models::Phrase, schema::phrases};
use diesel::{prelude::*, MysqlConnection};

pub struct PhraseRepository {
    conn: MysqlConnection,
}

impl PhraseRepository {
    pub fn new(conn: MysqlConnection) -> Self {
        Self { conn }
    }

    pub fn find_by_id(&mut self, phrase_id: &str) -> Result<Phrase, AppError> {
        use crate::schema::phrases::dsl::*;
        phrases
            .find(phrase_id)
            .first(&mut self.conn)
            .map_err(AppError::Database)
    }

    pub fn find_by_project(&mut self, project_id_param: &str) -> Result<Vec<Phrase>, AppError> {
        use crate::schema::phrases::dsl::*;
        phrases
            .filter(project_id.eq(project_id_param))
            .load::<Phrase>(&mut self.conn)
            .map_err(AppError::Database)
    }

    pub fn create(&mut self, phrase: &Phrase) -> Result<Phrase, AppError> {
        use crate::schema::phrases::dsl::*;
        diesel::insert_into(phrases)
            .values(phrase)
            .execute(&mut self.conn)
            .map_err(AppError::Database)?;

        Ok(phrase.clone())
    }

    pub fn update(&mut self, phrase_id_param: &str, phrase: &Phrase) -> Result<(), AppError> {
        use crate::schema::phrases::dsl::*;
        diesel::update(phrases.find(phrase_id_param))
            .set(phrase)
            .execute(&mut self.conn)
            .map_err(AppError::Database)?;

        Ok(())
    }

    pub fn delete(&mut self, phrase_id_param: &str) -> Result<(), AppError> {
        use crate::schema::phrases::dsl::*;
        diesel::delete(phrases.find(phrase_id_param))
            .execute(&mut self.conn)
            .map_err(AppError::Database)?;

        Ok(())
    }
}
