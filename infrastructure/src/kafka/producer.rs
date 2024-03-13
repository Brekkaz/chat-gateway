use async_trait::async_trait;
use domain::events::Producer;
use log::{info, warn};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use rdkafka::ClientConfig;
use std::time::Duration;

#[derive(Clone)]
pub struct KafkaProducer {
    pub producer: FutureProducer,
}

impl KafkaProducer {
    pub fn new() -> Self {
        let producer = ClientConfig::new()
            .set("bootstrap.servers", std::env::var("KAFKA_BROKER").unwrap())
            .set(
                "security.protocol",
                std::env::var("KAFKA_SECURITY_PROTOCOL")
                    .expect("Can't get KAFKA_SECURITY_PROTOCOL env var"),
            )
            .set(
                "sasl.mechanisms",
                std::env::var("KAFKA_SASL_MECHANISMS").unwrap(),
            )
            .set(
                "sasl.username",
                std::env::var("KAFKA_SASL_USERNAME")
                    .expect("Can't get KAFKA_SASL_USERNAME env var"),
            )
            .set(
                "sasl.password",
                std::env::var("KAFKA_SASL_PASSWORD")
                    .expect("Can't get KAFKA_SASL_PASSWORD env var"),
            )
            .set("message.timeout.ms", "45000")
            .create()
            .expect("Producer creation failed");

        Self { producer }
    }
}

#[async_trait]
impl Producer for KafkaProducer {
    async fn send_message(&self, message: &Vec<u8>, kafka_topic: &str) {
        let record: FutureRecord<String, Vec<u8>> = FutureRecord::to(kafka_topic).payload(&message);
        let delivery_status = self
            .producer
            .send(record, Timeout::After(Duration::from_secs(0)))
            .await;

        match delivery_status {
            Ok(_) => info!("Message was sent, topic: {}", kafka_topic),
            Err(res) => warn!("Message wasn't sent: {}", res.0),
        }
    }
}
