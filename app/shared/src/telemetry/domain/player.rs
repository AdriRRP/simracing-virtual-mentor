use crate::telemetry::domain::lap::Lap;

#[derive(Clone, PartialEq)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub laps: Vec<Lap>,
}