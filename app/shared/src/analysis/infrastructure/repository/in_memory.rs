use crate::analysis::domain::analyses::Analyses;
use crate::analysis::domain::analysis::Analysis;
use crate::analysis::domain::repository::Repository;

use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub struct InMemory {
    analyses: Arc<Mutex<Vec<Analysis>>>,
}

impl Default for InMemory {
    fn default() -> Self {
        Self {
            analyses: Arc::new(Mutex::new(Vec::default())),
        }
    }
}

#[async_trait]
impl Repository for InMemory {
    async fn create(&self, analysis: Analysis) {
        let mut analyses_guard = self.analyses.lock().unwrap();
        analyses_guard.push(analysis);
        drop(analyses_guard);
    }

    async fn delete(&self, id: &Uuid) -> Result<(), String> {
        let mut analyses_guard = self.analyses.lock().unwrap();
        let i = analyses_guard
            .iter()
            .position(|analysis| analysis.id == *id)
            .ok_or(format!("No analysis with {id} found."))?;
        analyses_guard.remove(i);
        drop(analyses_guard);
        Ok(())
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Analysis>, String> {
        let analyses_guard = self.analyses.lock().unwrap();
        let result = analyses_guard
            .iter()
            .find(|analysis| analysis.id == *id)
            .cloned();
        drop(analyses_guard);
        Ok(result)
    }

    async fn find_by_criteria(&self, _criteria: &str) -> Result<Option<Analyses>, String> {
        todo!()
    }
}
