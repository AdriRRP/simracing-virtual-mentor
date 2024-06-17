use crate::analysis::domain::analysis::Analysis;

use serde::{Deserialize, Serialize};
use std::ops::Deref;

/// A wrapper struct for a vector of analysis objects.
#[derive(Serialize, Deserialize, Clone)]
pub struct Analyses {
    analyses: Vec<Analysis>,
}

impl From<Vec<Analysis>> for Analyses {
    /// Converts a vector of laps into `Laps`.
    fn from(analyses: Vec<Analysis>) -> Self {
        Self { analyses }
    }
}

impl FromIterator<Analysis> for Analyses {
    /// Constructs `Analyses` from an iterator over analyses.
    fn from_iter<T: IntoIterator<Item = Analysis>>(iter: T) -> Self {
        let analyses_vec: Vec<Analysis> = Vec::from_iter(iter);
        Self::from(analyses_vec)
    }
}

impl Deref for Analyses {
    type Target = Vec<Analysis>;

    /// Returns a reference to the inner vector of analysis objects.
    ///
    /// # Returns
    ///
    /// A reference to the inner vector of analysis objects.
    fn deref(&self) -> &Self::Target {
        &self.analyses
    }
}
