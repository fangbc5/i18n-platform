use crate::{config::SETTINGS, errors::AppError};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    MysqlConnection,
};

mod language_repo;
mod module_repo;
mod phrase_repo;
mod phrase_type_repo;
mod project_repo;
mod screenshot_repo;
mod term_repo;
mod translation_repo;
mod user_repo;

pub use language_repo::LanguageRepository;
pub use module_repo::ModuleRepository;
pub use phrase_repo::PhraseRepository;
pub use phrase_type_repo::PhraseTypeRepository;
pub use project_repo::ProjectRepository;
pub use screenshot_repo::ScreenshotRepository;
pub use term_repo::TermRepository;
pub use translation_repo::TranslationRepository;
pub use user_repo::UserRepository;

pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;

pub fn init_pool() -> Result<DbPool, AppError> {
    let manager = ConnectionManager::<MysqlConnection>::new(&SETTINGS.database.url);
    Pool::builder()
        .max_size(SETTINGS.database.pool_size)
        .build(manager)
        .map_err(|e| AppError::Database(e.into()))
}
