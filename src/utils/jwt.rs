use crate::{config::SETTINGS, errors::AppError};
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(user_id: String) -> Self {
        let now = Utc::now();
        let expiry = now + Duration::hours(SETTINGS.jwt.expiration);
        Self {
            sub: user_id,
            exp: expiry.timestamp(),
            iat: now.timestamp(),
        }
    }
}

pub fn generate_token(user_id: &str) -> Result<String, AppError> {
    let claims = Claims::new(user_id.to_string());
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SETTINGS.jwt.secret.as_bytes()),
    )
    .map_err(|e| AppError::Token(e.to_string()))
}

pub fn decode_token(token: &str) -> Result<Claims, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SETTINGS.jwt.secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| AppError::Token(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_flow() {
        let claims = Claims::new("123".to_string());

        let token = generate_token(&claims.sub).unwrap();
        let decoded = decode_token(&token).unwrap();

        assert_eq!(claims.sub, decoded.sub);
    }
}
