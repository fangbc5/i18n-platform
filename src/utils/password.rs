use crate::errors::AppError;
use bcrypt::{hash, verify, DEFAULT_COST};

pub fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password, DEFAULT_COST).map_err(|e| AppError::Internal(e.to_string()))
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    verify(password, hash).map_err(|e| AppError::Internal(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hash_and_verify() {
        let password = "test_password";

        // 测试密码加密
        let hashed = hash_password(password).unwrap();
        assert_ne!(password, hashed);

        // 测试密码验证 - 正确密码
        let is_valid = verify_password(password, &hashed).unwrap();
        assert!(is_valid);

        // 测试密码验证 - 错误密码
        let is_valid = verify_password("wrong_password", &hashed).unwrap();
        assert!(!is_valid);
    }
}
