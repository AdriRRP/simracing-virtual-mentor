use crate::shared::domain::event::Event;

use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait Bus: Send + Sync + 'static {
    async fn dispatch(&self, event: Arc<dyn Event>) -> Result<(), String>;
}
