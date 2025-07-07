use crate::{errors::AppError, models::Translation};
use diesel::{prelude::*, MysqlConnection};

pub struct TranslationRepository {
    conn: MysqlConnection,
}

impl TranslationRepository {
    pub fn new(conn: MysqlConnection) -> Self {
        Self { conn }
    }

    pub fn find_by_id(&mut self, translation_id: i32) -> Result<Translation, AppError> {
        use crate::schema::translations::dsl::*;
        translations
            .find(translation_id)
            .first(&mut self.conn)
            .map_err(AppError::from)
    }

    pub fn find_by_phrase(&mut self, phrase_id_param: i32) -> Result<Vec<Translation>, AppError> {
        use crate::schema::translations::dsl::*;
        translations
            .filter(phrase_id.eq(phrase_id_param))
            .load(&mut self.conn)
            .map_err(AppError::from)
    }

    pub fn create(&mut self, new_translation: &Translation) -> Result<Translation, AppError> {
        use crate::schema::translations::dsl::*;
        diesel::insert_into(translations)
            .values(new_translation)
            .get_result(&mut self.conn)
            .map_err(AppError::from)
    }

    pub fn update_status(
        &mut self,
        translation_id: i32,
        new_status: &str,
    ) -> Result<Translation, AppError> {
        use crate::schema::translations::dsl::*;
        diesel::update(translations.find(translation_id))
            .set(status.eq(new_status))
            .get_result(&mut self.conn)
            .map_err(AppError::from)
    }
}
