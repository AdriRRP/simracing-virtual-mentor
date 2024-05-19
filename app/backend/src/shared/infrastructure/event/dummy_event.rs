use crate::shared::domain::event::Event;

#[derive(Debug)]
pub struct DummyEvent {}

impl Event for DummyEvent {
    fn id() -> &'static str {
        "dummy_event"
    }
}
