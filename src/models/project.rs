use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::i18n_projects;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = i18n_projects)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub base_language: String,
    pub owner_id: String,
    pub status: bool,
    pub crt_by: String,
    pub crt_at: NaiveDateTime,
    pub upt_by: Option<String>,
    pub upt_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateProject {
    pub name: String,
    pub description: Option<String>,
    pub base_language: String,
    pub owner_id: String,
    pub status: Option<bool>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = i18n_projects)]
pub struct UpdateProject {
    pub name: Option<String>,
    pub description: Option<String>,
    pub base_language: Option<String>,
    pub owner_id: Option<String>,
    pub status: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = i18n_project_languages)]
pub struct ProjectLanguage {
    pub project_id: String,
    pub language: String,
    pub is_default: bool,
}

#[derive(Debug, Deserialize)]
pub struct AddProjectLanguage {
    pub language: String,
    pub is_default: Option<bool>,
}
