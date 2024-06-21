use crate::file::domain::file::File;

use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

/// Represents a collection of files.
#[derive(Serialize, Deserialize, Default, Eq, PartialEq, Clone, Debug)]
pub struct Files {
    /// The files in the collection.
    files: Vec<File>,
}

impl From<Vec<File>> for Files {
    /// Converts a vector of files into a `Files` instance.
    ///
    /// # Arguments
    ///
    /// * `files` - The vector of files.
    ///
    /// # Returns
    ///
    /// A `Files` instance containing the provided files.
    fn from(files: Vec<File>) -> Self {
        Self { files }
    }
}

impl Deref for Files {
    type Target = Vec<File>;

    /// Allows accessing the files in the collection via dereferencing.
    ///
    /// # Returns
    ///
    /// A reference to the vector of files.
    fn deref(&self) -> &Self::Target {
        &self.files
    }
}

impl DerefMut for Files {
    /// Allows accessing the files in the collection via mutable dereferencing.
    ///
    /// # Returns
    ///
    /// A mutable reference to the vector of files.
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.files
    }
}
