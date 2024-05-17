use crate::lap::domain::lap::Lap;
use crate::lap::domain::laps::Laps;
use crate::lap::domain::repository::Repository;

use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

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
    async fn create(&self, lap: Lap) {
        let mut laps_guard = self.laps.lock().unwrap();
        laps_guard.push(lap);
        drop(laps_guard);
    }

    async fn delete(&self, id: &Uuid) -> Result<(), String> {
        let mut laps_guard = self.laps.lock().unwrap();
        let i = laps_guard
            .iter()
            .position(|lap| lap.id == *id)
            .ok_or(format!("No lap with {id} found."))?;
        laps_guard.remove(i);
        drop(laps_guard);
        Ok(())
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Lap>, String> {
        let laps_guard = self.laps.lock().unwrap();
        let result = laps_guard.iter().find(|lap| lap.id == *id).cloned();
        drop(laps_guard);
        Ok(result)
    }

    async fn find_by_criteria(&self, _criteria: &String) -> Result<Option<Laps>, String> {
        todo!()
    }
}
