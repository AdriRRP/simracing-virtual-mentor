use crate::analysis::domain::analysis::Analysis;
use crate::analysis::domain::repository::Repository;

use std::sync::Arc;

/// A struct responsible for creating analysis data asynchronously.
pub struct Creator<R: Repository> {
    repository: Arc<R>,
}

impl<R: Repository> Creator<R> {
    /// Creates a new `Creator` instance.
    ///
    /// # Parameters
    ///
    /// - `repository`: An asynchronous repository for analysis operations.
    ///
    /// # Returns
    ///
    /// A new `Creator` instance.
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    /// Asynchronously creates analysis data using the repository.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the underlying repository fails during the creation process.
    pub async fn create(&self, analysis: Analysis) -> Result<(), String> {
        self.repository.create(analysis).await
        // Send domain events
    }
}
