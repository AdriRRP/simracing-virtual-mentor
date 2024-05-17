use crate::file::domain::repository::Repository;

use std::sync::Arc;
use uuid::Uuid;

pub struct Deleter<R: Repository> {
    repository: Arc<R>,
}

impl<R: Repository> Deleter<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    /// # Errors
    ///
    /// Will return `Err` if `self.repository` fail deleting
    pub async fn delete(&self, id: &Uuid) -> Result<(), String> {
        self.repository.delete(id).await
        // Send domain events
    }
}
