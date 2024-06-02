use crate::lap::domain::lap::headers::Headers;
use crate::lap::domain::repository::Repository;
use std::sync::Arc;

/// Finder is responsible for finding lap headers by criteria from a repository asynchronously.
pub struct Finder<R: Repository> {
    repository: Arc<R>,
}

impl<R: Repository> Finder<R> {
    /// Creates a new instance of Finder.
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    /// Finds lap headers by criteria.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub async fn find(&self, criteria: &str) -> Result<Option<Headers>, String> {
        self.repository.find_header_by_criteria(criteria).await
        // Send domain events
    }
}
