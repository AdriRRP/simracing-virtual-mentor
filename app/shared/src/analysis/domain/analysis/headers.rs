use crate::analysis::domain::analysis::header::Header;
use crate::analysis::domain::analysis::Analysis;

use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

/// Represents a collection of analysis headers.
#[derive(Serialize, Deserialize, Default, Eq, PartialEq, Clone, Debug)]
pub struct Headers {
    /// The vector containing analysis headers.
    headers: Vec<Header>,
}

impl From<Vec<Header>> for Headers {
    /// Converts a vector of analysis headers into `Headers`.
    fn from(headers: Vec<Header>) -> Self {
        Self { headers }
    }
}

impl From<Vec<Analysis>> for Headers {
    /// Converts a vector of analyses into `Headers`, extracting the headers from each analysis.
    fn from(analyses: Vec<Analysis>) -> Self {
        let headers = analyses
            .into_iter()
            .map(|analysis| analysis.header)
            .collect();
        Self { headers }
    }
}

impl FromIterator<Header> for Headers {
    /// Constructs `Headers` from an iterator of analysis headers.
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

impl DerefMut for Headers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.headers
    }
}
