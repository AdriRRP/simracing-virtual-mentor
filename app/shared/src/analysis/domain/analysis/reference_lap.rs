use crate::lap::domain::lap::variables::Variables;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct ReferenceLap {
    pub number: u16,
    pub driver: String,
    pub category: String,
    pub car: String,
    pub variables: Variables,
}

impl ReferenceLap {
    #[must_use]
    pub const fn new(
        number: u16,
        driver: String,
        category: String,
        car: String,
        variables: Variables,
    ) -> Self {
        Self {
            number,
            driver,
            category,
            car,
            variables,
        }
    }
}
