use diesel::prelude::*;
use diesel::result::Error;
use diesel::MysqlConnection;
use uuid::Uuid;

use crate::models::{CreatePhraseType, PhraseType, UpdatePhraseType};
use crate::schema::i18n_phrase_types;

pub struct PhraseTypeRepository;

impl PhraseTypeRepository {
    pub fn find_all(conn: &mut MysqlConnection) -> Result<Vec<PhraseType>, Error> {
        i18n_phrase_types::table.load::<PhraseType>(conn)
    }

    pub fn find_by_id(conn: &mut MysqlConnection, id: &str) -> Result<Option<PhraseType>, Error> {
        i18n_phrase_types::table.find(id).first(conn).optional()
    }

    pub fn create(
        conn: &mut MysqlConnection,
        new_type: &CreatePhraseType,
        user_id: &str,
    ) -> Result<PhraseType, Error> {
        let phrase_type = PhraseType {
            id: Uuid::new_v4().to_string(),
            name: new_type.name.clone(),
            description: new_type.description.clone(),
            icon: new_type.icon.clone(),
            crt_by: user_id.to_string(),
            crt_at: chrono::Local::now().naive_local(),
            upt_by: None,
            upt_at: chrono::Local::now().naive_local(),
        };

        diesel::insert_into(i18n_phrase_types::table)
            .values(&phrase_type)
            .execute(conn)?;

        Ok(phrase_type)
    }

    pub fn update(
        conn: &mut MysqlConnection,
        id: &str,
        phrase_type: &UpdatePhraseType,
        user_id: &str,
    ) -> Result<PhraseType, Error> {
        diesel::update(i18n_phrase_types::table.find(id))
            .set((
                phrase_type,
                i18n_phrase_types::upt_by.eq(Some(user_id.to_string())),
                i18n_phrase_types::upt_at.eq(chrono::Local::now().naive_local()),
            ))
            .get_result(conn)
    }

    pub fn delete(conn: &mut MysqlConnection, id: &str) -> Result<bool, Error> {
        let count = diesel::delete(i18n_phrase_types::table.find(id)).execute(conn)?;
        Ok(count > 0)
    }
}
