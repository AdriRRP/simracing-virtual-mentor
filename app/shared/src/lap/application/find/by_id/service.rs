use crate::lap::domain::lap::Lap;
use crate::lap::domain::repository::Repository;

use std::sync::Arc;
use uuid::Uuid;

/// Finder is responsible for finding laps by ID from a repository asynchronously.
pub struct Finder<R: Repository> {
    repository: Arc<R>,
}

impl<R: Repository> Finder<R> {
    /// Creates a new instance of Finder.
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    /// Finds a lap by ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub async fn find(&self, id: &Uuid) -> Result<Option<Lap>, String> {
        self.repository.find_by_id(id).await
        // Send domain events
    }
}
