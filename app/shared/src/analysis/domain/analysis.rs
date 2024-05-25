use crate::lap::domain::lap::Lap;

use uuid::Uuid;

#[derive(Clone)]
pub struct Analysis {
    pub id: Uuid,
    pub name: String,
    pub laps: Vec<Lap>,
}
