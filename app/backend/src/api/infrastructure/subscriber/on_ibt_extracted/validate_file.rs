use crate::file::application::validate::service::Validator;
use crate::file::infrastructure::repository::in_memory::InMemory;
use crate::ibt_extractor::domain::event::extracted::Extracted as IbtExtracted;
use crate::shared::domain::event::subscriber::{Error, Subscriber};
use crate::shared::domain::event::Event;
use crate::shared::infrastructure::event::tokio_bus::TokioBus;

use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::broadcast::Receiver;
use tokio::sync::RwLock;

type EventReceiver = Receiver<Arc<dyn Event>>;

pub struct FileValidator {
    receiver: Arc<RwLock<EventReceiver>>,
    validator: Arc<Validator<InMemory, TokioBus>>,
}

impl FileValidator {
    #[must_use]
    pub async fn new(
        event_bus: &Arc<TokioBus>,
        validator: &Arc<Validator<InMemory, TokioBus>>,
    ) -> Self {
        tracing::debug!("Creating subscriber");

        let receiver = event_bus.receiver(IbtExtracted::event_id()).await;
        let receiver = Arc::new(RwLock::new(receiver));
        let validator = Arc::clone(validator);
        Self {
            receiver,
            validator,
        }
    }
}

#[async_trait]
impl Subscriber for FileValidator {
    async fn receive(&self) -> Result<Arc<dyn Event>, Error> {
        tracing::trace!("Waiting for next event");
        let mut receiver = self.receiver.write().await;
        receiver
            .recv()
            .await
            .map_err(|e| Error::Receive(format!("{e}")))
    }

    async fn process(&self, event: Arc<dyn Event>) {
        tracing::debug!("Processing new event {}", event.id());
        if let Some(ibt_parsed) = event.as_any().downcast_ref::<IbtExtracted>() {
            let validated = self.validator.validate(&ibt_parsed.file_id).await;
            match validated {
                Ok(()) => tracing::info!("File `{}` validated", ibt_parsed.file_id),
                Err(e) => {
                    tracing::error!("Cannot validate file `{}`: {}", ibt_parsed.file_id, e);
                }
            }
        } else {
            tracing::error!(
                "Can't downcast {} to {}",
                event.id(),
                IbtExtracted::event_id()
            );
        }
    }
}
