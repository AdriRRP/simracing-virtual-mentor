use crate::api::infrastructure::event::tokio_bus::TokioBus;
use crate::api::infrastructure::repository::mongo::lap::Mongo as LapRepository;

use shared::common::domain::criteria::filter::condition::Condition;
use shared::common::domain::criteria::filter::field::Field;
use shared::common::domain::criteria::filter::value::Value;
use shared::common::domain::criteria::filter::Filter;
use shared::common::domain::criteria::filters::Filters;
use shared::common::domain::criteria::Criteria;
use shared::common::domain::event::subscriber::{Error, Subscriber};
use shared::common::domain::event::Event;
use shared::file::domain::event::deleted::Deleted;
use shared::lap::application::delete::service::Deleter;
use shared::lap::application::find::by_criteria::service::Finder;

use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::broadcast::Receiver;
use tokio::sync::RwLock;

type EventReceiver = Receiver<Arc<dyn Event>>;

pub struct LapDeleter {
    receiver: Arc<RwLock<EventReceiver>>,
    finder: Arc<Finder<LapRepository>>,
    deleter: Arc<Deleter<LapRepository>>,
}

impl LapDeleter {
    #[must_use]
    pub async fn new(
        event_bus: &Arc<TokioBus>,
        finder: &Arc<Finder<LapRepository>>,
        deleter: &Arc<Deleter<LapRepository>>,
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
        tracing::trace!("Waiting for next file");
        let mut receiver = self.receiver.write().await;
        receiver
            .recv()
            .await
            .map_err(|e| Error::Receive(format!("{e}")))
    }

    async fn process(&self, event: Arc<dyn Event>) {
        tracing::debug!("Processing new file {}", event.id());
        if let Some(file_deleted) = event.as_any().downcast_ref::<Deleted>() {
            let file_id = file_deleted.id.clone();

            let criteria = Criteria::new(
                Some(Filters::from(vec![Filter::new(
                    Field::new("file_id"),
                    Condition::Equal,
                    Value::new(&file_id),
                )])),
                None,
                None,
                None,
            );

            tracing::debug!("Criteria: {:?}", criteria.clone());

            let laps = match self.finder.find(&criteria).await {
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
