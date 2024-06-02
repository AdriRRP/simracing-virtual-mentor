use crate::lap::domain::repository::Repository;

use std::sync::Arc;
use uuid::Uuid;

/// Deleter is responsible for deleting items from a repository asynchronously.
pub struct Deleter<R: Repository> {
    repository: Arc<R>,
}

impl<R: Repository> Deleter<R> {
    /// Creates a new instance of Deleter.
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    /// Deletes an item with the specified ID from the repository.
    ///
    /// # Errors
    ///
    /// Returns an error if the deletion operation fails.
    pub async fn delete(&self, id: &Uuid) -> Result<(), String> {
        self.repository.delete(id).await
        // Send domain events
    }
}
