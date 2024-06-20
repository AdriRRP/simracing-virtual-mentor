pub mod bus;
pub mod subscriber;

use std::any::Any;
use std::fmt::Debug;

/// Trait representing an file.
pub trait Event: Debug + Send + Sync + 'static {
    /// Returns the ID of the file type.
    ///
    /// By default, it returns the type name of the file.
    #[must_use]
    fn event_id() -> &'static str
    where
        Self: Sized,
    {
        std::any::type_name::<Self>()
    }

    /// Returns the ID of the specific file instance.
    ///
    /// By default, it returns the type name of the file.
    fn id(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    /// Returns a reference to this file as `dyn Any`.
    fn as_any(&self) -> &dyn Any;
}
