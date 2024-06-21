use crate::analysis::domain::analysis::Analysis;
use crate::analysis::domain::repository::Repository;
use crate::common::domain::event::bus::Bus as EventBus;
use crate::lap::domain::repository::Repository as LapRepository;

use crate::analysis::domain::event::created::Created;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use uuid::Uuid;

/// A struct responsible for asynchronously creating analysis data.
pub struct Creator<R: Repository, LR: LapRepository, E: EventBus> {
    repository: Arc<R>,
    lap_repository: Arc<LR>,
    event_bus: Arc<E>,
}

impl<R: Repository, LR: LapRepository, E: EventBus> Creator<R, LR, E> {
    /// Creates a new `Creator` instance.
    ///
    /// # Parameters
    ///
    /// - `repository`: An asynchronous repository for analysis operations.
    /// - `lap_repository`: An asynchronous repository for lap operations.
    ///
    /// # Returns
    ///
    /// A new `Creator` instance.
    pub fn new(repository: Arc<R>, lap_repository: Arc<LR>, event_bus: Arc<E>) -> Self {
        Self {
            repository,
            lap_repository,
            event_bus,
        }
    }

    /// Asynchronously creates analysis data using the repositories.
    ///
    /// # Parameters
    ///
    /// - `id`: The UUID identifier for the analysis.
    /// - `name`: The name of the analysis.
    /// - `ref_lap_id`: The UUID identifier of the reference lap.
    /// - `target_lap_id`: The UUID identifier of the target lap.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if either the reference lap or target lap is not found in the repository,
    /// or if the repository fails during the creation process.
    ///
    /// # Returns
    ///
    /// Returns `Ok` if the analysis data was successfully created and stored in the repository.
    pub async fn create(
        &self,
        id: Uuid,
        name: String,
        date: DateTime<Utc>,
        ref_lap_id: Uuid,
        target_lap_id: Uuid,
    ) -> Result<(), String> {
        let ref_lap = self
            .lap_repository
            .find_by_id(&ref_lap_id)
            .await?
            .ok_or(format!("Reference Lap with id {ref_lap_id} not found"))?;

        let target_lap = self
            .lap_repository
            .find_by_id(&target_lap_id)
            .await?
            .ok_or(format!("Target Lap with id {target_lap_id} not found"))?;

        if ref_lap.header.circuit != target_lap.header.circuit {
            return Err("the laps in an analysis have to belong to the same circuit".to_owned());
        }

        let analysis = Analysis::new(
            id,
            name,
            date,
            ref_lap.header.circuit.clone(),
            ref_lap_id,
            target_lap_id,
        );

        match self.repository.create(analysis).await {
            Ok(()) => {
                let event = Arc::new(Created::new(&id));
                self.event_bus.dispatch(event).await
            }
            e => e,
        }
    }
}
