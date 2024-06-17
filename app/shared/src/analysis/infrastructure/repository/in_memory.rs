use crate::analysis::domain::analyses::Analyses;
use crate::analysis::domain::analysis::Analysis;
use crate::analysis::domain::repository::Repository;

use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// An in-memory repository implementation for asynchronous operations.
pub struct InMemory {
    analyses: Arc<Mutex<Vec<Analysis>>>,
}

impl Default for InMemory {
    /// Creates a new `InMemory` instance with an empty list of analyses.
    fn default() -> Self {
        Self {
            analyses: Arc::new(Mutex::new(Vec::default())),
        }
    }
}

#[async_trait]
impl Repository for InMemory {
    /// Creates a new analysis and adds it to the in-memory storage.
    ///
    /// # Errors
    ///
    /// Returns an error if the analysis creation fails.
    async fn create(&self, analysis: Analysis) -> Result<(), String> {
        self.analyses
            .lock()
            .map_err(|e| format!("{e}"))?
            .push(analysis);
        Ok(())
    }

    /// Deletes an analysis with the specified ID from the in-memory storage.
    ///
    /// # Errors
    ///
    /// Returns an error if the analysis deletion fails or if no analysis with the specified ID is found.
    async fn delete(&self, id: &Uuid) -> Result<(), String> {
        let mut analyses_guard = self.analyses.lock().map_err(|e| format!("{e}"))?;
        let i = analyses_guard
            .iter()
            .position(|analysis| analysis.id == *id)
            .ok_or_else(|| format!("No analysis with ID {id} found."))?;

        analyses_guard.remove(i);
        drop(analyses_guard);
        Ok(())
    }

    /// Finds an analysis with the specified ID in the in-memory storage.
    ///
    /// # Errors
    ///
    /// Returns an error if the analysis retrieval fails or if no analysis with the specified ID is found.
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Analysis>, String> {
        let analyses_ward = self.analyses.lock().map_err(|e| format!("{e}"))?;
        let result = analyses_ward
            .iter()
            .find(|analysis| analysis.id == *id)
            .cloned();
        drop(analyses_ward);
        Ok(result)
    }

    /// Finds analyses based on the specified criteria in the in-memory storage.
    ///
    /// # Errors
    ///
    /// Returns an error if the analysis retrieval fails.
    async fn find_by_criteria(&self, _criteria: &str) -> Result<Option<Analyses>, String> {
        let analyses_guard = self.analyses.lock().map_err(|e| format!("{e}"))?;
        let analyses: Analyses = analyses_guard.clone().into();
        drop(analyses_guard);
        let opt_analyses: Option<Analyses> = if analyses.len() == 0 { None } else { Some(analyses) };
        Ok(opt_analyses)
    }
}
