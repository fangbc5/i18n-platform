use crate::{config::SETTINGS, errors::AppError};
use aws_sdk_s3::{
    config::{Credentials, Region},
    primitives::ByteStream,
    Client,
};
use std::path::Path;

const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10MB
const ALLOWED_EXTENSIONS: [&str; 8] = ["jpg", "jpeg", "png", "gif", "pdf", "txt", "mp4", "mov"];

pub async fn init_s3_client() -> Result<Client, AppError> {
    // 创建认证凭证
    let creds = Credentials::new(
        &SETTINGS.minio.access_key,
        &SETTINGS.minio.secret_key,
        None,      // Session Token
        None,      // 过期时间
        "minio",   // 提供者名称（任意）
    );

    // 配置 S3 客户端
    let config = aws_sdk_s3::Config::builder()
        .credentials_provider(creds)
        .region(Region::new("us-east-1"))  // MinIO 忽略区域，但需要占位符
        .endpoint_url(&SETTINGS.minio.endpoint)
        .force_path_style(true)  // 必须为 true（MinIO 使用路径样式）
        .build();

    let client = Client::from_conf(config);

    // 确保 bucket 存在
    create_bucket_if_not_exists(&client, &SETTINGS.minio.bucket).await?;

    Ok(client)
}

/// 创建存储桶（如果不存在）
async fn create_bucket_if_not_exists(client: &Client, bucket: &str) -> Result<(), AppError> {
    match client.head_bucket().bucket(bucket).send().await {
        Ok(_) => Ok(()),  // 存储桶已存在
        Err(_) => {
            client
                .create_bucket()
                .bucket(bucket)
                .send()
                .await?;
            println!("🪣 创建存储桶: {}", bucket);
            Ok(())
        }
    }
}

fn validate_file(file_path: &Path, content: &[u8]) -> Result<(), AppError> {
    // 检查文件大小
    if content.len() > MAX_FILE_SIZE {
        return Err(AppError::Storage(format!(
            "文件大小超过限制: {} > {}",
            content.len(),
            MAX_FILE_SIZE
        )));
    }

    // 检查文件扩展名
    let extension = file_path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_lowercase());

    match extension {
        Some(ext) if ALLOWED_EXTENSIONS.contains(&ext.as_str()) => Ok(()),
        _ => Err(AppError::Storage(format!(
            "不支持的文件类型: {:?}",
            extension
        ))),
    }
}

pub async fn upload_file(
    client: &Client,
    bucket: &str,
    key: &str,
    content: Vec<u8>,
) -> Result<String, AppError> {
    validate_file(Path::new(key), &content)?;

    let body = ByteStream::from(content);

    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(body)
        .send()
        .await
        .map_err(|e| AppError::Storage(format!("上传文件失败: {}", e)))?;

    Ok(format!("{}/{}/{}", SETTINGS.minio.endpoint, bucket, key))
}

pub async fn download_file(client: &Client, bucket: &str, key: &str) -> Result<Vec<u8>, AppError> {
    let resp = client
        .get_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .map_err(|e| AppError::Storage(format!("下载文件失败: {}", e)))?;

    let data = resp
        .body
        .collect()
        .await
        .map_err(|e| AppError::Storage(format!("读取文件内容失败: {}", e)))?;

    Ok(data.into_bytes().to_vec())
}

pub async fn delete_file(client: &Client, bucket: &str, key: &str) -> Result<(), AppError> {
    client
        .delete_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .map_err(|e| AppError::Storage(format!("删除文件失败: {}", e)))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    use uuid::Uuid;

    #[test]
    fn test_file_validation() {
        // 测试文件大小限制
        let large_content = vec![0; MAX_FILE_SIZE + 1];
        let result = validate_file(Path::new("test.txt"), &large_content);
        assert!(result.is_err());

        // 测试不支持的文件类型
        let result = validate_file(Path::new("test.exe"), b"content");
        assert!(result.is_err());

        // 测试支持的文件类型
        let result = validate_file(Path::new("test.txt"), b"content");
        assert!(result.is_ok());
        let result = validate_file(Path::new("test.jpg"), b"content");
        assert!(result.is_ok());
        let result = validate_file(Path::new("test.pdf"), b"content");
        assert!(result.is_ok());
    }

    async fn setup() -> Client {
        dotenv::dotenv().ok();
        init_s3_client().await.expect("Failed to init S3 client")
    }

    #[tokio::test]
    async fn test_upload_and_download() {
        let client = setup().await;
        let test_bucket = &SETTINGS.minio.bucket;
        let test_content = b"Hello, MinIO!";
        let test_key = format!("test-{}.txt", Uuid::new_v4());

        // 测试上传
        let url = upload_file(&client, test_bucket, &test_key, test_content.to_vec())
            .await
            .expect("Failed to upload file");
        assert!(url.contains(&test_key));

        // 测试下载
        let downloaded = download_file(&client, test_bucket, &test_key)
            .await
            .expect("Failed to download file");
        assert_eq!(downloaded, test_content);
    }
    
    #[tokio::test]
    async fn test_delete_file() {
        let client = setup().await;
        let test_bucket = &SETTINGS.minio.bucket;
        let test_key = format!("test-{}.jpg","54081465-eb69-4fbf-9650-64ab7824cb4c");
        // 测试删除
        delete_file(&client, test_bucket, &test_key)
            .await
            .expect("Failed to delete file");

        // 验证文件已被删除
        let download_result = download_file(&client, test_bucket, &test_key).await;
        assert!(download_result.is_err());
    }

    #[tokio::test]
    async fn test_upload_invalid_file() {
        let client = setup().await;
        let test_bucket = &SETTINGS.minio.bucket;

        // 测试文件大小超限
        let large_content = vec![0; MAX_FILE_SIZE + 1];
        let result = upload_file(&client, test_bucket, "large.txt", large_content).await;
        assert!(result.is_err());

        // 测试不支持的文件类型
        let result = upload_file(&client, test_bucket, "test.exe", b"test content".to_vec()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_upload_image() {
        let client = setup().await;
        let test_bucket = &SETTINGS.minio.bucket;
        let test_key = format!("test-{}.jpg", Uuid::new_v4());

        // 创建一个简单的 1x1 像素的 JPG 图片数据
        let image_data = vec![
            0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01, 0x01, 0x01,
            0x00, 0x48, 0x00, 0x48, 0x00, 0x00, 0xFF, 0xDB, 0x00, 0x43, 0x00, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        ];

        let url = upload_file(&client, test_bucket, &test_key, image_data)
            .await
            .expect("Failed to upload image");
        assert!(url.contains(&test_key));
    }
}
