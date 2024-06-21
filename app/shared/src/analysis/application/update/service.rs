use crate::analysis::domain::repository::Repository;

use crate::analysis::domain::analysis::Analysis;
use std::sync::Arc;

/// A struct responsible for updating data asynchronously.
pub struct Updater<R: Repository> {
    repository: Arc<R>,
}

impl<R: Repository> Updater<R> {
    /// Creates a new `Updater` instance.
    ///
    /// # Parameters
    ///
    /// - `repository`: An asynchronous repository for update operations.
    ///
    /// # Returns
    ///
    /// A new `Updater` instance.
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    /// Asynchronously updates data using the repository.
    ///
    /// # Parameters
    ///
    /// - `analysis`: The updated analysis.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the underlying repository fails during the update process.
    pub async fn updater(&self, analysis: &Analysis) -> Result<(), String> {
        self.repository.update(analysis).await
        // Send domain events
    }
}
