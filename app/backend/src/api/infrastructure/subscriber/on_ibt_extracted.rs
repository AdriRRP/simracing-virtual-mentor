use crate::ibt_extractor::domain::event::extracted::Extracted as IbtExtracted;
use crate::shared::domain::event::subscriber::{Error, Subscriber};
use crate::shared::domain::event::Event;

use async_trait::async_trait;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::broadcast::Receiver;

#[derive(Debug)]
pub struct FileUpdater {
    receiver: Receiver<Arc<dyn Event>>,
}

impl FileUpdater {
    #[must_use]
    pub fn new(receiver: Receiver<Arc<dyn Event>>) -> Self {
        Self { receiver }
    }
}

#[async_trait]
impl Subscriber for FileUpdater {
    async fn receive(&mut self) -> Result<Arc<dyn Event>, Error> {
        self.receiver
            .recv()
            .await
            .map_err(|e| Error::Receive(format!("{e}")))
    }

    async fn process(&self, event: Arc<dyn Event>) {
        match event.as_any().downcast_ref::<IbtExtracted>() {
            Some(ibt_parsed) => println!("{ibt_parsed:?}"),
            None => println!("Can't downcast"),
        }
    }
}
