use crate::common::domain::event::Event;

use async_trait::async_trait;
use std::sync::Arc;

/// Trait representing an file bus.
#[async_trait]
pub trait Bus: Send + Sync + 'static {
    /// Dispatches an file through the bus.
    async fn dispatch(&self, event: Arc<dyn Event>) -> Result<(), String>;
}
