use crate::analysis::domain::analyses::Analyses;
use crate::analysis::domain::analysis::Analysis;

use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait Repository: Send + Sync {
    async fn create(&self, analysis: Analysis) -> Result<(), String>;
    async fn delete(&self, id: &Uuid) -> Result<(), String>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Analysis>, String>;
    async fn find_by_criteria(&self, criteria: &str) -> Result<Option<Analyses>, String>;
}
