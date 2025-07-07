use crate::{config::SETTINGS, errors::AppError};
use aws_config::{meta::region::RegionProviderChain, retry::RetryConfig, timeout::TimeoutConfig};
use aws_sdk_s3::{
    config::{Credentials, Region},
    primitives::ByteStream,
    Client,
};
use std::path::Path;
use std::time::Duration;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub async fn init_s3_client() -> Result<Client, AppError> {
    let creds = Credentials::new(
        &SETTINGS.minio.access_key,
        &SETTINGS.minio.secret_key,
        None,
        None,
        "static",
    );

    let region_provider = RegionProviderChain::first_try(Region::new("us-east-1"));

    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region(region_provider)
        .credentials_provider(creds)
        .endpoint_url(&SETTINGS.minio.endpoint)
        .retry_config(RetryConfig::new().with_max_attempts(3))
        .timeout_config(
            TimeoutConfig::builder()
                .connect_timeout(Duration::from_secs(5))
                .read_timeout(Duration::from_secs(5))
                .build(),
        )
        .build()
        .await;

    let client = Client::new(&config);
    Ok(client)
}

pub async fn upload_file(
    client: &Client,
    bucket: &str,
    key: &str,
    content: Vec<u8>,
) -> Result<String, AppError> {
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
    use std::fs;
    use tokio;

    #[tokio::test]
    async fn test_s3_operations() {
        let client = init_s3_client().await.unwrap();
        let bucket = &SETTINGS.minio.bucket;
        let test_key = "test/test.txt";
        let test_content = "Hello, MinIO!";

        // 创建测试文件
        let test_file = Path::new("test.txt");
        fs::write(test_file, test_content).unwrap();

        // 测试上传
        let url = upload_file(&client, bucket, test_key, test_content.into_bytes())
            .await
            .unwrap();
        assert!(url.contains(test_key));

        // 测试下载
        let downloaded_content = download_file(&client, bucket, test_key).await.unwrap();
        let downloaded_string = String::from_utf8(downloaded_content).unwrap();
        assert_eq!(downloaded_string, test_content);

        // 测试删除
        delete_file(&client, bucket, test_key).await.unwrap();

        // 清理测试文件
        fs::remove_file(test_file).unwrap();
    }
}
