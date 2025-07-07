use crate::{config::SETTINGS, errors::AppError};
use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    ClientConfig,
};
use serde::Serialize;
use std::time::Duration;

pub fn init_producer() -> Result<FutureProducer, AppError> {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", &SETTINGS.kafka.brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .map_err(|e| AppError::Queue(format!("创建生产者失败: {}", e)))?;

    Ok(producer)
}

pub async fn send_message<T: Serialize>(
    producer: &FutureProducer,
    topic: &str,
    key: &str,
    message: &T,
) -> Result<(), AppError> {
    let payload = serde_json::to_string(message)
        .map_err(|e| AppError::Queue(format!("序列化消息失败: {}", e)))?;

    let record = FutureRecord::to(topic).key(key).payload(&payload);

    producer
        .send(record, Duration::from_secs(0))
        .await
        .map_err(|(e, _)| AppError::Queue(format!("发送消息失败: {}", e)))?;

    Ok(())
}

pub async fn send_message_with_headers<T: Serialize>(
    producer: &FutureProducer,
    topic: &str,
    key: &str,
    message: &T,
    headers: Vec<(&str, &[u8])>,
) -> Result<(), AppError> {
    let payload = serde_json::to_string(message)
        .map_err(|e| AppError::Queue(format!("序列化消息失败: {}", e)))?;

    let mut record = FutureRecord::to(topic).key(key).payload(&payload);

    for (key, value) in headers {
        record = record.header(key, value);
    }

    producer
        .send(record, Duration::from_secs(0))
        .await
        .map_err(|(e, _)| AppError::Queue(format!("发送消息失败: {}", e)))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use tokio;

    #[derive(Serialize, Deserialize)]
    struct TestMessage {
        id: i32,
        content: String,
    }

    #[tokio::test]
    async fn test_kafka_producer() {
        let producer = init_producer().unwrap();

        let message = TestMessage {
            id: 1,
            content: "test message".to_string(),
        };

        // 测试基本消息发送
        send_message(&producer, "test_topic", "test_key", &message)
            .await
            .unwrap();

        // 测试带header的消息发送
        let headers = vec![("version", b"1.0"), ("type", b"test")];
        send_message_with_headers(&producer, "test_topic", "test_key", &message, headers)
            .await
            .unwrap();
    }
}
