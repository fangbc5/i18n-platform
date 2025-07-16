use crate::dtos::user::UpdateUserDto;
use crate::errors::AppError;
use crate::models::user::User;
use crate::repositories::user_repo::UserRepository;
use crate::repositories::BaseRepository;
use crate::services::BaseService;
use crate::utils::password;
use chrono::Local;
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

    pub async fn verify_password(&self, username: &str, password: &str) -> Result<User, AppError> {
        let user = self.repo.find_by_username(username).await?;

        match user {
            Some(user) => {
                if password::verify_password(password, &user.password)? {
                    Ok(user)
                } else {
                    Err(AppError::Unauthorized("Invalid password".into()))
                }
            }
            None => Err(AppError::NotFound("User not found".into())),
        }
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
        self.repo.find_by_username(username).await
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        self.repo.find_by_email(email).await
    }

    pub async fn create_user(
        &self,
        username: &str,
        password: &str,
        email: &str,
    ) -> Result<User, AppError> {
        // 检查用户名是否已存在
        if let Some(_) = self.repo.find_by_username(username).await? {
            return Err(AppError::BadRequest("Username already exists".into()));
        }

        // 检查邮箱是否已存在
        if let Some(_) = self.repo.find_by_email(email).await? {
            return Err(AppError::BadRequest("Email already exists".into()));
        }

        let hashed_password = password::hash_password(password)?;
        let now = Local::now().naive_local();

        let user = User {
            id: 0, // 数据库自增
            tenant_id: 0,
            username: username.to_string(),
            password: hashed_password,
            email: email.to_string(),
            realname: "".to_string(),
            avatar: None,
            status: 1,
            last_login: None,
            crt_by: "system".to_string(),
            crt_at: now,
            upt_by: None,
            upt_at: now,
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

    pub async fn update_by_id(
        &self,
        id: u64,
        user: &UpdateUserDto,
    ) -> Result<bool, AppError> {
        let username = user.username.clone();
        let email = user.email.clone();
        let mut user = self
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
            user.username = username;
        }

        if let Some(email) = email {
            // 检查新邮箱是否已存在
            if let Some(existing) = self.repo.find_by_email(&email).await? {
                if existing.id != id {
                    return Err(AppError::BadRequest("Email already exists".into()));
                }
            }
            user.email = email;
        }

        user.upt_at = Local::now().naive_local();
        user.upt_by = Some("system".to_string());
        self.repo.update_by_id(id.try_into().unwrap(), &user).await
    }

    pub async fn update_password(
        &self,
        id: i64,
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
        user.upt_at = Local::now().naive_local();
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