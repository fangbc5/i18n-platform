use diesel::prelude::*;
use diesel::result::Error;
use diesel::MysqlConnection;
use uuid::Uuid;

use crate::models::{CreateModule, Module, UpdateModule};
use crate::schema::i18n_modules;

pub struct ModuleRepository;

impl ModuleRepository {
    pub fn find_all(conn: &mut MysqlConnection) -> Result<Vec<Module>, Error> {
        i18n_modules::table.load::<Module>(conn)
    }

    pub fn find_by_project(
        conn: &mut MysqlConnection,
        project_id: &str,
    ) -> Result<Vec<Module>, Error> {
        i18n_modules::table
            .filter(i18n_modules::project_id.eq(project_id))
            .load::<Module>(conn)
    }

    pub fn find_by_id(conn: &mut MysqlConnection, id: &str) -> Result<Option<Module>, Error> {
        i18n_modules::table.find(id).first(conn).optional()
    }

    pub fn create(
        conn: &mut MysqlConnection,
        new_module: &CreateModule,
        user_id: &str,
    ) -> Result<Module, Error> {
        let module = Module {
            id: Uuid::new_v4().to_string(),
            project_id: new_module.project_id.clone(),
            name: new_module.name.clone(),
            description: new_module.description.clone(),
            path: new_module.path.clone(),
            crt_by: user_id.to_string(),
            crt_at: chrono::Local::now().naive_local(),
            upt_by: None,
            upt_at: chrono::Local::now().naive_local(),
        };

        diesel::insert_into(i18n_modules::table)
            .values(&module)
            .execute(conn)?;

        Ok(module)
    }

    pub fn update(
        conn: &mut MysqlConnection,
        id: &str,
        module: &UpdateModule,
        user_id: &str,
    ) -> Result<Module, Error> {
        diesel::update(i18n_modules::table.find(id))
            .set((
                module,
                i18n_modules::upt_by.eq(Some(user_id.to_string())),
                i18n_modules::upt_at.eq(chrono::Local::now().naive_local()),
            ))
            .get_result(conn)
    }

    pub fn delete(conn: &mut MysqlConnection, id: &str) -> Result<bool, Error> {
        let count = diesel::delete(i18n_modules::table.find(id)).execute(conn)?;
        Ok(count > 0)
    }
}
