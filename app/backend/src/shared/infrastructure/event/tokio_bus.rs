use crate::shared::domain::event::bus::Bus;
use crate::shared::domain::event::Event;

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::broadcast::{channel, Receiver, Sender};

pub struct TokioBus {
    capacity: usize,
    senders: HashMap<String, Sender<Arc<dyn Event>>>,
}

impl TokioBus {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            senders: HashMap::new(),
        }
    }

    pub fn receiver(&mut self, event_id: &str) -> Receiver<Arc<dyn Event>> {
        match self.senders.get(event_id) {
            None => {
                let (sender, receiver) = channel(self.capacity);
                self.senders.insert(event_id.to_string(), sender);
                receiver
            }
            Some(sender) => sender.subscribe(),
        }
    }
}

#[async_trait]
impl Bus for TokioBus {
    async fn dispatch(&self, event: Arc<dyn Event>) -> Result<(), String> {
        let event_id = "test";
        if let Some(sender) = self.senders.get(event_id) {
            let _ = sender.send(event).map_err(|e| format!("{e}"))?;
            Ok(())
        } else {
            Err(format!("No receivers found for event id `{event_id}`"))
        }
    }
}
