use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct File {
    // Sha256 of the source laps
    pub id: String,
    pub name: String,
    pub complete: bool,
}

impl File {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            complete: false,
        }
    }
}
