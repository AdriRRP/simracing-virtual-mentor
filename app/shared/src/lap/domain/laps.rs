use crate::lap::domain::lap::Lap;

use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Serialize, Deserialize, Debug)]
pub struct Laps {
    laps: Vec<Lap>,
}

impl From<Vec<Lap>> for Laps {
    fn from(laps: Vec<Lap>) -> Self {
        Self { laps }
    }
}

impl FromIterator<Lap> for Laps {
    fn from_iter<T: IntoIterator<Item = Lap>>(iter: T) -> Self {
        let laps_vec: Vec<Lap> = Vec::from_iter(iter);
        Self::from(laps_vec)
    }
}

impl Deref for Laps {
    type Target = Vec<Lap>;

    fn deref(&self) -> &Self::Target {
        &self.laps
    }
}
