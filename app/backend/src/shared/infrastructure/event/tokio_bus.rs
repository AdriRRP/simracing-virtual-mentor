use crate::shared::domain::event::bus::Bus;
use crate::shared::domain::event::Event;

use async_trait::async_trait;
use std::collections::{hash_map, HashMap};
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
        let mut senders = self.senders.write().await;

        if let hash_map::Entry::Vacant(e) = senders.entry(event_id.to_string()) {
            let (sender, receiver) = channel(self.capacity);
            e.insert(sender);
            receiver
        } else {
            senders
                .get(&event_id.to_string())
                .map_or_else(|| unreachable!(), Sender::subscribe)
        }
    }
}

#[async_trait]
impl Bus for TokioBus {
    async fn dispatch(&self, event: Arc<dyn Event>) -> Result<(), String> {
        let event_id = event.id();
        let senders_lock = self.senders.read().await;
        if let Some(sender) = senders_lock.get(event_id) {
            let _ = sender.send(event).map_err(|e| format!("{e}"))?;
            Ok(())
        } else {
            Err(format!("No receivers found for event id `{event_id}`"))
        }
    }
}
