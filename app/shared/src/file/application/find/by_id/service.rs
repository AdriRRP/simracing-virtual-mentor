use crate::file::domain::file::File;
use crate::file::domain::repository::Repository;

use std::sync::Arc;

/// Represents a finder for files by ID.
#[derive(Debug)]
pub struct Finder<R: Repository> {
    repository: Arc<R>,
}

impl<R: Repository> Finder<R> {
    /// Creates a new `Finder` instance.
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    /// Finds a file by its ID.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the repository fails to find a file by ID.
    pub async fn find(&self, id: &str) -> Result<Option<File>, String> {
        self.repository.find_by_id(id).await
    }
}
