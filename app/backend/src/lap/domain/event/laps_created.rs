use crate::shared::domain::event::Event;

use std::fmt::Debug;

#[derive(Debug)]
pub struct LapsCreated {
    pub file_id: String,
}

impl LapsCreated {
    #[must_use]
    pub fn new(file_id: &str) -> Self {
        Self {
            file_id: file_id.to_string(),
        }
    }
}

impl Event for LapsCreated {
    fn id() -> &'static str {
        "laps_created_event"
    }
}
