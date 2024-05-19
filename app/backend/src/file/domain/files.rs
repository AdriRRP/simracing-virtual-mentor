use crate::file::domain::file::File;

use std::ops::Deref;

#[derive(Debug)]
pub struct Files {
    files: Vec<File>,
}

impl From<Vec<File>> for Files {
    fn from(files: Vec<File>) -> Self {
        Self { files }
    }
}

impl Deref for Files {
    type Target = Vec<File>;

    fn deref(&self) -> &Self::Target {
        &self.files
    }
}
