use crate::errors::AppError;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| AppError::Internal(e.to_string()))
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(hash).map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};
    use tokio::task;

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

    #[test]
    fn test_hash_performance() {
        let password = "test_password";
        let iterations = 100;
        let start = Instant::now();

        for _ in 0..iterations {
            let _ = hash_password(password).unwrap();
        }

        let duration = start.elapsed();
        println!("\nPassword Hash Performance:");
        println!(
            "Hashed {} passwords in {:?} ({:?} per hash)",
            iterations,
            duration,
            duration / iterations
        );
    }

    #[test]
    fn test_verify_performance() {
        let password = "test_password";
        let hash = hash_password(password).unwrap();
        let iterations = 100;
        let start = Instant::now();

        for _ in 0..iterations {
            let _ = verify_password(password, &hash).unwrap();
        }

        let duration = start.elapsed();
        println!("\nPassword Verification Performance:");
        println!(
            "Verified {} passwords in {:?} ({:?} per verification)",
            iterations,
            duration,
            duration / iterations
        );
    }

    #[tokio::test]
    async fn test_concurrent_password_operations() {
        let password = "test_password";
        let hash_iterations = 100;
        let verify_iterations = 100;
        let concurrency = 10;
        let start = Instant::now();

        // 并发哈希测试
        let mut hash_handles = Vec::new();
        for _ in 0..concurrency {
            let password = password.to_string();
            let handle = task::spawn(async move {
                let mut times = Vec::new();
                for _ in 0..hash_iterations / concurrency {
                    let start = Instant::now();
                    let _ = hash_password(&password).unwrap();
                    times.push(start.elapsed());
                }
                times
            });
            hash_handles.push(handle);
        }

        // 并发验证测试
        let hash = hash_password(password).unwrap();
        let mut verify_handles = Vec::new();
        for _ in 0..concurrency {
            let password = password.to_string();
            let hash = hash.clone();
            let handle = task::spawn(async move {
                let mut times = Vec::new();
                for _ in 0..verify_iterations / concurrency {
                    let start = Instant::now();
                    let _ = verify_password(&password, &hash).unwrap();
                    times.push(start.elapsed());
                }
                times
            });
            verify_handles.push(handle);
        }

        let mut total_hash_time = Duration::from_secs(0);
        let mut total_verify_time = Duration::from_secs(0);
        let mut hash_count = 0;
        let mut verify_count = 0;

        for handle in hash_handles {
            let times = handle.await.unwrap();
            for time in times {
                total_hash_time += time;
                hash_count += 1;
            }
        }

        for handle in verify_handles {
            let times = handle.await.unwrap();
            for time in times {
                total_verify_time += time;
                verify_count += 1;
            }
        }

        let total_duration = start.elapsed();
        println!("\nConcurrent Password Operations Performance:");
        println!("Total time: {:?}", total_duration);
        println!("Average hash time: {:?}", total_hash_time / hash_count);
        println!(
            "Average verify time: {:?}",
            total_verify_time / verify_count
        );
        println!(
            "Hash operations per second: {:.2}",
            hash_count as f64 / total_hash_time.as_secs_f64()
        );
        println!(
            "Verify operations per second: {:.2}",
            verify_count as f64 / total_verify_time.as_secs_f64()
        );
    }

    #[test]
    fn test_hash_size() {
        let password = "test_password";
        let hash = hash_password(password).unwrap();
        println!("\nPassword Hash Size Analysis:");
        println!("Hash length: {} bytes", hash.len());
        println!("Hash: {}", hash);
    }
}
