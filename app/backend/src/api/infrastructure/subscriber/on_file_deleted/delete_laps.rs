use crate::file::domain::event::deleted::Deleted;
use crate::lap::application::delete::service::Deleter;
use crate::lap::application::find::by_criteria::service::Finder;
use crate::lap::infrastructure::repository::in_memory::InMemory;
use crate::shared::domain::event::subscriber::{Error, Subscriber};
use crate::shared::domain::event::Event;
use crate::shared::infrastructure::event::tokio_bus::TokioBus;

use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::broadcast::Receiver;
use tokio::sync::RwLock;

type EventReceiver = Receiver<Arc<dyn Event>>;

pub struct LapDeleter {
    receiver: Arc<RwLock<EventReceiver>>,
    finder: Arc<Finder<InMemory>>,
    deleter: Arc<Deleter<InMemory>>,
}

impl LapDeleter {
    #[must_use]
    pub async fn new(
        event_bus: &Arc<TokioBus>,
        finder: &Arc<Finder<InMemory>>,
        deleter: &Arc<Deleter<InMemory>>,
    ) -> Self {
        tracing::debug!("Creating subscriber");

        let receiver = event_bus.receiver(Deleted::event_id()).await;
        let receiver = Arc::new(RwLock::new(receiver));
        let finder = Arc::clone(finder);
        let deleter = Arc::clone(deleter);
        Self {
            receiver,
            finder,
            deleter,
        }
    }
}

#[async_trait]
impl Subscriber for LapDeleter {
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
        if let Some(file_deleted) = event.as_any().downcast_ref::<Deleted>() {
            let file_id = file_deleted.id.clone();
            // Find all laps with given id
            let criteria = "lap where lap.file_id == file_id";
            let laps = match self.finder.find(criteria).await {
                Ok(Some(laps)) => laps,
                Ok(None) => {
                    tracing::warn!("No laps found with file_id `{file_id}`");
                    return;
                }
                Err(e) => {
                    tracing::error!("Can't retrieve laps with file_id `{file_id}`: {e}");
                    return;
                }
            };
            for lap in laps.iter() {
                match self.deleter.delete(&lap.header.id).await {
                    Ok(()) => tracing::trace!("Lap with id {} deleted", lap.header.id),
                    Err(e) => tracing::error!("Can't delete lap with id `{}`: {e}", lap.header.id),
                }
            }
            // Delete all
        } else {
            tracing::error!("Can't downcast {} to {}", event.id(), Deleted::event_id());
        }
    }
}
