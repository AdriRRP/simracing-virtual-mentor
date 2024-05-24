use crate::shared::domain::event::Event;

use std::any::Any;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Validated {
    pub id: String,
}

impl Validated {
    #[must_use]
    pub fn new(id: &str) -> Self {
        Self { id: id.to_string() }
    }
}

impl Event for Validated {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
