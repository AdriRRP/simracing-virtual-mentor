use crate::lap::domain::laps::Laps;
use crate::lap::domain::repository::Repository;

use std::sync::Arc;

/// Finder is responsible for finding laps based on criteria from a repository asynchronously.
pub struct Finder<R: Repository> {
    repository: Arc<R>,
}

impl<R: Repository> Finder<R> {
    /// Creates a new instance of Finder.
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    /// Finds laps based on the specified criteria.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub async fn find(&self, criteria: &str) -> Result<Option<Laps>, String> {
        self.repository.find_by_criteria(criteria).await
        // Send domain events
    }
}
