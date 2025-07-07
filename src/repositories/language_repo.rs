use diesel::prelude::*;
use diesel::result::Error;
use diesel::MysqlConnection;

use crate::models::{CreateLanguage, Language, UpdateLanguage};
use crate::schema::i18n_languages;

pub struct LanguageRepository;

impl LanguageRepository {
    pub fn find_all(conn: &mut MysqlConnection) -> Result<Vec<Language>, Error> {
        i18n_languages::table.load::<Language>(conn)
    }

    pub fn find_by_code(conn: &mut MysqlConnection, code: &str) -> Result<Option<Language>, Error> {
        i18n_languages::table.find(code).first(conn).optional()
    }

    pub fn create(
        conn: &mut MysqlConnection,
        new_language: &CreateLanguage,
        user_id: &str,
    ) -> Result<Language, Error> {
        let language = Language {
            code: new_language.code.clone(),
            name: new_language.name.clone(),
            native_name: new_language.native_name.clone(),
            is_active: new_language.is_active.unwrap_or(true),
            crt_by: user_id.to_string(),
            crt_at: chrono::Local::now().naive_local(),
            upt_by: None,
            upt_at: chrono::Local::now().naive_local(),
        };

        diesel::insert_into(i18n_languages::table)
            .values(&language)
            .execute(conn)?;

        Ok(language)
    }

    pub fn update(
        conn: &mut MysqlConnection,
        code: &str,
        language: &UpdateLanguage,
        user_id: &str,
    ) -> Result<Language, Error> {
        diesel::update(i18n_languages::table.find(code))
            .set((
                language,
                i18n_languages::upt_by.eq(Some(user_id.to_string())),
                i18n_languages::upt_at.eq(chrono::Local::now().naive_local()),
            ))
            .get_result(conn)
    }

    pub fn delete(conn: &mut MysqlConnection, code: &str) -> Result<bool, Error> {
        let count = diesel::delete(i18n_languages::table.find(code)).execute(conn)?;
        Ok(count > 0)
    }
}
