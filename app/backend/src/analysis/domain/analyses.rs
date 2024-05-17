use crate::analysis::domain::analysis::Analysis;

use std::ops::Deref;

pub struct Analyses {
    analyses: Vec<Analysis>,
}

impl Deref for Analyses {
    type Target = Vec<Analysis>;

    fn deref(&self) -> &Self::Target {
        &self.analyses
    }
}
