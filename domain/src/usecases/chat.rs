use actix_web::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait Chat: Send + Sync {
    async fn new_message(&self) -> Result<(), Error>;
}
