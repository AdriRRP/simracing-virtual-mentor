pub mod bus;
pub mod subscriber;

use std::any::Any;
use std::fmt::Debug;

/// Trait representing an event.
pub trait Event: Debug + Send + Sync + 'static {
    /// Returns the ID of the event type.
    ///
    /// By default, it returns the type name of the event.
    #[must_use]
    fn event_id() -> &'static str
    where
        Self: Sized,
    {
        std::any::type_name::<Self>()
    }

    /// Returns the ID of the specific event instance.
    ///
    /// By default, it returns the type name of the event.
    fn id(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    /// Returns a reference to this event as `dyn Any`.
    fn as_any(&self) -> &dyn Any;
}
