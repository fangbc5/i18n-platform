use chrono::Utc;
use sqlx::MySqlPool;
use std::sync::Arc;

use crate::dtos::common::PageRequest;
use crate::dtos::language::{CreateLanguageDto, UpdateLanguageDto};
use crate::{
    dtos::language::LanguageVo,
    errors::AppError,
    models::language::Language,
    repositories::{base_repo::BaseRepository, language_repo::LanguageRepository},
    services::BaseService,
};

pub struct LanguageService {
    repo: Arc<LanguageRepository>,
}

impl LanguageService {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Self {
            repo: Arc::new(LanguageRepository::new(pool)),
        }
    }

    /// 分页查询语言列表
    pub async fn select_by_page(&self, req: &PageRequest) -> Result<(Vec<LanguageVo>, i64), AppError> {
        if let Some(search_key) = &req.search_key {
            let languages = self
                .repo
                .select_page_by_key(req.page, req.size, search_key)
                .await?;
            let list = languages.0.iter().map(|language| LanguageVo::from(language)).collect();
            Ok((list, languages.1))
        } else {
            let languages = self.repo.select_by_page(req.page, req.size).await?;
            let list = languages.0.iter().map(|language| LanguageVo::from(language)).collect();
            Ok((list, languages.1))
        }
    }
    
    /// 插入语言
    pub async fn insert(&self, language: &CreateLanguageDto) -> Result<u64, AppError> {
        let code = language.code.clone();
        let name = language.name.clone();
        if let Some(_) = self.repo.find_by_code(&code,0).await? {
            return Err(AppError::BusinessError("repeat language code".into()));
        }
        if let Some(_) = self.repo.find_by_name(&name,0).await? {
            return Err(AppError::BusinessError("repeat language name".into()));
        }
        let entity = Language {
            id: 0,
            code,
            name,
            is_active: language.is_active.unwrap_or(false),
            is_native: language.is_native.unwrap_or(false),
            crt_by: language.crt_by.clone().unwrap_or("admin".to_owned()),
            crt_at: Utc::now(),
            upt_by: None,
            upt_at: Utc::now()
        };
        self.repo.insert(&entity).await
    }
    
    /// 更新语言
    pub async fn update_by_id(&self, id: u64, language: &UpdateLanguageDto) -> Result<bool, AppError> {
        let code = language.code.clone();
        let name = language.name.clone();
        let is_active = language.is_active.clone();
        let is_native = language.is_native.clone();
        let upt_by = language.upt_by.clone();
        // 查询语言是否存在
        let exist = self.repo.select_by_id(id).await?;
        if exist.is_none() {
            return Err(AppError::BusinessError("language not found".into()));
        }
        let mut exist = exist.unwrap();
        if let Some(code) = code { 
            if let Some(_) = self.repo.find_by_code(&code,id).await? {
             return Err(AppError::BusinessError("repeat language code".into())); 
            }
            exist.code = code;
        }
        if let Some(name) = name {
            if let Some(_) = self.repo.find_by_name(&name,id).await? {
                return Err(AppError::BusinessError("repeat language name".into()));
            }
            exist.name = name;
        }
        if let Some(is_active) = is_active { 
            exist.is_active = is_active
        }
        if let Some(is_native) = is_native {
            exist.is_active = is_native
        }
        if let Some(upt_by) = upt_by {
            exist.upt_by = Some(upt_by)
        }
        self.repo.update_by_id(id,&exist).await
    }
}

impl BaseService<Language> for LanguageService {
    type Repository = LanguageRepository;

    fn get_repository(&self) -> &Self::Repository {
        &self.repo
    }
}
