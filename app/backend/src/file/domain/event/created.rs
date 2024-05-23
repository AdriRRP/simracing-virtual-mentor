use crate::file::domain::file::File;
use crate::shared::domain::event::Event;
use std::any::Any;

use std::fmt::Debug;

#[derive(Debug)]
pub struct Created {
    pub file: File,
}

impl Created {
    #[must_use]
    pub fn new(file: &File) -> Self {
        Self { file: file.clone() }
    }
}

impl Event for Created {
    fn id() -> &'static str {
        "file_created_event"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
