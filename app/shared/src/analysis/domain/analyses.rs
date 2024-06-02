use crate::analysis::domain::analysis::Analysis;

use serde::{Deserialize, Serialize};
use std::ops::Deref;

/// A wrapper struct for a vector of analysis objects.
#[derive(Serialize, Deserialize, Clone)]
pub struct Analyses {
    analyses: Vec<Analysis>,
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
