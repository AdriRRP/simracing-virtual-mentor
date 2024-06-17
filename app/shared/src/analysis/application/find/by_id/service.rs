use crate::analysis::domain::analysis::Analysis;
use crate::analysis::domain::repository::Repository;

use std::sync::Arc;
use uuid::Uuid;

/// A struct responsible for finding data asynchronously based on criteria.
pub struct Finder<R: Repository> {
    repository: Arc<R>,
}

impl<R: Repository> Finder<R> {
    /// Creates a new `Finder` instance.
    ///
    /// # Parameters
    ///
    /// - `repository`: An asynchronous repository for finding operations.
    ///
    /// # Returns
    ///
    /// A new `Finder` instance.
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }
    
    /// Asynchronously finds data by ID using the repository.
    ///
    /// # Parameters
    ///
    /// - `id`: The identifier of the data to be found.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the underlying repository fails during the find operation.
    pub async fn find(&self, id: &Uuid) -> Result<Option<Analysis>, String> {
        self.repository.find_by_id(id).await
        // Send domain events
    }
}
