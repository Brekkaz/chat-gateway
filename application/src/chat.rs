use actix_web::error::Error;
use async_trait::async_trait;
use domain::{events::Producer, usecases::Chat};
use std::sync::Arc;

pub struct ChatUseCase {
    producer: Arc<dyn Producer>,
}

impl ChatUseCase {
    pub fn new(producer: Arc<dyn Producer>) -> Self {
        Self { producer }
    }
}

#[async_trait]
impl Chat for ChatUseCase {
    async fn new_message(&self) -> Result<(), Error> {
        self.producer.send_message(&Vec::new(), "lorem").await;
        Ok(())
    }
}
