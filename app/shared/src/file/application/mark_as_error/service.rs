use crate::common::domain::event::bus::Bus as EventBus;
use crate::file::domain::repository::Repository;

use std::sync::Arc;

/// Marks a file as Error.
#[allow(dead_code)]
pub struct ErrorMarker<R: Repository, E: EventBus> {
    repository: Arc<R>,
    event_bus: Arc<E>,
}

impl<R: Repository, E: EventBus> ErrorMarker<R, E> {
    /// Creates a new `ErrorMarker` instance.
    pub fn new(repository: Arc<R>, event_bus: Arc<E>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    /// Marks as error a file.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the repository fails to mark as error the operation.
    pub async fn mark_as_error(&self, id: &str, msg: &str) -> Result<(), String> {
        self.repository.mark_as_error(id, msg).await
        // self.event_bus.dispatch(file).await
        // TODO: Log error dispatching file
    }
}
