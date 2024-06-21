use crate::common::domain::event::Event;

use std::any::Any;
use std::fmt::Debug;
use uuid::Uuid;

/// Represents the "created" event for an analysis.
#[derive(Debug)]
pub struct Created {
    /// The created analysis's header.
    pub id: Uuid,
}

impl Created {
    /// Creates a new `Created` event.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the analysis that was created.
    ///
    /// # Returns
    ///
    /// A new `Created` event.
    #[must_use]
    pub const fn new(id: &Uuid) -> Self {
        Self { id: *id }
    }
}

impl Event for Created {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
