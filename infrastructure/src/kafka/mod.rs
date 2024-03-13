mod consumer;
mod kafka_consumer;
mod producer;

pub use {consumer::ChatConsumer, kafka_consumer::KafkaConsumer, producer::KafkaProducer};

pub const TOPIC_TEST: &str = "lorem";
