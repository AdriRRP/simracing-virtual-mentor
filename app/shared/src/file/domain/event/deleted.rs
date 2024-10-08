use crate::common::domain::event::Event;

use std::any::Any;
use std::fmt::Debug;

/// Represents the "deleted" file for a file.
#[derive(Debug)]
pub struct Deleted {
    /// The ID of the deleted file.
    pub id: String,
}

impl Deleted {
    /// Creates a new `Deleted` file with the given ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the deleted file.
    ///
    /// # Returns
    ///
    /// A new `Deleted` file.
    #[must_use]
    pub fn new(id: &str) -> Self {
        Self { id: id.to_string() }
    }
}

impl Event for Deleted {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
