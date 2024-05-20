use crate::lap::domain::laps::Laps;
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
    pub async fn create(&self, laps: Laps) {
        self.repository.create(laps).await;
        // Dispatch events if needed
    }
}
