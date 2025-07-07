use crate::{errors::AppError, models::Phrase, repositories::PhraseRepository};

pub struct PhraseService {
    repo: PhraseRepository,
}

impl PhraseService {
    pub fn new(repo: PhraseRepository) -> Self {
        Self { repo }
    }

    pub async fn create_phrase(
        &mut self,
        project_id: &str,
        key: &str,
        source_text: &str,
        context: Option<&str>,
    ) -> Result<Phrase, AppError> {
        let phrase = Phrase {
            id: uuid::Uuid::new_v4().to_string(),
            project_id: project_id.to_string(),
            key: key.to_string(),
            source_text: source_text.to_string(),
            context: context.map(|s| s.to_string()),
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        };

        self.repo.create(&phrase)
    }

    pub async fn get_phrase(&mut self, phrase_id: &str) -> Result<Phrase, AppError> {
        self.repo.find_by_id(phrase_id)
    }

    pub async fn get_project_phrases(&mut self, project_id: &str) -> Result<Vec<Phrase>, AppError> {
        self.repo.find_by_project(project_id)
    }

    pub async fn update_phrase(
        &mut self,
        phrase_id: &str,
        key: Option<&str>,
        source_text: Option<&str>,
        context: Option<&str>,
    ) -> Result<(), AppError> {
        let mut phrase = self.get_phrase(phrase_id).await?;

        if let Some(k) = key {
            phrase.key = k.to_string();
        }
        if let Some(st) = source_text {
            phrase.source_text = st.to_string();
        }
        if let Some(ctx) = context {
            phrase.context = Some(ctx.to_string());
        }
        phrase.updated_at = chrono::Local::now().naive_local();

        self.repo.update(phrase_id, &phrase)
    }

    pub async fn delete_phrase(&mut self, phrase_id: &str) -> Result<(), AppError> {
        self.repo.delete(phrase_id)
    }
}
