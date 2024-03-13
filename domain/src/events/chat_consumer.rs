use crate::utils::AppError;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait ChatConsumer {
    async fn cashout_status_updated(&self, payload: &[u8]) -> Result<(), AppError>;
}
