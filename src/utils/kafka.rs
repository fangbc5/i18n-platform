use crate::{config::SETTINGS, errors::AppError};
use futures::StreamExt;
use rdkafka::{
    consumer::BaseConsumer,
    consumer::{Consumer, ConsumerContext, Rebalance, StreamConsumer},
    error::KafkaResult,
    message::Message,
    producer::{FutureProducer, FutureRecord},
    ClientConfig, ClientContext, TopicPartitionList,
};
use serde::{de::DeserializeOwned, Serialize};
use std::{fmt::Debug, sync::Arc, time::Duration};
use tokio::sync::mpsc;
use tracing::{error, info, warn};

const MAX_RETRIES: u32 = 3;
const RETRY_DELAY_MS: u64 = 1000;
const BATCH_SIZE: usize = 100;
const CHANNEL_BUFFER_SIZE: usize = 1000;

// 自定义消费者上下文，用于处理再平衡事件
pub struct CustomConsumerContext;

impl ClientContext for CustomConsumerContext {}

impl ConsumerContext for CustomConsumerContext {
    fn pre_rebalance(&self, consumer: &BaseConsumer<Self>, rebalance: &Rebalance) {
        info!("Pre rebalance: {:?}", rebalance);
    }

    fn post_rebalance(&self, consumer: &BaseConsumer<Self>, rebalance: &Rebalance) {
        info!("Post rebalance: {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        match result {
            Ok(_) => info!("Offsets committed successfully"),
            Err(e) => warn!("Error while committing offsets: {}", e),
        }
    }
}

// 生产者相关代码保持不变
pub fn init_producer() -> Result<FutureProducer, AppError> {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", &SETTINGS.kafka.brokers)
        .set("message.timeout.ms", "5000")
        .set("compression.type", "snappy")
        .set("batch.size", "16384")
        .set("linger.ms", "5")
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

// 消费者配置和初始化
pub fn init_consumer(group_id: &str) -> Result<StreamConsumer<CustomConsumerContext>, AppError> {
    let consumer: StreamConsumer<CustomConsumerContext> = ClientConfig::new()
        .set("bootstrap.servers", &SETTINGS.kafka.brokers)
        .set("group.id", group_id)
        .set("enable.auto.commit", "false")
        .set("auto.offset.reset", "earliest")
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "45000")
        .set("max.poll.interval.ms", "300000")
        .set("fetch.min.bytes", "1") // 改小这个值以便测试
        .set("fetch.max.bytes", "5242880")
        .set("fetch.wait.max.ms", "100") // 减少等待时间
        .create_with_context(CustomConsumerContext)
        .map_err(|e| AppError::Queue(format!("创建消费者失败: {}", e)))?;

    Ok(consumer)
}

// 消息处理器特征
#[async_trait::async_trait]
pub trait MessageHandler<T>: Send + Sync + 'static {
    async fn handle_message(&self, message: T) -> Result<(), AppError>;
    async fn handle_batch(&self, messages: Vec<T>) -> Result<(), AppError>;
}

// 批量消息处理
async fn process_batch<T, H>(messages: Vec<T>, handler: Arc<H>) -> Result<(), AppError>
where
    T: DeserializeOwned + Send + 'static,
    H: MessageHandler<T>,
{
    if messages.is_empty() {
        return Ok(());
    }

    let result = handler.handle_batch(messages).await;
    for retry in 1..MAX_RETRIES {
        if result.is_ok() {
            break;
        }
        error!(
            "批处理失败 (重试 {}/{}): {}",
            retry,
            MAX_RETRIES,
            result.as_ref().unwrap_err()
        );
        tokio::time::sleep(Duration::from_millis(RETRY_DELAY_MS)).await;
    }

    result
}

