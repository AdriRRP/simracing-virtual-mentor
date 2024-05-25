use crate::file::domain::repository::Repository;
use crate::common::domain::event::bus::Bus as EventBus;

use std::sync::Arc;

#[allow(dead_code)]
pub struct Validator<R: Repository, E: EventBus> {
    repository: Arc<R>,
    event_bus: Arc<E>,
}

impl<R: Repository, E: EventBus> Validator<R, E> {
    pub fn new(repository: Arc<R>, event_bus: Arc<E>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    /// # Errors
    ///
    /// Will return `Err` if `self.repository` fail deleting
    pub async fn validate(&self, id: &str) -> Result<(), String> {
        // let event = Arc::new(Validated::new(id));
        self.repository.validate(id).await
        // self.event_bus.dispatch(event).await
        // TODO: Log error dispatching event
    }
}
