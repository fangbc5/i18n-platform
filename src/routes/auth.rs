use actix_web::cookie::{time::Duration, Cookie};
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use captcha::filters::Noise;
use captcha::Captcha;
use serde::{Deserialize, Serialize};

use crate::config::SETTINGS;
use crate::constants::{
    USER_EMAIL_REGEX, USER_NAME_MAX_LENGTH, USER_NAME_MIN_LENGTH, USER_PASSWORD_MAX_LENGTH,
    USER_PASSWORD_MIN_LENGTH, USER_PHONE_REGEX,
};
use crate::dtos::user::{CreateUserDto, RegisterRequest};
use crate::errors::AppError;
use crate::services::user_service::UserService;
use crate::utils::{self, jwt, R};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub password: String,
    pub verify_code: Option<String>,
    pub captcha: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_token: String,
}

impl From<jwt::TokenPair> for LoginResponse {
    fn from(token_pair: jwt::TokenPair) -> Self {
        Self {
            access_token: token_pair.access_token,
            expires_in: token_pair.expires_in,
            refresh_token: token_pair.refresh_token,
        }
    }
}

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(login)
            .service(register)
            .service(refresh_token)
            .service(generate_captcha),
    );
}

#[post("/login")]
#[allow(unused_assignments)]
pub async fn login(
    user_service: web::Data<UserService>,
    login_req: web::Json<LoginRequest>,
    http_req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    // 验证用户名、邮箱、手机号、验证码
    let mut user = None;
    if let Some(username) = &login_req.username {
        user = user_service
            .verify_password(username, &login_req.password)
            .await?;
    } else if let Some(email) = &login_req.email {
        if let Some(code) = &login_req.verify_code {
            user = user_service.verify_email_code(email, code).await?;
        } else {
            return Ok(R::<String>::failure("Verify code is required".into()));
        }
    } else if let Some(phone) = &login_req.phone {
        if let Some(code) = &login_req.verify_code {
            user = user_service.verify_phone_code(phone, code).await?;
        } else {
            return Ok(R::<String>::failure("Verify code is required".into()));
        }
    } else {
        return Ok(R::<String>::failure("Username, Email, or Phone is required".into()));
    }

    if user.is_none() {
        return Ok(R::<String>::failure("Invalid username, email, or phone".into()));
    }
    // 校验验证码
    if SETTINGS.server.captcha_enabled {
        if let Some(captcha_id) = http_req.cookie("captcha_token") {
            let captcha_id = captcha_id.value();
            let captcha_id = utils::base64::decode(&captcha_id)?;
            if let Some(captcha) = &login_req.captcha {
                if captcha_id.to_lowercase() != captcha.to_lowercase()   {
                    return Ok(R::<String>::failure("Invalid captcha".into()));
                }
            } else {
                return Ok(R::<String>::failure("Captcha is required".into()));
            }
        } else {
            return Ok(R::<String>::failure("Cookie Param Captcha is required".into()));
        }
    }

    let token_pair = jwt::generate_token_pair(user.unwrap().id)?;
    Ok(R::ok(LoginResponse::from(token_pair)))
}

