use crate::lap::domain::laps::Laps;
use crate::lap::domain::repository::Repository;

use std::sync::Arc;

/// A struct responsible for creating laps asynchronously.
#[derive(Debug)]
pub struct Creator<R: Repository> {
    repository: Arc<R>,
}

impl<R: Repository> Creator<R> {
    /// Creates a new `Creator` instance.
    ///
    /// # Parameters
    ///
    /// - `repository`: An asynchronous repository for lap operations.
    ///
    /// # Returns
    ///
    /// A new `Creator` instance.
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    /// Asynchronously creates laps using the repository.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the underlying repository fails during the creation process.
    pub async fn create(&self, laps: Laps) {
        let _ = self.repository.create(laps).await;
        // Dispatch events if needed
    }
}
