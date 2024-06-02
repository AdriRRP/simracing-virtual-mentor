use crate::analysis::domain::analysis::Analysis;

use std::ops::Deref;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Analyses {
    analyses: Vec<Analysis>,
}

impl Deref for Analyses {
    type Target = Vec<Analysis>;

    fn deref(&self) -> &Self::Target {
        &self.analyses
    }
}
