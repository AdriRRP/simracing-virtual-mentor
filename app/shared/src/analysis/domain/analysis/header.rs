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
}

impl Header {
    #[must_use]
    pub const fn new(id: Uuid, name: String, date: DateTime<Utc>, circuit: String) -> Self {
        Self {
            id,
            name,
            date,
            circuit,
        }
    }
}
