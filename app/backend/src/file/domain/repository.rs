use crate::file::domain::file::File;
use crate::file::domain::files::Files;

use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait Repository: Send + Sync {
    async fn create(&self, file: File);
    async fn delete(&self, id: &Uuid) -> Result<(), String>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<File>, String>;
    async fn find_by_criteria(&self, criteria: &String) -> Result<Option<Files>, String>;
}
