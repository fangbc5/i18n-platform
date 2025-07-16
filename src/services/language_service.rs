use std::sync::Arc;

use sqlx::MySqlPool;

use crate::{
    dtos::language::{CreateLanguageDto, LanguageVo, UpdateLanguageDto},
    errors::AppError,
    models::language::Language,
    repositories::{base_repo::BaseRepository, language_repo::LanguageRepository}, services::BaseService,
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

    /// 创建语言
    pub async fn insert(&self, dto: CreateLanguageDto) -> Result<LanguageVo, AppError> {
        // 检查语言代码是否已存在
        if let Some(_) = self.repo.find_by_code(&dto.code).await? {
            return Err(AppError::BadRequest("Language code already exists".into()));
        }

        // 检查语言名称是否已存在
        if let Some(_) = self.repo.find_by_name(&dto.name).await? {
            return Err(AppError::BadRequest("Language name already exists".into()));
        }

        let now = chrono::Local::now().naive_local();

        // 创建语言
        let language = Language {
            id: 0,
            code: dto.code,
            name: dto.name,
            native_name: dto.native_name,
            is_active: true,
            crt_at: now,
            upt_at: now,
            crt_by: "system".to_string(), // 设置为系统创建
            upt_by: None,
        };

        let id = self.repo.insert(&language).await?;
        let language = self
            .repo
            .select_by_id(id)
            .await?
            .ok_or_else(|| AppError::Internal("Failed to get created language".into()))?;

        Ok(language.into())
    }

    /// 获取语言信息
    pub async fn select_by_id(&self, id: u64) -> Result<LanguageVo, AppError> {
        let language = self
            .repo
            .select_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Language not found".into()))?;
        Ok(language.into())
    }

    /// 更新语言信息
    pub async fn update_by_id(
        &self,
        id: u64,
        dto: UpdateLanguageDto,
    ) -> Result<LanguageVo, AppError> {
        let mut language = self
            .repo
            .select_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Language not found".into()))?;

        // 如果要更新语言名称，检查是否已存在
        if let Some(name) = &dto.name {
            if let Some(existing) = self.repo.find_by_name(name).await? {
                if existing.id != id {
                    return Err(AppError::BadRequest("Language name already exists".into()));
                }
            }
            language.name = name.clone();
        }

        // 如果要更新语言代码，检查是否已存在
        if let Some(code) = &dto.code {
            if let Some(existing) = self.repo.find_by_code(code).await? {
                if existing.id != id {
                    return Err(AppError::BadRequest("Language code already exists".into()));
                }
            }
            language.code = code.clone();
        }

        // 更新其他字段
        if let Some(native_name) = dto.native_name {
            language.native_name = native_name;
        }
        if let Some(is_active) = dto.is_active {
            language.is_active = is_active;
        }

        self.repo.update_by_id(id, &language).await?;
        Ok(language.into())
    }

    /// 删除语言
    pub async fn delete_by_id(&self, id: u64) -> Result<(), AppError> {
        if !self.repo.delete_by_id(id).await? {
            return Err(AppError::NotFound("Language not found".into()));
        }
        Ok(())
    }

    /// 获取语言列表
    pub async fn get_languages(
        &self,
        page: u32,
        page_size: u32,
    ) -> Result<(Vec<LanguageVo>, u64), AppError> {
        let (languages, total) = self.repo.select_by_page(page, page_size).await?;
        Ok((languages.into_iter().map(|l| l.into()).collect(), total))
    }
}

impl BaseService<Language> for LanguageService {
    type Repository = LanguageRepository;

    fn get_repository(&self) -> &Self::Repository {
        &self.repo
    }
}