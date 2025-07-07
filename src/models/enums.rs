use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TranslationStatus {
    Draft,
    InReview,
    Approved,
    Rejected,
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
            TranslationStatus::Draft => "draft".to_string(),
            TranslationStatus::InReview => "in_review".to_string(),
            TranslationStatus::Approved => "approved".to_string(),
            TranslationStatus::Rejected => "rejected".to_string(),
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
