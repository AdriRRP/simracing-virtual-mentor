use crate::shared::domain::event::Event;

use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait Subscriber {
    async fn receive(&self, event: Arc<dyn Event>);
}
