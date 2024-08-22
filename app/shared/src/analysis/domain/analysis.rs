pub mod clusters_memberships;
pub mod differences;
pub mod distances;
mod fcm_grid;
pub mod fuzzy_c_means;
pub mod header;
pub mod headers;
pub mod interpolation;
pub mod reference_lap;
pub mod status;
pub mod tag;
pub mod tags;

use crate::analysis::domain::analysis::clusters_memberships::ClustersMemberships;
use crate::analysis::domain::analysis::differences::calculate as calculate_differences;
use crate::analysis::domain::analysis::distances::generate_union as generate_union_distances;
use crate::analysis::domain::analysis::header::Header;
use crate::analysis::domain::analysis::interpolation::interpolate_variables;
use crate::analysis::domain::analysis::reference_lap::ReferenceLap;
use crate::analysis::domain::analysis::status::Status;
use crate::analysis::domain::analysis::tags::Tags;
use crate::lap::domain::lap::variables::Variables;
use crate::lap::domain::lap::Lap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents an analysis entity, containing an identifier, name, and a list of laps.
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Analysis {
    /// Information about analysis
    pub header: Header,

    /// Laps to compare each other
    pub reference: Option<ReferenceLap>,
    pub target: Option<ReferenceLap>,

    /// Interpolated common distance
    pub union_distances: Vec<f32>,

    /// Difference metrics: reference - target
    pub differences: Option<Variables>,

    /// Clustering results
    pub clustering: Option<ClustersMemberships>,

    /// Assigned tags for differences
    pub tags: Option<Tags>,
}

impl Analysis {
    #[must_use]
    pub fn new(
        id: Uuid,
        name: String,
        date: DateTime<Utc>,
        circuit: String,
        ref_lap_id: Uuid,
        target_lap_id: Uuid,
    ) -> Self {
        Self {
            header: Header::new(id, name, date, circuit, ref_lap_id, target_lap_id),
            reference: None,
            target: None,
            union_distances: vec![],
            differences: None,
            clustering: None,
            tags: None,
        }
    }

    #[must_use]
    pub fn with_error(
        id: Uuid,
        name: String,
        date: DateTime<Utc>,
        circuit: String,
        error_msg: String,
    ) -> Self {
        Self {
            header: Header::with_error(id, name, date, circuit, error_msg),
            reference: None,
            target: None,
            union_distances: vec![],
            differences: None,
            clustering: None,
            tags: None,
        }
    }

    /// # Errors
    ///
    /// This function returns an error if:
    ///
    /// * The reference lap and the target lap are from different circuits. In this case,
    ///   an `Error::DifferentCircuits` is returned with the circuit information of both laps.
    /// * Any error occurs during the interpolation of metrics. This might be due to issues
    ///   with the data points not aligning properly between the reference and target laps.
    ///
    /// # Note
    ///
    /// Ensure that both `ref_lap` and `target_lap` are from the same circuit to avoid the `DifferentCircuits` error.
    ///
    pub fn analyze(&mut self, ref_lap: Lap, target_lap: Lap) -> Result<(), Error> {
        if ref_lap.header.circuit != target_lap.header.circuit {
            return Err(Error::DifferentCircuits(
                ref_lap.header.circuit,
                target_lap.header.circuit,
            ));
        }

        let union_distances = generate_union_distances(&ref_lap, &target_lap);
        let ref_variables = interpolate_variables(&ref_lap.variables, &union_distances)?;
        let target_variables = interpolate_variables(&target_lap.variables, &union_distances)?;
        let differences = calculate_differences(&ref_variables, &target_variables);

        self.reference = Some(ReferenceLap::new(
            ref_lap.header.number,
            ref_lap.header.driver.clone(),
            ref_lap.header.category.clone(),
            ref_lap.header.car,
            ref_variables,
        ));

        self.target = Some(ReferenceLap::new(
            target_lap.header.number,
            target_lap.header.driver.clone(),
            target_lap.header.category.clone(),
            target_lap.header.car,
            target_variables,
        ));

        self.union_distances = union_distances;
        self.differences = Some(differences.clone());

        self.header.status = Status::Completed;

        // Clustering
        let clustering =
            ClustersMemberships::try_transform_and_fit(&differences).map_err(Error::Clustering)?;
        self.clustering = Some(clustering);

        Ok(())
    }
}

#[derive(PartialEq, Eq, Debug, thiserror::Error)]
pub enum Error {
    #[error("the reference lap has been run on Circuit `{0}` while the target lap has been run on Circuit `{0}`")]
    DifferentCircuits(String, String),
    #[error("error interpolating variables: {0}")]
    InterpolatingVariables(String),
    #[error("cannot build an analysis with given header: {0}")]
    FromHeader(String),
    #[error("cannot perform clusters_memberships: {0}")]
    Clustering(String),
}
