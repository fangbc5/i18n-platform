use diesel::prelude::*;
use diesel::result::Error;
use diesel::MysqlConnection;
use uuid::Uuid;

use crate::models::{CreateScreenshot, Screenshot, UpdateScreenshot};
use crate::schema::i18n_phrase_screenshots;

pub struct ScreenshotRepository;

impl ScreenshotRepository {
    pub fn find_all(conn: &mut MysqlConnection) -> Result<Vec<Screenshot>, Error> {
        i18n_phrase_screenshots::table.load::<Screenshot>(conn)
    }

    pub fn find_by_phrase(
        conn: &mut MysqlConnection,
        phrase_id: &str,
    ) -> Result<Vec<Screenshot>, Error> {
        i18n_phrase_screenshots::table
            .filter(i18n_phrase_screenshots::phrase_id.eq(phrase_id))
            .load::<Screenshot>(conn)
    }

    pub fn find_by_id(conn: &mut MysqlConnection, id: &str) -> Result<Option<Screenshot>, Error> {
        i18n_phrase_screenshots::table
            .find(id)
            .first(conn)
            .optional()
    }

    pub fn create(
        conn: &mut MysqlConnection,
        new_screenshot: &CreateScreenshot,
        user_id: &str,
    ) -> Result<Screenshot, Error> {
        let screenshot = Screenshot {
            id: Uuid::new_v4().to_string(),
            phrase_id: new_screenshot.phrase_id.clone(),
            image_url: new_screenshot.image_url.clone(),
            description: new_screenshot.description.clone(),
            crt_by: user_id.to_string(),
            crt_at: chrono::Local::now().naive_local(),
            upt_by: None,
            upt_at: chrono::Local::now().naive_local(),
        };

        diesel::insert_into(i18n_phrase_screenshots::table)
            .values(&screenshot)
            .execute(conn)?;

        Ok(screenshot)
    }

    pub fn update(
        conn: &mut MysqlConnection,
        id: &str,
        screenshot: &UpdateScreenshot,
        user_id: &str,
    ) -> Result<Screenshot, Error> {
        diesel::update(i18n_phrase_screenshots::table.find(id))
            .set((
                screenshot,
                i18n_phrase_screenshots::upt_by.eq(Some(user_id.to_string())),
                i18n_phrase_screenshots::upt_at.eq(chrono::Local::now().naive_local()),
            ))
            .get_result(conn)
    }

    pub fn delete(conn: &mut MysqlConnection, id: &str) -> Result<bool, Error> {
        let count = diesel::delete(i18n_phrase_screenshots::table.find(id)).execute(conn)?;
        Ok(count > 0)
    }
}
