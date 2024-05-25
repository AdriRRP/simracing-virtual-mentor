use crate::file::domain::file::File;
use crate::file::domain::repository::Repository;

use std::sync::Arc;

#[derive(Debug)]
pub struct Finder<R: Repository> {
    repository: Arc<R>,
}

impl<R: Repository> Finder<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    /// # Errors
    ///
    /// Will return `Err` if `self.repository` fail finding by id
    pub async fn find(&self, id: &str) -> Result<Option<File>, String> {
        self.repository.find_by_id(id).await
        // Send domain events
    }
}
