use crate::{errors::AppError, models::Term, repositories::TermRepository};

pub struct TermService {
    repo: TermRepository,
}

impl TermService {
    pub fn new(repo: TermRepository) -> Self {
        Self { repo }
    }

    pub async fn create_term(
        &mut self,
        project_id: &str,
        source_term: &str,
        definition: &str,
    ) -> Result<Term, AppError> {
        let term = Term {
            id: uuid::Uuid::new_v4().to_string(),
            project_id: project_id.to_string(),
            source_term: source_term.to_string(),
            definition: definition.to_string(),
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        };

        self.repo.create(&term)
    }

    pub async fn get_term(&mut self, term_id: &str) -> Result<Term, AppError> {
        self.repo.find_by_id(term_id)
    }

    pub async fn get_project_terms(&mut self, project_id: &str) -> Result<Vec<Term>, AppError> {
        self.repo.find_by_project(project_id)
    }

    pub async fn update_term(
        &mut self,
        term_id: &str,
        source_term: Option<&str>,
        definition: Option<&str>,
    ) -> Result<(), AppError> {
        let mut term = self.get_term(term_id).await?;

        if let Some(st) = source_term {
            term.source_term = st.to_string();
        }
        if let Some(def) = definition {
            term.definition = def.to_string();
        }
        term.updated_at = chrono::Local::now().naive_local();

        self.repo.update(term_id, &term)
    }

    pub async fn delete_term(&mut self, term_id: &str) -> Result<(), AppError> {
        self.repo.delete(term_id)
    }
}
