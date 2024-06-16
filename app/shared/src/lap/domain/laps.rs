use crate::lap::domain::lap::Lap;
use serde::{Deserialize, Serialize};
use std::iter::FromIterator;
use std::ops::Deref;

/// Represents a collection of laps.
#[derive(Serialize, Deserialize, PartialEq, Default, Clone, Debug)]
pub struct Laps {
    laps: Vec<Lap>,
}

impl From<Vec<Lap>> for Laps {
    /// Converts a vector of laps into `Laps`.
    fn from(laps: Vec<Lap>) -> Self {
        Self { laps }
    }
}

impl FromIterator<Lap> for Laps {
    /// Constructs `Laps` from an iterator over laps.
    fn from_iter<T: IntoIterator<Item = Lap>>(iter: T) -> Self {
        let laps_vec: Vec<Lap> = Vec::from_iter(iter);
        Self::from(laps_vec)
    }
}

impl Deref for Laps {
    type Target = Vec<Lap>;

    /// Implements dereferencing for `Laps`.
    fn deref(&self) -> &Self::Target {
        &self.laps
    }
}
