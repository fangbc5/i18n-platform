mod auth;
mod language;
mod module;
mod phrase;
mod phrase_type;
mod project;
mod screenshot;
mod term;
mod translation;
mod user;

use crate::repositories::DbPool;
use axum::Router;

pub fn api_routes() -> Router<DbPool> {
    Router::new()
        .merge(auth::auth_routes())
        .merge(language::language_routes())
        .merge(module::module_routes())
        .merge(phrase::phrase_routes())
        .merge(phrase_type::phrase_type_routes())
        .merge(project::project_routes())
        .merge(screenshot::screenshot_routes())
        .merge(term::term_routes())
        .merge(translation::translation_routes())
        .merge(user::user_routes())
}
