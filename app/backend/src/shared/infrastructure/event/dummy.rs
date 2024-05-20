use crate::shared::domain::event::Event;

#[derive(Debug)]
pub struct Dummy {}

impl Event for Dummy {
    fn id() -> &'static str {
        "dummy_event"
    }
}
