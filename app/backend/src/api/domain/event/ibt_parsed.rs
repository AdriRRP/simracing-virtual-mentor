use crate::shared::domain::event::Event;

use std::fmt::Debug;
use std::any::Any;

#[derive(Debug)]
pub struct IbtParsed {
    pub file_id: String,
}

impl IbtParsed {
    #[must_use]
    pub fn new(file_id: &str) -> Self {
        Self {
            file_id: file_id.to_string(),
        }
    }
}

impl Event for IbtParsed {
    fn id() -> &'static str {
        "ibt_parsed_event"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
