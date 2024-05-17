pub mod metric;
pub mod metrics;

use crate::lap::domain::lap::metrics::Metrics;

use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone)]
pub struct Lap {
    pub id: Uuid,
    // Source file
    pub file_id: Uuid,
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
        file_id: Uuid,
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
