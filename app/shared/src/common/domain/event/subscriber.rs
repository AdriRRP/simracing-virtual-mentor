use crate::common::domain::event::Event;

use async_trait::async_trait;
use std::sync::Arc;

/// Trait representing a subscriber for events.
#[async_trait]
pub trait Subscriber: Send + Sync + 'static {
    /// Receives an event from the event source.
    async fn receive(&self) -> Result<Arc<dyn Event>, Error>;

    /// Processes the received event.
    async fn process(&self, event: Arc<dyn Event>);

    /// Runs the subscriber, continuously receiving and processing events.
    async fn run(&self) {
        while let Ok(event) = self.receive().await {
            self.process(event).await;
        }
    }
}

/// Errors that can occur during event receiving.
#[derive(PartialEq, Eq, Debug, thiserror::Error)]
pub enum Error {
    /// Error receiving event.
    #[error("Error receiving event: {0}")]
    Receive(String),
}
