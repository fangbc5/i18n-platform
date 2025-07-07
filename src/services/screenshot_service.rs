use diesel::result::Error;

use crate::models::{CreateScreenshot, Screenshot, UpdateScreenshot};
use crate::repositories::{DbPool, ScreenshotRepository};

pub struct ScreenshotService;

impl ScreenshotService {
    pub async fn find_all(pool: &DbPool) -> Result<Vec<Screenshot>, Error> {
        let mut conn = pool.get().unwrap();
        ScreenshotRepository::find_all(&mut conn)
    }

    pub async fn find_by_phrase(pool: &DbPool, phrase_id: &str) -> Result<Vec<Screenshot>, Error> {
        let mut conn = pool.get().unwrap();
        ScreenshotRepository::find_by_phrase(&mut conn, phrase_id)
    }

    pub async fn find_by_id(pool: &DbPool, id: &str) -> Result<Option<Screenshot>, Error> {
        let mut conn = pool.get().unwrap();
        ScreenshotRepository::find_by_id(&mut conn, id)
    }

    pub async fn create(
        pool: &DbPool,
        new_screenshot: &CreateScreenshot,
        user_id: &str,
    ) -> Result<Screenshot, Error> {
        let mut conn = pool.get().unwrap();
        ScreenshotRepository::create(&mut conn, new_screenshot, user_id)
    }

    pub async fn update(
        pool: &DbPool,
        id: &str,
        screenshot: &UpdateScreenshot,
        user_id: &str,
    ) -> Result<Screenshot, Error> {
        let mut conn = pool.get().unwrap();
        ScreenshotRepository::update(&mut conn, id, screenshot, user_id)
    }

    pub async fn delete(pool: &DbPool, id: &str) -> Result<bool, Error> {
        let mut conn = pool.get().unwrap();
        ScreenshotRepository::delete(&mut conn, id)
    }
}
