use crate::lap::domain::lap::Lap;
use std::ops::Deref;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Laps {
    laps: Vec<Lap>,
}

impl Laps {
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
