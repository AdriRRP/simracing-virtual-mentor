use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct File {
    // Sha256 of the source bytes
    pub id: String,
    pub name: String,
    pub complete: bool,
}

impl File {
    #[must_use]
    pub const fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            complete: false,
        }
    }
}
