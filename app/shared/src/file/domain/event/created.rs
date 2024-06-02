use crate::common::domain::event::Event;
use crate::file::domain::file::File;

use std::any::Any;
use std::fmt::Debug;

/// Represents the "created" event for a file.
#[derive(Debug)]
pub struct Created {
    /// The created file.
    pub file: File,
}

impl Created {
    /// Creates a new `Created` event.
    ///
    /// # Arguments
    ///
    /// * `file` - The file that was created.
    ///
    /// # Returns
    ///
    /// A new `Created` event.
    #[must_use]
    pub fn new(file: &File) -> Self {
        Self { file: file.clone() }
    }
}

impl Event for Created {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
