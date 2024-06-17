use crate::lap::domain::lap::header::Header;
use crate::lap::domain::lap::headers::Headers;
use crate::lap::domain::lap::Lap;
use crate::lap::domain::laps::Laps;
use crate::lap::domain::repository::Repository;
use std::ops::Deref;

use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// A repository implementation storing laps in memory.
#[derive(Debug)]
pub struct InMemory {
    laps: Arc<Mutex<Vec<Lap>>>,
}

impl Default for InMemory {
    /// Creates a new `InMemory` repository with an empty laps collection.
    fn default() -> Self {
        Self {
            laps: Arc::new(Mutex::new(Vec::default())),
        }
    }
}

#[async_trait]
impl Repository for InMemory {
    /// Asynchronously creates laps in memory.
    async fn create(&self, laps: Laps) -> Result<(), String> {
        let mut laps_guard = self.laps.lock().map_err(|e| format!("{e}"))?;
        laps_guard.extend(laps.deref().clone());
        drop(laps_guard);
        Ok(())
    }

    /// Asynchronously deletes laps from memory by ID.
    async fn delete(&self, id: &Uuid) -> Result<(), String> {
        let mut laps_guard = self.laps.lock().map_err(|e| format!("{e}"))?;
        let i = laps_guard
            .iter()
            .position(|lap| lap.header.id == *id)
            .ok_or(format!("No lap with {id} found."))?;
        laps_guard.remove(i);
        drop(laps_guard);
        Ok(())
    }

    /// Asynchronously finds a lap by ID.
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Lap>, String> {
        let laps_guard = self.laps.lock().map_err(|e| format!("{e}"))?;
        let result = laps_guard.iter().find(|lap| lap.header.id == *id).cloned();
        drop(laps_guard);
        Ok(result)
    }

    /// Asynchronously finds laps by criteria.
    async fn find_by_criteria(&self, _criteria: &str) -> Result<Option<Laps>, String> {
        let laps_guard = self.laps.lock().map_err(|e| format!("{e}"))?;
        let laps: Laps = laps_guard.clone().into();
        drop(laps_guard);
        let opt_laps: Option<Laps> = if laps.len() == 0 { None } else { Some(laps) };
        Ok(opt_laps)
    }

    /// Asynchronously finds the header of a lap by ID.
    async fn find_header_by_id(&self, id: &Uuid) -> Result<Option<Header>, String> {
        self.find_by_id(id)
            .await
            .map(|opt_lap| opt_lap.map(|lap| lap.header))
    }

    /// Asynchronously finds headers of laps by criteria.
    async fn find_header_by_criteria(&self, criteria: &str) -> Result<Option<Headers>, String> {
        self.find_by_criteria(criteria).await.map(|opt_laps| {
            opt_laps.map(|laps| laps.deref().iter().map(|lap| lap.header.clone()).collect())
        })
    }
}
