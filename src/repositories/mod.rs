use sqlx::{MySql, Pool};

pub mod base_repo;
pub mod language_repo;
pub mod module_repo;
pub mod phrase_repo;
pub mod phrase_type_repo;
pub mod project_repo;
pub mod screenshot_repo;
pub mod term_repo;
pub mod translation_repo;
pub mod user_repo;

pub trait Repository {
    fn get_pool(&self) -> &Pool<MySql>;
}

pub use base_repo::BaseRepository;
pub use language_repo::LanguageRepository;
pub use module_repo::ModuleRepository;
pub use phrase_repo::PhraseRepository;
pub use phrase_type_repo::PhraseTypeRepository;
pub use project_repo::ProjectRepository;
pub use screenshot_repo::ScreenshotRepository;
pub use term_repo::TermRepository;
pub use translation_repo::TranslationRepository;
pub use user_repo::UserRepository;
