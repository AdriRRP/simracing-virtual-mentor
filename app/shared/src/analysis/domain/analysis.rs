use crate::lap::domain::lap::Lap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Analysis {
    pub id: Uuid,
    pub name: String,
    pub laps: Vec<Lap>,
}
