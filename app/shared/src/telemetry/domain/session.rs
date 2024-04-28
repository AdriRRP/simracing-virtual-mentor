use crate::telemetry::domain::circuit::Circuit;
use crate::telemetry::domain::player::Player;

#[derive(Clone, PartialEq)]
pub struct Session {
    pub id: String,
    pub name: String,
    pub circuit: Circuit,
    pub players: Vec<Player>,
}