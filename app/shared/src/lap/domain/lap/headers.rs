use crate::lap::domain::lap::header::Header;
use crate::lap::domain::lap::Lap;

use serde::{Deserialize, Serialize};
use std::ops::Deref;

/// Represents a collection of lap headers.
#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct Headers {
    /// The vector containing lap headers.
    headers: Vec<Header>,
}

impl From<Vec<Header>> for Headers {
    /// Converts a vector of lap headers into `Headers`.
    fn from(headers: Vec<Header>) -> Self {
        Self { headers }
    }
}

impl From<Vec<Lap>> for Headers {
    /// Converts a vector of laps into `Headers`, extracting the headers from each lap.
    fn from(laps: Vec<Lap>) -> Self {
        let headers = laps.into_iter().map(|lap| lap.header).collect();
        Self { headers }
    }
}

impl FromIterator<Header> for Headers {
    /// Constructs `Headers` from an iterator of lap headers.
    fn from_iter<T: IntoIterator<Item = Header>>(iter: T) -> Self {
        let headers_vec = Vec::from_iter(iter);
        Self::from(headers_vec)
    }
}

impl Deref for Headers {
    type Target = Vec<Header>;

    fn deref(&self) -> &Self::Target {
        &self.headers
    }
}
