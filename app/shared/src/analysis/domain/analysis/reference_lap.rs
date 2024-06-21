use crate::lap::domain::lap::metrics::Metrics;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct ReferenceLap {
    pub number: u16,
    pub driver: String,
    pub category: String,
    pub car: String,
    pub metrics: Metrics,
}

impl ReferenceLap {
    #[must_use]
    pub const fn new(
        number: u16,
        driver: String,
        category: String,
        car: String,
        metrics: Metrics,
    ) -> Self {
        Self {
            number,
            driver,
            category,
            car,
            metrics,
        }
    }
}
