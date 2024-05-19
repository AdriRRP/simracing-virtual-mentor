use crate::file::domain::event::deleted::Deleted;
use crate::file::domain::repository::Repository;
use crate::shared::domain::event::bus::Bus as EventBus;

use std::sync::Arc;

pub struct Deleter<R: Repository> {
    repository: Arc<R>,
    event_bus: Arc<dyn EventBus>,
}

impl<R: Repository> Deleter<R> {
    pub fn new(repository: Arc<R>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    /// # Errors
    ///
    /// Will return `Err` if `self.repository` fail deleting
    pub async fn delete(&self, id: &str) -> Result<(), String> {
        let event = Arc::new(Deleted::new(id));
        self.repository.delete(id).await?;
        self.event_bus.dispatch(event).await
        // TODO: Log error dispatching event
    }
}
