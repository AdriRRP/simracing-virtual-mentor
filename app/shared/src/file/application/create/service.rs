use crate::common::domain::event::bus::Bus as EventBus;
use crate::file::domain::event::created::Created;
use crate::file::domain::file::File;
use crate::file::domain::repository::Repository;

use std::sync::Arc;

/// Represents a creator for files.
#[derive(Debug)]
pub struct Creator<R: Repository, E: EventBus> {
    repository: Arc<R>,
    event_bus: Arc<E>,
}

impl<R: Repository, E: EventBus> Creator<R, E> {
    /// Creates a new `Creator` instance.
    pub fn new(repository: Arc<R>, event_bus: Arc<E>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    /// Creates a file and dispatches a corresponding event.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the repository fails to create the file.
    pub async fn create(&self, file: File) {
        let event = Arc::new(Created::new(&file));
        if let Err(_err) = self.repository.create(file.clone()).await {
            // TODO: Log error creating file
            return;
        }
        if let Err(_err) = self.event_bus.dispatch(event.clone()).await {
            // TODO: Log error dispatching event
        }
    }
}
