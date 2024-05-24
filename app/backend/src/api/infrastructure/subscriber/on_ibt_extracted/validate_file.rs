use crate::ibt_extractor::domain::event::extracted::Extracted as IbtExtracted;
use crate::shared::domain::event::subscriber::{Error, Subscriber};
use crate::shared::domain::event::Event;

use crate::shared::infrastructure::event::tokio_bus::TokioBus;
use async_trait::async_trait;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::broadcast::Receiver;
use tokio::sync::RwLock;

type EventReceiver = Receiver<Arc<dyn Event>>;

#[derive(Debug)]
pub struct FileValidator {
    receiver: Arc<RwLock<EventReceiver>>,
}

impl FileValidator {
    #[must_use]
    pub async fn new(event_bus: &Arc<TokioBus>) -> Self {
        println!("Extracted event name: {}", IbtExtracted::event_id());
        let receiver = event_bus.receiver(IbtExtracted::event_id()).await;
        let receiver = Arc::new(RwLock::new(receiver));
        Self { receiver }
    }
}

#[async_trait]
impl Subscriber for FileValidator {
    async fn receive(&self) -> Result<Arc<dyn Event>, Error> {
        let mut receiver = self.receiver.write().await;

        receiver
            .recv()
            .await
            .map_err(|e| Error::Receive(format!("{e}")))
    }

    async fn process(&self, event: Arc<dyn Event>) {
        println!("ohhh {event:?}");
        match event.as_any().downcast_ref::<IbtExtracted>() {
            Some(ibt_parsed) => println!("{ibt_parsed:?}"),
            None => println!("Can't downcast"),
        }
    }
}
