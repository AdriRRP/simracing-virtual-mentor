use crate::common::domain::event::bus::Bus as EventBus;
use crate::file::domain::event::deleted::Deleted;
use crate::file::domain::repository::Repository;

use std::sync::Arc;

/// Represents a deleter for files.
pub struct Deleter<R: Repository, E: EventBus> {
    repository: Arc<R>,
    event_bus: Arc<E>,
}

impl<R: Repository, E: EventBus> Deleter<R, E> {
    /// Creates a new `Deleter` instance.
    pub fn new(repository: Arc<R>, event_bus: Arc<E>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    /// Deletes a file and dispatches a corresponding event.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the repository fails to delete the file or if an error occurs while dispatching the event.
    pub async fn delete(&self, id: &str) -> Result<(), String> {
        let event = Arc::new(Deleted::new(id));
        self.repository.delete(id).await?;
        self.event_bus.dispatch(event).await
    }
}
