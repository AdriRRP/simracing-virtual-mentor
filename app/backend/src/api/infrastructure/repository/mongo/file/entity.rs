use shared::file::domain::file::{File, Status};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Entity {
    // id is moved out of header to identify analysis as an entire document
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    pub status: Status,
    pub created_on: bson::DateTime,
}
impl TryInto<File> for Entity {
    type Error = String;

    fn try_into(self) -> Result<File, Self::Error> {
        Ok(File {
            id: self.id.to_string(),
            name: self.name,
            status: self.status,
            created_on: self.created_on.to_chrono(),
        })
    }
}

impl TryFrom<File> for Entity {
    type Error = String;

    fn try_from(file: File) -> Result<Self, Self::Error> {
        let created_on =
            bson::DateTime::parse_rfc3339_str(file.created_on.to_rfc3339()).map_err(|e| {
                format!(
                    "chrono::DateTime {} cannot be cast to bson::DateTime: {e}",
                    file.created_on
                )
            })?;
        Ok(Self {
            id: file.id,
            name: file.name,
            status: file.status,
            created_on,
        })
    }
}
