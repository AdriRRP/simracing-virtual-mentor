use crate::common::domain::event::Event;

use std::any::Any;
use std::fmt::Debug;

/// Represents the "validated" file for a resource.
#[derive(Debug)]
pub struct Validated {
    /// The ID of the validated resource.
    pub id: String,
}

impl Validated {
    /// Creates a new `Validated` file with the given ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the validated resource.
    ///
    /// # Returns
    ///
    /// A new `Validated` file.
    #[must_use]
    pub fn new(id: &str) -> Self {
        Self { id: id.to_string() }
    }
}

impl Event for Validated {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
