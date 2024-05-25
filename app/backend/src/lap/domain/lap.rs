pub mod header;
pub mod headers;
pub mod metrics;

use crate::lap::domain::lap::header::Header;
use crate::lap::domain::lap::metrics::Metrics;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Lap {
    pub header: Header,
    pub metrics: Metrics,
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
        let time = *metrics.lap_current_lap_time.last().unwrap_or(&0f32);
        let header = Header::new(
            id, file_id, number, driver, category, car, circuit, date, time,
        );

        Self { header, metrics }
    }
}
