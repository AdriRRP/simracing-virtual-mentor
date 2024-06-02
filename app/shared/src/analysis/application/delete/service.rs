use crate::analysis::domain::repository::Repository;

use std::sync::Arc;
use uuid::Uuid;

/// A struct responsible for deleting data asynchronously.
pub struct Deleter<R: Repository> {
    repository: Arc<R>,
}

impl<R: Repository> Deleter<R> {
    /// Creates a new `Deleter` instance.
    ///
    /// # Parameters
    ///
    /// - `repository`: An asynchronous repository for deletion operations.
    ///
    /// # Returns
    ///
    /// A new `Deleter` instance.
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    /// Asynchronously deletes data using the repository.
    ///
    /// # Parameters
    ///
    /// - `id`: The identifier of the data to be deleted.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the underlying repository fails during the deletion process.
    pub async fn delete(&self, id: &Uuid) -> Result<(), String> {
        self.repository.delete(id).await
        // Send domain events
    }
}
