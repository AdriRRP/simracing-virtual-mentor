use serde::{Deserialize, Serialize};

/// Represents a file.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct File {
    /// The SHA256 hash of the source bytes.
    pub id: String,
    /// The name of the file.
    pub name: String,
    /// Indicates whether the file is complete.
    pub complete: bool,
}

impl File {
    /// Creates a new `File` instance with the specified ID and name.
    ///
    /// # Arguments
    ///
    /// * `id` - The SHA256 hash of the source bytes.
    /// * `name` - The name of the file.
    ///
    /// # Returns
    ///
    /// A new `File` instance.
    #[must_use]
    pub const fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            complete: false,
        }
    }

    /// Marks the file as complete.
    pub fn validate(&mut self) {
        self.complete = true;
    }
}
