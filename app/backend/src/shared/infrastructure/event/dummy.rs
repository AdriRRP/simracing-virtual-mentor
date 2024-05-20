use std::any::Any;
use crate::shared::domain::event::Event;

#[derive(Debug)]
pub struct Dummy {}

impl Event for Dummy {
    fn id() -> &'static str {
        "dummy_event"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
