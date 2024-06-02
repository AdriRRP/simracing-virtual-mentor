use crate::lap::domain::lap::Lap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents an analysis entity, containing an identifier, name, and a list of laps.
#[derive(Serialize, Deserialize, Clone)]
pub struct Analysis {
    /// The unique identifier for the analysis.
    pub id: Uuid,
    /// The name of the analysis.
    pub name: String,
    /// The list of laps associated with the analysis.
    pub laps: Vec<Lap>,
}
