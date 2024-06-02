use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents the header of a lap.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Header {
    /// The unique identifier of the lap.
    pub id: Uuid,
    /// The identifier of the source file containing the lap data.
    pub file_id: String,
    /// The lap number.
    pub number: u16,
    /// The driver's name.
    pub driver: String,
    /// The category of the lap.
    pub category: String,
    /// The car associated with the lap.
    pub car: String,
    /// The circuit where the lap was recorded.
    pub circuit: String,
    /// The date and time when the lap was recorded (in UTC).
    pub date: DateTime<Utc>,
    /// The time taken to complete the lap (in seconds).
    pub time: f32,
}

impl Header {
    /// Creates a new instance of `Header`.
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        id: Uuid,
        file_id: String,
        number: u16,
        driver: String,
        category: String,
        car: String,
        circuit: String,
        date: DateTime<Utc>,
        time: f32,
    ) -> Self {
        Self {
            id,
            file_id,
            number,
            driver,
            category,
            car,
            circuit,
            date,
            time,
        }
    }
}
