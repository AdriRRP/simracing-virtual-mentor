pub mod metrics;

use crate::lap::domain::lap::metrics::Metrics;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Lap {
    // Lap completa = en ningún punto estás dentro del pit
    pub id: Uuid,
    // Source laps
    pub file_id: String,
    pub number: u16,
    pub driver: String,
    // Customer ID
    pub category: String,
    // En principio da igual
    pub car: String,
    pub circuit: String,
    // Track id para identificar el circuito
    // TrackName + TrackConfig
    pub date: DateTime<Utc>,
    pub metrics: Metrics,
    pub time: f32,
}

impl Lap {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new(
        // TODO: this function has too many arguments (9/7)
        id: Uuid,
        file_id: String,
        number: u16,
        driver: String,
        category: String,
        car: String,
        circuit: String,
        date: DateTime<Utc>,
        metrics: Metrics,
    ) -> Self {
        let time = metrics
            .lap_current_lap_time
            .iter()
            .fold(0f32, |a, &b| a.max(b));

        Self {
            id,
            file_id,
            number,
            driver,
            category,
            car,
            circuit,
            date,
            metrics,
            time,
        }
    }
}
