use crate::common::domain::event::bus::Bus as EventBus;
use crate::file::domain::repository::Repository;

use std::sync::Arc;

/// Represents a validator for file operations.
#[allow(dead_code)]
pub struct Validator<R: Repository, E: EventBus> {
    repository: Arc<R>,
    event_bus: Arc<E>,
}

impl<R: Repository, E: EventBus> Validator<R, E> {
    /// Creates a new `Validator` instance.
    pub fn new(repository: Arc<R>, event_bus: Arc<E>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    /// Validates a file operation.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the repository fails to validate the operation.
    pub async fn validate(&self, id: &str) -> Result<(), String> {
        self.repository.validate(id).await
        // self.event_bus.dispatch(file).await
        // TODO: Log error dispatching file
    }
}
