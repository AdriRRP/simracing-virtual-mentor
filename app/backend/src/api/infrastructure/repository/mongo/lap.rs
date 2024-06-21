pub mod entity;

use crate::api::infrastructure::repository::mongo::lap::entity::Entity;
use crate::api::infrastructure::repository::mongo::Mongo as MongoTrait;
use crate::api::infrastructure::settings::Settings;

use shared::common::domain::criteria::Criteria;
use shared::lap::domain::lap::header::Header;
use shared::lap::domain::lap::headers::Headers;
use shared::lap::domain::lap::Lap;
use shared::lap::domain::laps::Laps;
use shared::lap::domain::repository::Repository;

use async_trait::async_trait;
use bson::{doc, Bson};
use mongodb::Collection;
use uuid::Uuid;

pub struct Mongo {
    collection: Collection<Entity>,
}

impl MongoTrait<Lap, Entity> for Mongo {
    fn map_criteria_field(name: &str, value: &str) -> Result<Bson, String> {
        match name {
            "id" => bson::Uuid::parse_str(value)
                .map(Bson::from)
                .map_err(|e| e.to_string()),
            "file_id" | "driver" | "category" | "car" | "circuit" | "number" | "time" => {
                Ok(Bson::from(value))
            }
            "date" => bson::DateTime::parse_rfc3339_str(value)
                .map_err(|e| e.to_string())
                .map(Bson::from),
            unknown => Err(format!(
                "Field `{unknown}` not registered in criteria search"
            )),
        }
    }
}

impl Mongo {
    /// # Errors
    ///
    /// This function will return an error in the following situations:
    ///
    /// * If the collection cannot be obtained due to an issue with the database connection or
    ///   configuration, an error string describing the problem will be returned.
    pub async fn new(settings: &Settings) -> Result<Self, String> {
        let collection = Self::get_collection(settings, &settings.mongo.collections.lap)
            .await
            .map_err(|e| e.to_string())?;
        Ok(Self { collection })
    }
}

#[async_trait]
impl Repository for Mongo {
    // TODO: &Laps instead of Laps
    async fn create(&self, laps: Laps) -> Result<(), String> {
        let entities = laps
            .iter()
            .cloned()
            .map(Entity::try_from)
            .collect::<Result<Vec<Entity>, String>>()?;

        let _ = self
            .collection
            .insert_many(entities, None)
            .await
            .map_err(|e| format!("{e}"))?;

        Ok(())
    }

    async fn delete(&self, id: &Uuid) -> Result<(), String> {
        let bson_id = bson::Uuid::parse_str(id.to_string())
            .map_err(|e| format!("Error parsing uuid: {e}"))?;
        let filter = doc! { "_id": bson_id };
        let _ = self
            .collection
            .delete_one(filter, None)
            .await
            .map_err(|e| format!("{e}"))?;
        Ok(())
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Lap>, String> {
        let bson_id = bson::Uuid::parse_str(id.to_string())
            .map_err(|e| format!("Error parsing uuid: {e}"))?;
        let query = Some(doc! { "_id": bson_id });
        let result = self.query(&self.collection, query, None).await;
        let analyses: Vec<Lap> = result.map_err(|e| format!("{e}"))?;
        let found = analyses.first().cloned();
        Ok(found)
    }

    async fn find_by_criteria(&self, criteria: &Criteria) -> Result<Option<Laps>, String> {
        self.query_criteria(&self.collection, criteria)
            .await
            .map(|v| Some(Laps::from(v)))
            .map_err(|e| e.to_string())
    }

    async fn find_header_by_id(&self, id: &Uuid) -> Result<Option<Header>, String> {
        self.find_by_id(id).await.map(|opt| opt.map(|a| a.header))
    }

    async fn find_header_by_criteria(
        &self,
        criteria: &Criteria,
    ) -> Result<Option<Headers>, String> {
        self.query_criteria(&self.collection, criteria)
            .await
            .map(|v| {
                Some(Headers::from(
                    v.iter().map(|a| a.header.clone()).collect::<Vec<Header>>(),
                ))
            })
            .map_err(|e| e.to_string())
    }
}
