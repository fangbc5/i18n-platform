use actix_web::{HttpMessage, HttpRequest};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::errors::AppError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: u64,
    pub tenant_id: u64,
    pub exp: i64,
}

impl Claims {
    pub fn new(user_id: u64, tenant_id: u64) -> Self {
        Self {
            sub: user_id,
            tenant_id,
            exp: (Utc::now() + Duration::hours(24)).timestamp(),
        }
    }
}

pub fn create_token(user_id: u64, tenant_id: u64) -> Result<String, AppError> {
    let claims = Claims::new(user_id, tenant_id);
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(e.to_string()))
}

pub fn verify_token(token: &str) -> Result<Claims, AppError> {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
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
    use super::*;

    #[test]
    fn test_jwt_flow() {
        let claims = Claims {
            sub: 123,
            tenant_id: 1,
            exp: (Utc::now() + Duration::hours(24)).timestamp(),
        };

        let token = create_token(123, 1).unwrap();
        let decoded = verify_token(&token).unwrap();

        assert_eq!(claims.sub, decoded.sub);
        assert_eq!(claims.tenant_id, decoded.tenant_id);
    }
}
