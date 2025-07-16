use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "ENUM", rename_all = "lowercase")]
pub enum TranslationStatus {
    Pending,
    Reviewed,
    Published,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum UserRole {
    Admin,
    ProjectManager,
    Translator,
    Reviewer,
}

impl ToString for TranslationStatus {
    fn to_string(&self) -> String {
        match self {
            TranslationStatus::Pending => "pending".to_string(),
            TranslationStatus::Reviewed => "reviewed".to_string(),
            TranslationStatus::Published => "published".to_string(),
        }
    }
}

impl ToString for UserRole {
    fn to_string(&self) -> String {
        match self {
            UserRole::Admin => "admin".to_string(),
            UserRole::ProjectManager => "project_manager".to_string(),
            UserRole::Translator => "translator".to_string(),
            UserRole::Reviewer => "reviewer".to_string(),
        }
    }
}
