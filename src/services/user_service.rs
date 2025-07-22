use crate::dtos::user::{CreateUserDto, PageUserRequest, UpdateUserDto, UserVo};
use crate::errors::AppError;
use crate::models::user::User;
use crate::repositories::user_repo::UserRepository;
use crate::repositories::BaseRepository;
use crate::services::BaseService;
use crate::utils::password;
use chrono::Utc;
use sqlx::MySqlPool;
use std::sync::Arc;

pub struct UserService {
    repo: UserRepository,
}

impl UserService {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Self {
            repo: UserRepository::new(pool),
        }
    }

    pub async fn select_by_page(
        &self,
        req: &PageUserRequest,
    ) -> Result<(Vec<UserVo>, i64), AppError> {
        if let Some(search_key) = &req.search_key {
            let users = self
                .repo
                .select_page_by_key(req.page, req.size, search_key)
                .await?;
            let list = users.0.iter().map(|user| UserVo::from(user)).collect();
            Ok((list, users.1))
        } else {
            let users = self.repo.select_by_page(req.page, req.size).await?;
            let list = users.0.iter().map(|user| UserVo::from(user)).collect();
            Ok((list, users.1))
        }
    }

    pub async fn verify_password(
        &self,
        username: &str,
        password: &str,
    ) -> Result<Option<User>, AppError> {
        let user = self.repo.find_by_username(username).await?;

        match user {
            Some(user) => {
                if password::verify_password(password, &user.password)? {
                    Ok(Some(user))
                } else {
                    Err(AppError::Auth("Invalid password".into()))
                }
            }
            None => Err(AppError::NotFound("User not found".into())),
        }
    }

    pub async fn verify_email_code(
        &self,
        email: &str,
        code: &str,
    ) -> Result<Option<User>, AppError> {
        let user = self.repo.find_by_email(email).await?;
        Ok(user)
    }

    pub async fn verify_phone_code(
        &self,
        phone: &str,
        code: &str,
    ) -> Result<Option<User>, AppError> {
        let user = self.repo.find_by_phone(phone).await?;
        Ok(user)
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
        self.repo.find_by_username(username).await
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        self.repo.find_by_email(email).await
    }

    pub async fn find_by_phone(&self, phone: &str) -> Result<Option<User>, AppError> {
        self.repo.find_by_phone(phone).await
    }

    pub async fn create_user(&self, user: &CreateUserDto) -> Result<User, AppError> {
        let user = User {
            id: 0,        // 数据库自增
            tenant_id: 0, // 租户id
            username: user.username.clone(),
            password: password::hash_password(&user.password)?,
            email: user.email.clone(),
            phone: user.phone.clone(),
            realname: None,
            id_card: None,
            nickname: user.nickname.clone(),
            avatar: None,
            gender: None,
            birthday: None,
            status: user.status.clone().unwrap_or(true),
            last_login: None,
            crt_by: user.crt_by.clone().unwrap_or("register".to_owned()),
            crt_at: Utc::now(),
            upt_by: None,
            upt_at: Utc::now(),
        };

        let _id = self.insert(&user).await?;
        Ok(user)
    }

    pub async fn select_all(&self) -> Result<Vec<User>, AppError> {
        self.repo.select_all().await
    }

    pub async fn select_by_id(&self, id: u64) -> Result<User, AppError> {
        let user = self
            .repo
            .select_by_id(id.try_into().unwrap())
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".into()))?;
        Ok(user)
    }

    pub async fn update_by_id(&self, id: u64, user: &UpdateUserDto) -> Result<bool, AppError> {
        let username = user.username.clone();
        let email = user.email.clone();
        let nickname = user.nickname.clone();
        let status = user.status.clone();
        let mut exist_user = self
            .repo
            .select_by_id(id.try_into().unwrap())
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".into()))?;

        if let Some(username) = username {
            // 检查新用户名是否已存在
            if let Some(existing) = self.repo.find_by_username(&username).await? {
                if existing.id != id {
                    return Err(AppError::BadRequest("Username already exists".into()));
                }
            }
            exist_user.username = Some(username);
        }

        if let Some(email) = email {
            // 检查新邮箱是否已存在
            if let Some(existing) = self.repo.find_by_email(&email).await? {
                if existing.id != id {
                    return Err(AppError::BadRequest("Email already exists".into()));
                }
            }
            exist_user.email = Some(email);
        }
        
        if let Some(nickname) = nickname { 
            exist_user.nickname = Some(nickname);
        }
        if let Some(status) = user.status {
            exist_user.status = status;
        }

        exist_user.upt_by = user.upt_by.clone();
        self.repo.update_by_id(id.try_into().unwrap(), &exist_user).await
    }

    pub async fn update_password(
        &self,
        id: u64,
        old_password: &str,
        new_password: &str,
    ) -> Result<bool, AppError> {
        let user = self
            .repo
            .select_by_id(id.try_into().unwrap())
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".into()))?;

        if !password::verify_password(old_password, &user.password)? {
            return Err(AppError::BadRequest("Invalid old password".into()));
        }

        let mut user = user;
        user.password = password::hash_password(new_password)?;
        user.upt_at = Utc::now();
        user.upt_by = Some("system".to_string());

        self.repo.update_by_id(id.try_into().unwrap(), &user).await
    }
}

impl BaseService<User> for UserService {
    type Repository = UserRepository;

    fn get_repository(&self) -> &Self::Repository {
        &self.repo
    }
}
