use crate::file::domain::file::File;

use std::ops::Deref;

pub struct Files {
    files: Vec<File>,
}

impl Deref for Files {
    type Target = Vec<File>;

    fn deref(&self) -> &Self::Target {
        &self.files
    }
}
