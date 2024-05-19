pub mod metrics;

use crate::lap::domain::lap::metrics::Metrics;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Lap {
    pub id: Uuid,
    // Source laps
    pub file_id: String,
    pub number: u16,
    pub driver: String,
    pub category: String,
    pub car: String,
    pub circuit: String,
    pub date: DateTime<Utc>,
    pub metrics: Metrics,
}

impl Lap {
    pub fn new(
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
        }
    }
}
