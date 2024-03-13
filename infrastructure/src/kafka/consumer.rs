use super::TOPIC_TEST;
use crate::websocket::{ChatServer, MessageFromBroker};
use actix::Addr;
use anyhow::Result;
use async_trait::async_trait;
use domain::{
    events::{ChatConsumer as IChatConsumer, Consumer},
    utils::AppError,
};
use log::info;
use rdkafka::{
    message::{BorrowedMessage, OwnedMessage},
    Message,
};
use uuid::Uuid;

pub struct ChatConsumer {
    server: Addr<ChatServer>,
}

impl ChatConsumer {
    pub fn new(server: Addr<ChatServer>) -> ChatConsumer {
        ChatConsumer { server }
    }
}

#[async_trait]
impl IChatConsumer for &ChatConsumer {
    async fn cashout_status_updated(&self, _payload: &[u8]) -> Result<(), AppError> {
        self.server.do_send(MessageFromBroker {
            session_id: Uuid::nil(),
            sender_id: Uuid::nil(),
            receiver_id: Uuid::parse_str("fe5c762a-5e78-4e95-96d1-c197b2cda605")?,
            msg: "msg from kafka".to_string(),
        });
        Ok(())
    }
}

#[async_trait]
impl Consumer for ChatConsumer {
    async fn record_borrowed_message_receipt(&self, msg: &BorrowedMessage<'_>) {
        info!("Message received Borrowed: {:?}", msg);
    }

    async fn record_owned_message_receipt(&self, msg: &OwnedMessage) -> Result<()> {
        //info!("Message received Owned: {:?}", msg);

        match msg.payload() {
            Some(payload) => {
                let result: Result<(), AppError> = match msg.topic() {
                    TOPIC_TEST => self.cashout_status_updated(payload).await,
                    _ => Ok({}),
                };

                if let Err(err) = result {
                    log::error!("Error: {:?}", err);
                }
            }
            None => {
                log::error!("Kafka message has no payload");
            }
        }

        Ok(())
    }
}
