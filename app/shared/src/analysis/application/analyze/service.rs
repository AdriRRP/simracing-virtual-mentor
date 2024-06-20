use crate::analysis::domain::analysis::status::Status;
use crate::analysis::domain::repository::Repository;
use crate::lap::domain::repository::Repository as LapRepository;

use std::sync::Arc;
use uuid::Uuid;

/// A struct responsible for asynchronously perform an analysis.
pub struct Analyzer<R: Repository, LR: LapRepository> {
    repository: Arc<R>,
    lap_repository: Arc<LR>,
}

impl<R: Repository, LR: LapRepository> Analyzer<R, LR> {
    /// Creates a new `Analyzer` instance.
    ///
    /// # Parameters
    ///
    /// - `repository`: An asynchronous repository for analysis operations.
    /// - `lap_repository`: An asynchronous repository for lap operations.
    ///
    /// # Returns
    ///
    /// A new `Analyzer` instance.
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
    pub async fn analyze(&self, id: Uuid) -> Result<(), String> {
        let mut analysis = self.repository.find_by_id(&id).await?.ok_or(format!(
            "Cannot found analysis with id `{id}` to perform an analysis"
        ))?;

        if let Status::Pending { ref_id, target_id } = analysis.header.status {
            let ref_lap = self
                .lap_repository
                .find_by_id(&ref_id)
                .await?
                .ok_or(format!("Reference Lap with id {ref_id} not found"))?;

            let target_lap = self
                .lap_repository
                .find_by_id(&target_id)
                .await?
                .ok_or(format!("Target Lap with id {target_id} not found"))?;

            analysis
                .analyze(ref_lap, target_lap)
                .map_err(|e| format!("{e}"))?;

            self.repository.update(&analysis).await
        } else {
            Err(format!(
                "Analysis `{}` ({}) cannot be analyzed in `{:?}` status",
                analysis.header.name, analysis.header.id, analysis.header.status
            ))
        }

        // Optionally, you might want to trigger domain events here.
    }
}
