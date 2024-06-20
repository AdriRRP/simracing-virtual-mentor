use crate::analysis::domain::analysis::status::Status;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct Header {
    /// The unique identifier for the analysis.
    pub id: Uuid,
    /// The name of the analysis.
    pub name: String,
    /// Date on which the analysis was created
    pub date: DateTime<Utc>,
    /// The circuit on which the analysis is performed
    pub circuit: String,
    /// Current status of the analysis
    pub status: Status,
}

impl Header {
    #[must_use]
    pub const fn new(
        id: Uuid,
        name: String,
        date: DateTime<Utc>,
        circuit: String,
        ref_lap_id: Uuid,
        target_lap_id: Uuid,
    ) -> Self {
        Self {
            id,
            name,
            date,
            circuit,
            status: Status::Pending {
                ref_id: ref_lap_id,
                target_id: target_lap_id,
            },
        }
    }

    #[must_use]
    pub const fn with_error(
        id: Uuid,
        name: String,
        date: DateTime<Utc>,
        circuit: String,
        error_msg: String,
    ) -> Self {
        Self {
            id,
            name,
            date,
            circuit,
            status: Status::Error(error_msg),
        }
    }
}
