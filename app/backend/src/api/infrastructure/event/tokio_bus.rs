use symracing_virtual_mentor_shared::common::domain::event::bus::Bus;
use symracing_virtual_mentor_shared::common::domain::event::Event;

use async_trait::async_trait;
use std::collections::{hash_map, HashMap};
use std::sync::Arc;
use tokio::sync::broadcast::{channel, Receiver, Sender};
use tokio::sync::RwLock;

type EventSender = Sender<Arc<dyn Event>>;

#[derive(Default, Debug)]
pub struct TokioBus {
    capacity: usize,
    senders: Arc<RwLock<HashMap<String, EventSender>>>,
}

impl TokioBus {
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        tracing::trace!("Creating TokioBus with capacity {capacity}");
        Self {
            capacity,
            senders: Arc::new(RwLock::new(HashMap::default())),
        }
    }

    pub async fn receiver(&self, event_id: &str) -> Receiver<Arc<dyn Event>> {
        let mut senders = self.senders.write().await;

        if let hash_map::Entry::Vacant(e) = senders.entry(event_id.to_string()) {
            tracing::trace!("Creating new channel for events `{event_id}`");
            let (sender, receiver) = channel(self.capacity);
            e.insert(sender);
            tracing::trace!("Retrieve new receiver for events `{event_id}`");
            receiver
        } else {
            tracing::trace!("Retrieve existing receiver for events `{event_id}`");
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

        tracing::trace!("Dispatching file `{event_id}`");

        let senders_lock = self.senders.read().await;

        if let Some(sender) = senders_lock.get(event_id) {
            let _ = sender.send(event).map_err(|e| format!("{e}"))?;

            Ok(())
        } else {
            let msg = format!("No receivers found for file `{event_id}`");

            tracing::warn!(msg);

            Err(msg)
        }
    }
}
