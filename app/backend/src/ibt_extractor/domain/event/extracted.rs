use shared::common::domain::event::Event;

use std::any::Any;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Extracted {
    pub file_id: String,
}

impl Extracted {
    #[must_use]
    pub fn new(file_id: &str) -> Self {
        Self {
            file_id: file_id.to_string(),
        }
    }
}

impl Event for Extracted {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
