use crate::file::domain::file::File;

use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Serialize, Deserialize, Debug)]
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
