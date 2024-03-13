use async_trait::async_trait;

#[async_trait]
pub trait Producer: Send + Sync {
    async fn send_message(&self, message: &Vec<u8>, kafka_topic: &str);
}
