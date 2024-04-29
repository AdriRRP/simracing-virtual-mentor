use crate::telemetry::domain::repository::Repository;
use crate::telemetry::domain::session::Session;
use crate::telemetry::domain::player::Player;
use crate::telemetry::domain::circuit::Circuit;
use crate::telemetry::domain::lap::Lap;

pub struct InMemory {
    memory: Vec<Session>,
}

impl Default for InMemory {
    fn default() -> Self {
        Self {
            memory: vec![
                Session {
                    id: String::from("00000"),
                    name: String::from("Test Session"),
                    circuit: Circuit {
                        name: String::from("Test Circuit"),
                    },
                    players: vec![
                        Player {
                            id: String::from("P01"),
                            name: String::from("GregoryHH"),
                            laps: vec![
                                Lap {
                                    number: 1,
                                    distances: vec![1.0, 2.1, 3.2, 4.3, 5.4, 6.5, 7.6, 8.7, 9.8, 11.1],
                                    velocity: vec![105.0, 123.0, 150.0, 178.0, 200.0, 120.0, 80.0, 40.0, 100.0, 180.0],
                                }
                            ],
                        },
                        Player {
                            id: String::from("P02"),
                            name: String::from("Apocalypses"),
                            laps: vec![
                                Lap {
                                    number: 1,
                                    distances: vec![1.0, 2.1, 3.2, 4.3, 5.4, 6.5, 7.6, 8.7, 9.8, 11.1],
                                    velocity: vec![115.0, 143.0, 170.0, 175.0, 180.0, 170.0, 160.0, 100.0, 80.0, 120.0],
                                }
                            ],
                        },
                    ],
                }
            ]
        }
    }
}

impl Repository for InMemory {
    fn find_all(self) -> Vec<Session> {
        return self.memory;
    }

    fn find_by_id(self, id: String) -> Option<Session> {
        return self.memory.iter().find(|s| { s.id == id }).cloned();
    }
}