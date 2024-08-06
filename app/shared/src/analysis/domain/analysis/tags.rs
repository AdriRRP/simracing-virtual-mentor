use crate::analysis::domain::analysis::tag::Tag;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

/// Represents a collection of tags.
#[derive(Serialize, Deserialize, PartialEq, Eq, Default, Clone, Debug)]
pub struct Tags {
    tags: Vec<Tag>,
}

impl From<Vec<Tag>> for Tags {
    /// Converts a vector of tags into `Tags`.
    fn from(tags: Vec<Tag>) -> Self {
        Self { tags }
    }
}

impl FromIterator<Tag> for Tags {
    /// Constructs `Tags` from an iterator over tags.
    fn from_iter<T: IntoIterator<Item = Tag>>(iter: T) -> Self {
        let tags_vec: Vec<Tag> = Vec::from_iter(iter);
        Self::from(tags_vec)
    }
}

impl Deref for Tags {
    type Target = Vec<Tag>;

    /// Implements dereferencing for `Tags`.
    fn deref(&self) -> &Self::Target {
        &self.tags
    }
}

impl DerefMut for Tags {
    /// Allows accessing the analyses in the collection via mutable dereferencing.
    ///
    /// # Returns
    ///
    /// A mutable reference to the vector of tags.
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tags
    }
}
