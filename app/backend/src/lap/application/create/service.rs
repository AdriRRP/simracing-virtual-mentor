use crate::lap::domain::lap::Lap;
use crate::lap::domain::repository::Repository;

use std::sync::Arc;

pub struct Creator<R: Repository> {
    repository: Arc<R>,
}

impl<R: Repository> Creator<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    /// # Errors
    ///
    /// Will return `Err` if `self.repository` fail creating
    pub async fn create(&self, lap: Lap) {
        self.repository.create(lap).await
        // Send domain events
    }
}
