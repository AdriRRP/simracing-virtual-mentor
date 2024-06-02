use crate::lap::domain::lap::header::Header;
use crate::lap::domain::repository::Repository;

use std::sync::Arc;
use uuid::Uuid;

/// Finder is responsible for finding lap headers by ID from a repository asynchronously.
pub struct Finder<R: Repository> {
    repository: Arc<R>,
}

impl<R: Repository> Finder<R> {
    /// Creates a new instance of Finder.
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    /// Finds a lap header by ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub async fn find(&self, id: &Uuid) -> Result<Option<Header>, String> {
        self.repository.find_header_by_id(id).await
        // Send domain events
    }
}
