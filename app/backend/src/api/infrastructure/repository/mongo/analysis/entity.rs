use shared::analysis::domain::analysis::header::Header as DomainHeader;
use shared::analysis::domain::analysis::reference_lap::ReferenceLap;
use shared::analysis::domain::analysis::status::Status;
use shared::analysis::domain::analysis::Analysis;
use shared::lap::domain::lap::variables::Variables;
use shared::analysis::domain::analysis::clustering::Clustering;
use shared::analysis::domain::analysis::tags::Tags;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Entity {
    // id is moved out of header to identify analysis as an entire document
    #[serde(rename = "_id")]
    pub id: bson::uuid::Uuid,
    pub name: String,
    pub date: bson::DateTime,
    pub circuit: String,
    pub status: Status,
    pub reference: Option<ReferenceLap>,
    pub target: Option<ReferenceLap>,
    pub union_distances: Vec<f32>,
    pub differences: Option<Variables>,
    pub clustering: Option<Clustering>,
    pub tags: Option<Tags>,
}
impl TryInto<Analysis> for Entity {
    type Error = String;

    fn try_into(self) -> Result<Analysis, Self::Error> {
        let id = Uuid::parse_str(&self.id.to_string()).map_err(|e| e.to_string())?;
        Ok(Analysis {
            header: DomainHeader {
                id,
                name: self.name,
                date: self.date.to_chrono(),
                circuit: self.circuit,
                status: self.status,
            },
            reference: self.reference,
            target: self.target,
            union_distances: self.union_distances,
            differences: self.differences,
            clustering: self.clustering,
            tags: self.tags,
        })
    }
}

impl TryFrom<Analysis> for Entity {
    type Error = String;

    fn try_from(analysis: Analysis) -> Result<Self, Self::Error> {
        let id = bson::Uuid::parse_str(analysis.header.id.to_string()).map_err(|e| {
            format!(
                "uuid::Uuid {} cannot be cast to bson::Uuid: {e}",
                analysis.header.id
            )
        })?;
        let date =
            bson::DateTime::parse_rfc3339_str(analysis.header.date.to_rfc3339()).map_err(|e| {
                format!(
                    "chrono::DateTime {} cannot be cast to bson::DateTime: {e}",
                    analysis.header.date
                )
            })?;
        Ok(Self {
            id,
            name: analysis.header.name,
            date,
            circuit: analysis.header.circuit,
            status: analysis.header.status,
            reference: analysis.reference,
            target: analysis.target,
            union_distances: analysis.union_distances,
            differences: analysis.differences,
            clustering: analysis.clustering,
            tags: analysis.tags,
        })
    }
}
