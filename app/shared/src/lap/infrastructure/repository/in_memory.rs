use crate::lap::domain::lap::Lap;
use crate::lap::domain::laps::Laps;
use crate::lap::domain::repository::Repository;

use crate::lap::domain::lap::header::Header;
use crate::lap::domain::lap::headers::Headers;
use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Debug)]
pub struct InMemory {
    laps: Arc<Mutex<Vec<Lap>>>,
}

impl Default for InMemory {
    fn default() -> Self {
        Self {
            laps: Arc::new(Mutex::new(Vec::default())),
        }
    }
}

#[async_trait]
impl Repository for InMemory {
    async fn create(&self, laps: Laps) {
        let mut laps_guard = self.laps.lock().unwrap();
        laps_guard.extend(laps.clone());
        drop(laps_guard);
    }

    async fn delete(&self, id: &Uuid) -> Result<(), String> {
        let mut laps_guard = self.laps.lock().unwrap();
        let i = laps_guard
            .iter()
            .position(|lap| lap.header.id == *id)
            .ok_or(format!("No lap with {id} found."))?;
        laps_guard.remove(i);
        drop(laps_guard);
        Ok(())
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Lap>, String> {
        let laps_guard = self.laps.lock().unwrap();
        let result = laps_guard.iter().find(|lap| lap.header.id == *id).cloned();
        drop(laps_guard);
        Ok(result)
    }

    async fn find_by_criteria(&self, _criteria: &str) -> Result<Option<Laps>, String> {
        let laps_guard = self.laps.lock().unwrap();
        let laps: Laps = laps_guard.clone().into();
        drop(laps_guard);
        let opt_laps: Option<Laps> = if laps.len() == 0 { None } else { Some(laps) };
        Ok(opt_laps)
    }

    async fn find_header_by_id(&self, id: &Uuid) -> Result<Option<Header>, String> {
        self.find_by_id(id)
            .await
            .map(|opt_lap| opt_lap.map(|lap| lap.header))
    }

    async fn find_header_by_criteria(&self, criteria: &str) -> Result<Option<Headers>, String> {
        self.find_by_criteria(criteria).await.map(|opt_laps| {
            opt_laps.map(|laps| laps.clone().into_iter().map(|lap| lap.header).collect())
        })
    }
}
