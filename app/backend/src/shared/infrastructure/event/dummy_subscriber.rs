use crate::shared::domain::event::subscriber::Subscriber;
use crate::shared::domain::event::Event;

use crate::shared::infrastructure::event::tokio_bus::TokioBus;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::broadcast::Receiver;
use tokio::sync::Mutex;

pub struct DummySubscriber {
    receiver: Receiver<Arc<dyn Event>>,
}

impl DummySubscriber {
    pub async fn new(event_bus: Arc<Mutex<TokioBus>>) -> Self {
        let mut tokio_bus_lock = event_bus.lock().await;
        let receiver = tokio_bus_lock.receiver("test");
        Self { receiver }
    }

    pub async fn run(&mut self) {
        while let Ok(event) = self.receiver.recv().await {
            self.receive(event).await;
        }
    }
}

#[async_trait]
impl Subscriber for DummySubscriber {
    async fn receive(&self, event: Arc<dyn Event>) {
        println!("{:?}", event);
    }
}
