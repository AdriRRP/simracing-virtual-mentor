use crate::lap::domain::lap::metric::Metric;

use std::ops::Deref;

#[derive(Clone)]
pub struct Metrics {
    metrics: Vec<Metric>,
}

impl Deref for Metrics {
    type Target = Vec<Metric>;

    fn deref(&self) -> &Self::Target {
        &self.metrics
    }
}
