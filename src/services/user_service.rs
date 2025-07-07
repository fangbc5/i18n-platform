use crate::{
    errors::AppError,
    models::User,
    repositories::UserRepository,
    utils::{jwt::generate_token, password::hash_password},
};

pub struct UserService {
    repo: UserRepository,
}

impl UserService {
    pub fn new(repo: UserRepository) -> Self {
        Self { repo }
    }

    pub async fn register(
        &mut self,
        email: &str,
        password: &str,
        name: &str,
    ) -> Result<User, AppError> {
        // 检查邮箱是否已存在
        if let Some(_) = self.repo.find_by_email(email)? {
            return Err(AppError::Validation("邮箱已存在".into()));
        }

        let hashed_password = hash_password(password)?;
        let user = User {
            id: uuid::Uuid::new_v4().to_string(),
            email: email.to_string(),
            password: hashed_password,
            name: name.to_string(),
            role: "user".to_string(),
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        };

        self.repo.create(&user)
    }

    pub async fn get_user(&mut self, user_id: &str) -> Result<User, AppError> {
        self.repo.find_by_id(user_id)
    }

    pub async fn get_user_by_email(&mut self, email: &str) -> Result<User, AppError> {
        self.repo
            .find_by_email(email)?
            .ok_or_else(|| AppError::NotFound("用户不存在".into()))
    }

    pub async fn login(&mut self, email: &str, password: &str) -> Result<String, AppError> {
        let user = self.get_user_by_email(email).await?;

        if !crate::utils::password::verify_password(password, &user.password)? {
            return Err(AppError::Authentication("密码错误".into()));
        }

        generate_token(&user.id)
    }
}
