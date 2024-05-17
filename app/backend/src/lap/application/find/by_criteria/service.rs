use crate::lap::domain::lap::Lap;
use crate::lap::domain::repository::Repository;

use std::sync::Arc;
use uuid::Uuid;

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
    pub async fn find(&self, id: Uuid) -> Result<Option<Lap>, String> {
        self.repository.find_by_id(&id).await
        // Send domain events
    }
}
