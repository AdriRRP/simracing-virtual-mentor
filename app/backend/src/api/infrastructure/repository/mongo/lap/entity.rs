use crate::shared::lap::domain::lap::Lap;

use shared::lap::domain::lap::header::Header as DomainHeader;
use shared::lap::domain::lap::variables::Variables;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Entity {
    // id is moved out of header to identify analysis as an entire document
    #[serde(rename = "_id")]
    pub id: bson::uuid::Uuid,
    pub file_id: String,
    pub number: u16,
    pub driver: String,
    pub category: String,
    pub car: String,
    pub circuit: String,
    pub date: bson::DateTime,
    pub time: f32,
    pub variables: Variables,
}

impl TryInto<Lap> for Entity {
    type Error = String;

    fn try_into(self) -> Result<Lap, Self::Error> {
        let id = Uuid::parse_str(&self.id.to_string()).map_err(|e| e.to_string())?;
        Ok(Lap {
            header: DomainHeader {
                id,
                file_id: self.file_id,
                number: self.number,
                driver: self.driver,
                category: self.category,
                date: self.date.to_chrono(),
                circuit: self.circuit,
                car: self.car,
                time: self.time,
            },
            variables: self.variables,
        })
    }
}

impl TryFrom<Lap> for Entity {
    type Error = String;

    fn try_from(lap: Lap) -> Result<Self, Self::Error> {
        let id = bson::Uuid::parse_str(lap.header.id.to_string()).map_err(|e| {
            format!(
                "uuid::Uuid {} cannot be cast to bson::Uuid: {e}",
                lap.header.id
            )
        })?;
        let date =
            bson::DateTime::parse_rfc3339_str(lap.header.date.to_rfc3339()).map_err(|e| {
                format!(
                    "chrono::DateTime {} cannot be cast to bson::DateTime: {e}",
                    lap.header.date
                )
            })?;
        Ok(Self {
            id,
            file_id: lap.header.file_id,
            number: lap.header.number,
            driver: lap.header.driver,
            category: lap.header.category,
            date,
            circuit: lap.header.circuit,
            car: lap.header.car,
            time: lap.header.time,
            variables: lap.variables,
        })
    }
}
