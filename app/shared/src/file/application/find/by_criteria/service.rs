use crate::file::domain::files::Files;
use crate::file::domain::repository::Repository;

use std::sync::Arc;

/// Represents a finder for files.
pub struct Finder<R: Repository> {
    repository: Arc<R>,
}

impl<R: Repository> Finder<R> {
    /// Creates a new `Finder` instance.
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    /// Finds files based on the provided criteria.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the repository fails to find files by criteria.
    pub async fn find(&self, criteria: &str) -> Result<Option<Files>, String> {
        self.repository.find_by_criteria(criteria).await
    }
}