#[post("/register")]
pub async fn register(
    user_service: web::Data<UserService>,
    register_req: web::Json<RegisterRequest>,
) -> Result<HttpResponse, AppError> {
    // 检查是否为空，手机号、邮箱、用户名不可同时为空
    if register_req.username.is_none()
        && register_req.email.is_none()
        && register_req.phone.is_none()
    {
        return Ok(R::<String>::failure("Username, Email, or Phone is required".into()));
    }
    // 判断是用户名还是邮箱还是手机号
    if let Some(username) = &register_req.username {
        // 检查用户名是否符合要求, 只能包含字母、数字、下划线
        if !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Ok(R::<String>::failure("Username can only contain letters, numbers, and underscores".into()));
        }
        // 长度至少为5
        if username.len() < USER_NAME_MIN_LENGTH {
            return Ok(R::<String>::failure("Username must be at least 5 characters long".into()));
        }
        // 长度不超过32
        if username.len() > USER_NAME_MAX_LENGTH {
            return Ok(R::<String>::failure("Username must be less than 32 characters long".into()));
        }
        // 检查用户名是否已存在
        if user_service.find_by_username(&username).await?.is_some() {
            return Ok(R::<String>::failure("Username already exists".into()));
        }
    } else if let Some(email) = &register_req.email {
        // 检查邮箱是否符合要求, 使用正则表达式
        if !regex::Regex::new(USER_EMAIL_REGEX)
            .unwrap()
            .is_match(&email)
        {
            return Ok(R::<String>::failure("Invalid email".into()));
        }
        // 检查邮箱是否已存在
        if user_service.find_by_email(&email).await?.is_some() {
            return Ok(R::<String>::failure("Email already exists".into()));
        }
    } else if let Some(phone) = &register_req.phone {
        // 检查手机号是否符合要求, 使用正则表达式
        if !regex::Regex::new(USER_PHONE_REGEX)
            .unwrap()
            .is_match(&phone)
        {
            return Ok(R::<String>::failure("Invalid phone".into()));
        }
        // 检查手机号是否已存在
        if user_service.find_by_phone(&phone).await?.is_some() {
            return Ok(R::<String>::failure("Phone already exists".into()));
        }
    }

    // 检查密码是否符合要求, 长度至少为8
    if register_req.password.len() < USER_PASSWORD_MIN_LENGTH {
        return Ok(R::<String>::failure("Password must be at least 8 characters long".into()));
    }

    // 检查密码是否符合要求, 长度不超过32
    if register_req.password.len() > USER_PASSWORD_MAX_LENGTH {
        return Ok(R::<String>::failure("Password must be less than 32 characters long".into()));
    }
    // 如果用户名为空，则校验验证码不为空
    if register_req.username.is_none() && register_req.verify_code.is_none() {
        return Ok(R::<String>::failure("Verify code is required".into()));
    }

    // 密码加密
    let user = user_service
        .create_user(&CreateUserDto::from(&register_req.into_inner()))
        .await?;

    Ok(R::ok(user))
}

#[post("/refresh")]
pub async fn refresh_token(
    refresh_req: web::Json<RefreshTokenRequest>,
) -> Result<HttpResponse, AppError> {
    // 验证刷新令牌
    let claims = jwt::verify_token(&refresh_req.refresh_token)?;

    // 检查是否为刷新令牌
    if claims.refresh.is_none() || !claims.refresh.unwrap() {
        return Ok(R::<String>::failure("Invalid refresh token".into()));
    }

    // 生成新的令牌对
    let token_pair = jwt::generate_token_pair(claims.sub)?;
    Ok(R::ok(LoginResponse::from(token_pair)))
}

#[derive(Debug, Serialize)]
pub struct CaptchaResponse {
    pub image: String,
    pub captcha_id: String,
    pub enabled: bool,
}

#[get("/captcha")]
pub async fn generate_captcha() -> Result<HttpResponse, AppError> {
    if !SETTINGS.server.captcha_enabled {
        return Ok(R::ok(CaptchaResponse {
            image: "".to_string(),
            captcha_id: "".to_string(),
            enabled: SETTINGS.server.captcha_enabled,
        }));
    }

    let mut captcha = Captcha::new();
    captcha.add_chars(4);
    captcha.apply_filter(Noise::new(0.1));
    let image = captcha.view(220, 120).as_base64().unwrap_or_default();

    let code = captcha.chars_as_string();
    let captcha_id = utils::base64::encode(&code);

    let mut response = HttpResponse::Ok();
    let cookie = Cookie::build("captcha_token", captcha_id.clone())
        .path("/")
        .max_age(Duration::seconds(300))
        .http_only(true)
        .finish();
    
    Ok(R::ok_with_cookie(CaptchaResponse {
        image,
        captcha_id,
        enabled: SETTINGS.server.captcha_enabled,
    },cookie))
}
