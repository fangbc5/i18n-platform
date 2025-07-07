use diesel::result::Error;

use crate::models::{CreateLanguage, Language, UpdateLanguage};
use crate::repositories::{DbPool, LanguageRepository};

pub struct LanguageService;

impl LanguageService {
    pub async fn find_all(pool: &DbPool) -> Result<Vec<Language>, Error> {
        let mut conn = pool.get().unwrap();
        LanguageRepository::find_all(&mut conn)
    }

    pub async fn find_by_code(pool: &DbPool, code: &str) -> Result<Option<Language>, Error> {
        let mut conn = pool.get().unwrap();
        LanguageRepository::find_by_code(&mut conn, code)
    }

    pub async fn create(
        pool: &DbPool,
        new_language: &CreateLanguage,
        user_id: &str,
    ) -> Result<Language, Error> {
        let mut conn = pool.get().unwrap();
        LanguageRepository::create(&mut conn, new_language, user_id)
    }

    pub async fn update(
        pool: &DbPool,
        code: &str,
        language: &UpdateLanguage,
        user_id: &str,
    ) -> Result<Language, Error> {
        let mut conn = pool.get().unwrap();
        LanguageRepository::update(&mut conn, code, language, user_id)
    }

    pub async fn delete(pool: &DbPool, code: &str) -> Result<bool, Error> {
        let mut conn = pool.get().unwrap();
        LanguageRepository::delete(&mut conn, code)
    }
}
