use crate::lap::domain::laps::Laps;
use crate::lap::domain::repository::Repository;
use crate::shared::domain::event::bus::Bus as EventBus;

use crate::lap::domain::event::laps_created::LapsCreated;
use std::sync::Arc;

pub struct Creator<R: Repository> {
    repository: Arc<R>,
    event_bus: Arc<dyn EventBus>,
}

impl<R: Repository> Creator<R> {
    pub fn new(repository: Arc<R>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    /// # Errors
    ///
    /// Will return `Err` if `self.repository` fail creating
    pub async fn create(&self, file_id: &str, laps: Laps) {
        let event = Arc::new(LapsCreated::new(file_id));
        self.repository.create(laps).await;
        let _ = self.event_bus.dispatch(event).await;
        // TODO: Log error dispatching event
    }
}
