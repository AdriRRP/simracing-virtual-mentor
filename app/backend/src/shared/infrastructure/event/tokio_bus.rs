use crate::shared::domain::event::bus::Bus;
use crate::shared::domain::event::Event;

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::broadcast::{channel, Receiver, Sender};
use tokio::sync::RwLock;

type EventSender = Sender<Arc<dyn Event>>;

#[derive(Default)]
pub struct TokioBus {
    capacity: usize,
    senders: Arc<RwLock<HashMap<String, EventSender>>>,
}

impl TokioBus {
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            senders: Arc::new(RwLock::new(HashMap::default())),
        }
    }

    pub async fn receiver(&self, event_id: &str) -> Receiver<Arc<dyn Event>> {
        if let Some(sender) = self.senders.read().await.get(event_id) {
            sender.subscribe()
        } else {
            let (sender, receiver) = channel(self.capacity);
            self.senders
                .write()
                .await
                .insert(event_id.to_string(), sender);
            receiver
        }
    }
}

#[async_trait]
impl Bus for TokioBus {
    async fn dispatch(&self, event: Arc<dyn Event>) -> Result<(), String> {
        let event_id = "test"; // TODO!!! Pick from Event
        let senders_lock = self.senders.read().await;
        if let Some(sender) = senders_lock.get(event_id) {
            let _ = sender.send(event).map_err(|e| format!("{e}"))?;
            Ok(())
        } else {
            Err(format!("No receivers found for event id `{event_id}`"))
        }
    }
}
