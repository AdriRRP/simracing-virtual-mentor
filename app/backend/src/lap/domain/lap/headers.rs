use crate::lap::domain::lap::header::Header;
use crate::lap::domain::lap::Lap;

use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Serialize, Deserialize, Debug)]
pub struct Headers {
    headers: Vec<Header>,
}

impl From<Vec<Header>> for Headers {
    fn from(headers: Vec<Header>) -> Self {
        Self { headers }
    }
}

impl From<Vec<Lap>> for Headers {
    fn from(laps: Vec<Lap>) -> Self {
        let headers = laps.into_iter().map(|lap| lap.header).collect();
        Self { headers }
    }
}

impl FromIterator<Header> for Headers {
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
