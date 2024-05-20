use crate::lap::domain::lap::Lap;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Serialize, Deserialize, Debug)]
pub struct Laps {
    laps: Vec<Lap>,
}

impl Laps {
    #[must_use]
    pub fn new(laps: Vec<Lap>) -> Self {
        Self { laps }
    }
}

impl Deref for Laps {
    type Target = Vec<Lap>;

    fn deref(&self) -> &Self::Target {
        &self.laps
    }
}
