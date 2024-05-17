use crate::lap::domain::lap::Lap;
use std::ops::Deref;

pub struct Laps {
    laps: Vec<Lap>,
}

impl Deref for Laps {
    type Target = Vec<Lap>;

    fn deref(&self) -> &Self::Target {
        &self.laps
    }
}
