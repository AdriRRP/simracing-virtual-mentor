use crate::file::domain::file::File;
use crate::file::domain::files::Files;

use async_trait::async_trait;

#[async_trait]
pub trait Repository: Send + Sync {
    async fn create(&self, file: File);
    async fn delete(&self, id: &str) -> Result<(), String>;
    async fn find_by_id(&self, id: &str) -> Result<Option<File>, String>;
    async fn find_by_criteria(&self, criteria: &str) -> Result<Option<Files>, String>;
}
