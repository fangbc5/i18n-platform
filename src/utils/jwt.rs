use actix_web::{HttpMessage, HttpRequest};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::errors::AppError;

const ACCESS_TOKEN_EXPIRES_IN: i64 = 300; // 访问令牌过期时间（1小时）
const REFRESH_TOKEN_EXPIRES_IN: i64 = 25200; // 刷新令牌过期时间（7天）

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: u64, // 用户ID
    pub username: Option<String>,
    pub exp: i64, // 过期时间
    pub iat: i64, // 签发时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh: Option<bool>, // 是否为刷新令牌
}

#[derive(Debug)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

impl Claims {
    pub fn new(user_id: u64, username: Option<String>, is_refresh: bool) -> Self {
        let now = Utc::now().timestamp();
        let expires_in = if is_refresh {
            REFRESH_TOKEN_EXPIRES_IN
        } else {
            ACCESS_TOKEN_EXPIRES_IN
        };

        Self {
            sub: user_id,
            username,
            iat: now,
            exp: now + expires_in,
            refresh: if is_refresh { Some(true) } else { None },
        }
    }
}

pub fn generate_token_pair(user_id: u64, username: Option<String>) -> Result<TokenPair, AppError> {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());
    let encoding_key = EncodingKey::from_secret(secret.as_bytes());

    // 生成访问令牌
    let access_claims = Claims::new(user_id, username.clone(), false);
    let access_token = encode(&Header::default(), &access_claims, &encoding_key)
        .map_err(|e| AppError::Unauthorized(format!("Failed to create access token: {}", e)))?;

    // 生成刷新令牌
    let refresh_claims = Claims::new(user_id, username, true);
    let refresh_token = encode(&Header::default(), &refresh_claims, &encoding_key)
        .map_err(|e| AppError::Unauthorized(format!("Failed to create refresh token: {}", e)))?;

    Ok(TokenPair {
        access_token,
        refresh_token,
        expires_in: ACCESS_TOKEN_EXPIRES_IN,
    })
}

pub fn verify_token(token: &str) -> Result<Claims, AppError> {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());

    let token_data = jsonwebtoken::decode::<Claims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()),
        &jsonwebtoken::Validation::default(),
    )
    .map_err(|e| AppError::Unauthorized(e.to_string()))?;

    Ok(token_data.claims)
}

pub fn get_claims(req: &HttpRequest) -> Result<Claims, AppError> {
    let extensions = req.extensions();
    let claims = extensions
        .get::<Claims>()
        .ok_or(AppError::Internal("Claims not found".into()))?;
    Ok(claims.clone())
}

#[cfg(test)]
mod tests {
    use chrono::Duration as ChronoDuration;
    use std::time::{Duration, Instant};
    use tokio::task;

    use super::*;

    #[test]
    fn test_jwt_flow() {
        let claims = Claims {
            sub: 123,
            username: Some("admin".to_owned()),
            exp: (Utc::now() + ChronoDuration::hours(1)).timestamp(),
            iat: Utc::now().timestamp(),
            refresh: None,
        };

        let token_pair = generate_token_pair(123, Some("admin".to_string())).unwrap();
        let decoded_access = verify_token(&token_pair.access_token).unwrap();
        let decoded_refresh = verify_token(&token_pair.refresh_token).unwrap();

        assert_eq!(claims.sub, decoded_access.sub);
        assert_eq!(claims.exp, decoded_access.exp);
        assert_eq!(claims.iat, decoded_access.iat);
        assert_eq!(claims.refresh, decoded_access.refresh);

        assert_eq!(claims.sub, decoded_refresh.sub);
        assert_eq!(claims.exp, decoded_refresh.exp);
        assert_eq!(claims.iat, decoded_refresh.iat);
        assert_eq!(claims.refresh, decoded_refresh.refresh);
    }

    #[test]
    fn test_token_generation_performance() {
        let iterations = 1000;
        let start = Instant::now();

        for i in 0..iterations {
            let _ = generate_token_pair(i as u64, Some("admin".to_string())).unwrap();
        }

        let duration = start.elapsed();
        println!(
            "Generated {} token pairs in {:?} ({:?} per token pair)",
            iterations,
            duration,
            duration / iterations
        );
    }

    #[test]
    fn test_token_verification_performance() {
        let token_pair = generate_token_pair(123, Some("admin".to_string())).unwrap();
        let iterations = 1000;
        let start = Instant::now();

        for _ in 0..iterations {
            let _ = verify_token(&token_pair.access_token).unwrap();
        }

        let duration = start.elapsed();
        println!(
            "Verified {} tokens in {:?} ({:?} per token)",
            iterations,
            duration,
            duration / iterations
        );
    }

    #[tokio::test]
    async fn test_concurrent_token_operations() {
        let iterations = 1000;
        let concurrency = 10;
        let start = Instant::now();

        let mut handles = Vec::new();

        // 并发生成和验证令牌
        for i in 0..concurrency {
            let handle = task::spawn(async move {
                let mut times = Vec::new();
                for j in 0..iterations / concurrency {
                    let gen_start = Instant::now();
                    let token_pair =
                        generate_token_pair((i * iterations / concurrency + j) as u64, Some("admin".to_string())).unwrap();
                    let gen_time = gen_start.elapsed();

                    let verify_start = Instant::now();
                    let _ = verify_token(&token_pair.access_token).unwrap();
                    let verify_time = verify_start.elapsed();

                    times.push((gen_time, verify_time));
                }
                times
            });
            handles.push(handle);
        }

        let mut total_gen_time = Duration::from_secs(0);
        let mut total_verify_time = Duration::from_secs(0);
        let mut count = 0;

        for handle in handles {
            let times = handle.await.unwrap();
            for (gen_time, verify_time) in times {
                total_gen_time += gen_time;
                total_verify_time += verify_time;
                count += 1;
            }
        }

        let total_duration = start.elapsed();
        println!("\nConcurrent Performance Test Results:");
        println!(
            "Total time for {} operations: {:?}",
            iterations, total_duration
        );
        println!("Average generation time: {:?}", total_gen_time / count);
        println!("Average verification time: {:?}", total_verify_time / count);
        println!(
            "Operations per second: {:.2}",
            iterations as f64 / total_duration.as_secs_f64()
        );
    }

    #[test]
    fn test_token_size() {
        let token_pair = generate_token_pair(123, Some("admin".to_string())).unwrap();
        println!("\nToken Size Analysis:");
        println!(
            "Access token length: {} bytes",
            token_pair.access_token.len()
        );
        println!(
            "Refresh token length: {} bytes",
            token_pair.refresh_token.len()
        );
    }
}
