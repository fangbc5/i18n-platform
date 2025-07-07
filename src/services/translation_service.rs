use crate::{errors::AppError, models::Translation, repositories::TranslationRepository};

pub struct TranslationService {
    repo: TranslationRepository,
}

impl TranslationService {
    pub fn new(repo: TranslationRepository) -> Self {
        Self { repo }
    }

    pub async fn create_translation(
        &mut self,
        phrase_id: &str,
        language: &str,
        content: &str,
        translated_by: Option<&str>,
        user_id: &str,
    ) -> Result<Translation, AppError> {
        let translation = Translation {
            id: uuid::Uuid::new_v4().to_string(),
            phrase_id: phrase_id.to_string(),
            language: language.to_string(),
            content: content.to_string(),
            status: "pending".to_string(),
            translated_by: translated_by.map(|id| id.to_string()),
            reviewed_by: None,
            crt_by: user_id.to_string(),
            crt_at: chrono::Local::now().naive_local(),
            upt_by: None,
            upt_at: chrono::Local::now().naive_local(),
        };

        self.repo.create(&translation)
    }

    pub async fn get_translation(&mut self, translation_id: &str) -> Result<Translation, AppError> {
        self.repo.find_by_id(translation_id)
    }

    pub async fn get_phrase_translations(
        &mut self,
        phrase_id: &str,
    ) -> Result<Vec<Translation>, AppError> {
        self.repo.find_by_phrase(phrase_id)
    }

    pub async fn update_translation(
        &mut self,
        translation_id: &str,
        content: &str,
        status: &str,
        user_id: &str,
    ) -> Result<(), AppError> {
        let mut translation = self.get_translation(translation_id).await?;
        translation.content = content.to_string();
        translation.status = status.to_string();
        translation.upt_by = Some(user_id.to_string());
        translation.upt_at = chrono::Local::now().naive_local();

        self.repo.update(translation_id, &translation)
    }

    pub async fn delete_translation(&mut self, translation_id: &str) -> Result<(), AppError> {
        self.repo.delete(translation_id)
    }

    pub async fn review_translation(
        &mut self,
        translation_id: &str,
        reviewer_id: &str,
        approved: bool,
    ) -> Result<(), AppError> {
        let mut translation = self.get_translation(translation_id).await?;
        translation.status = if approved {
            "approved".to_string()
        } else {
            "rejected".to_string()
        };
        translation.reviewed_by = Some(reviewer_id.to_string());
        translation.upt_by = Some(reviewer_id.to_string());
        translation.upt_at = chrono::Local::now().naive_local();

        self.repo.update(translation_id, &translation)
    }
}
