use crate::analysis::domain::analysis::Analysis;
use crate::analysis::domain::repository::Repository;
use crate::lap::domain::repository::Repository as LapRepository;

use std::sync::Arc;
use uuid::Uuid;

/// A struct responsible for asynchronously creating analysis data.
pub struct Creator<R: Repository, LR: LapRepository> {
    repository: Arc<R>,
    lap_repository: Arc<LR>,
}

impl<R: Repository, LR: LapRepository> Creator<R, LR> {
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
    pub fn new(repository: Arc<R>, lap_repository: Arc<LR>) -> Self {
        Self {
            repository,
            lap_repository,
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

        let analysis =
            Analysis::analyze(id, name, ref_lap, target_lap).map_err(|e| e.to_string())?;

        self.repository.create(analysis).await
        // Optionally, you might want to trigger domain events here.
    }
}