// 启动消费者
pub async fn start_consumer<T, H>(
    consumer: StreamConsumer<CustomConsumerContext>,
    topics: Vec<&str>,
    handler: Arc<H>,
) -> Result<(), AppError>
where
    T: DeserializeOwned + Send + Debug + Clone + 'static,
    H: MessageHandler<T>,
{
    println!("开始订阅主题: {:?}", topics);
    consumer
        .subscribe(topics.as_slice())
        .map_err(|e| AppError::Queue(format!("订阅主题失败: {}", e)))?;

    let (tx, mut rx) = mpsc::channel::<T>(CHANNEL_BUFFER_SIZE);
    let handler_clone = handler.clone();

    // 消息接收任务
    let receive_task = tokio::spawn(async move {
        let mut message_stream = consumer.stream();
        println!("开始监听消息流");
        while let Some(message_result) = message_stream.next().await {
            match message_result {
                Ok(message) => {
                    println!("收到消息");
                    if let Some(payload) = message.payload() {
                        match serde_json::from_slice::<T>(payload) {
                            Ok(parsed_message) => {
                                println!("解析消息成功: {:?}", parsed_message);
                                // 直接处理单条消息
                                if let Err(e) = handler.handle_message(parsed_message.clone()).await
                                {
                                    error!("处理单条消息失败: {}", e);
                                }
                                // 同时发送到批处理通道
                                if tx.send(parsed_message).await.is_err() {
                                    error!("发送消息到通道失败");
                                    break;
                                }
                            }
                            Err(e) => error!("消息解析失败: {}", e),
                        }
                    }
                }
                Err(e) => error!("接收消息失败: {}", e),
            }
        }
        println!("消息流结束");
    });

    // 批处理任务
    let process_task = tokio::spawn(async move {
        let mut batch = Vec::new();
        while let Some(message) = rx.recv().await {
            batch.push(message);

            if batch.len() >= BATCH_SIZE {
                let messages = std::mem::take(&mut batch);
                println!("开始批量处理 {} 条消息", messages.len());
                if let Err(e) = process_batch(messages, handler_clone.clone()).await {
                    error!("批处理失败: {}", e);
                }
            }
        }

        // 处理剩余消息
        if !batch.is_empty() {
            println!("处理剩余 {} 条消息", batch.len());
            if let Err(e) = process_batch(batch, handler_clone).await {
                error!("处理剩余消息失败: {}", e);
            }
        }
    });

    // 等待任务完成
    tokio::try_join!(receive_task, process_task)
        .map_err(|e| AppError::Queue(format!("消费者任务失败: {}", e)))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rdkafka::admin::{AdminClient, AdminOptions, NewTopic, TopicReplication};
    use serde::{Deserialize, Serialize};
    use std::sync::atomic::{AtomicU32, Ordering};
    use tokio::time::sleep;

    async fn create_topic(brokers: &str, topic_name: &str) -> Result<(), AppError> {
        let admin_client: AdminClient<_> = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .create()
            .map_err(|e| AppError::Queue(format!("创建管理客户端失败: {}", e)))?;

        let topic = NewTopic::new(topic_name, 1, TopicReplication::Fixed(1));
        admin_client
            .create_topics(&[topic], &AdminOptions::new())
            .await
            .map_err(|e| AppError::Queue(format!("创建主题失败: {}", e)))?;

        // 等待主题创建完成
        sleep(Duration::from_secs(2)).await;
        println!("主题 {} 创建成功", topic_name);
        Ok(())
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    struct TestMessage {
        id: i32,
        content: String,
    }

    struct TestHandler {
        processed_count: Arc<AtomicU32>,
    }

    #[async_trait::async_trait]
    impl MessageHandler<TestMessage> for TestHandler {
        async fn handle_message(&self, message: TestMessage) -> Result<(), AppError> {
            println!("处理单条消息: {:?}", message);
            self.processed_count.fetch_add(1, Ordering::SeqCst);
            println!(
                "当前处理数: {}",
                self.processed_count.load(Ordering::SeqCst)
            );
            Ok(())
        }

        async fn handle_batch(&self, messages: Vec<TestMessage>) -> Result<(), AppError> {
            let count = messages.len() as u32;
            println!("批量处理 {} 条消息", count);
            self.processed_count.fetch_add(count, Ordering::SeqCst);
            println!(
                "当前处理数（批量后）: {}",
                self.processed_count.load(Ordering::SeqCst)
            );
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_kafka_producer_consumer() {
        // 创建测试主题
        let test_topic = "test_topic";
        create_topic(&SETTINGS.kafka.brokers, test_topic)
            .await
            .expect("Failed to create topic");

        // 初始化生产者和消费者
        let producer = init_producer().expect("Failed to create producer");
        let consumer = init_consumer("test_group").expect("Failed to create consumer");

        println!("Kafka brokers: {}", SETTINGS.kafka.brokers);
        println!("Consumer group: test_group");
        println!("Topic: {}", test_topic);

        let processed_count = Arc::new(AtomicU32::new(0));
        let handler = Arc::new(TestHandler {
            processed_count: processed_count.clone(),
        });

        // 启动消费者
        let consumer_handle = tokio::spawn(start_consumer::<TestMessage, _>(
            consumer,
            vec![test_topic],
            handler,
        ));

        // 等待消费者启动
        sleep(Duration::from_secs(5)).await;

        println!("开始发送测试消息...");
        // 发送测试消息
        for i in 0..10 {
            let message = TestMessage {
                id: i,
                content: format!("test message {}", i),
            };
            match send_message(&producer, test_topic, "test_key", &message).await {
                Ok(_) => println!("已发送消息: {:?}", message),
                Err(e) => println!("发送消息失败: {:?} - {}", message, e),
            }
            // 每条消息发送后稍微等待一下
            sleep(Duration::from_millis(100)).await;
        }

        // 等待消息处理完成
        let mut retries = 0;
        while processed_count.load(Ordering::SeqCst) == 0 && retries < 20 {
            // 增加重试次数
            println!(
                "等待消息处理... 当前处理数: {}",
                processed_count.load(Ordering::SeqCst)
            );
            sleep(Duration::from_secs(1)).await;
            retries += 1;
        }

        let final_count = processed_count.load(Ordering::SeqCst);
        println!("最终处理消息数: {}", final_count);
        assert!(final_count > 0, "没有消息被处理");

        // 清理
        consumer_handle.abort();
    }
}
