use crate::common::domain::event::Event;

use async_trait::async_trait;
use std::fmt::Debug;
use std::sync::Arc;

#[async_trait]
pub trait Subscriber: Send + Sync + 'static {
    async fn receive(&self) -> Result<Arc<dyn Event>, Error>;
    async fn process(&self, event: Arc<dyn Event>);
    async fn run(&self) {
        while let Ok(event) = self.receive().await {
            self.process(event).await;
        }
    }
}

/// Errors that can be returned from [`Subscriber::receive`].
#[derive(PartialEq, Eq, Debug, thiserror::Error)]
pub enum Error {
    #[error("Error receiving event: {0}")]
    Receive(String),
}
