mod entity;

use crate::api::infrastructure::repository::mongo::analysis::entity::Entity;
use crate::api::infrastructure::repository::mongo::Mongo as MongoTrait;
use crate::api::infrastructure::settings::Settings;

use shared::analysis::domain::analyses::Analyses;
use shared::analysis::domain::analysis::header::Header;
use shared::analysis::domain::analysis::headers::Headers;
use shared::analysis::domain::analysis::status::Status;
use shared::analysis::domain::analysis::Analysis;
use shared::analysis::domain::repository::Repository;
use shared::common::domain::criteria::Criteria;

use async_trait::async_trait;
use bson::{doc, Bson};
use mongodb::Collection;
use uuid::Uuid;

pub struct Mongo {
    collection: Collection<Entity>,
}

impl MongoTrait<Analysis, Entity> for Mongo {
    fn map_criteria_field(name: &str, value: &str) -> Result<Bson, String> {
        match name {
            "id" => bson::Uuid::parse_str(value)
                .map(Bson::from)
                .map_err(|e| e.to_string()),
            "name" | "circuit" => Ok(Bson::from(value)),
            "date" => bson::DateTime::parse_rfc3339_str(value)
                .map_err(|e| e.to_string())
                .map(Bson::from),
            "status" => {
                let status: Status = serde_json::from_str(value).map_err(|e| e.to_string())?;
                bson::to_bson(&status).map_err(|e| e.to_string())
            }
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
        let collection = Self::get_collection(settings, &settings.mongo.collections.analysis)
            .await
            .map_err(|e| e.to_string())?;
        Ok(Self { collection })
    }
}

#[async_trait]
impl Repository for Mongo {
    async fn create(&self, analysis: Analysis) -> Result<(), String> {
        let entity = Entity::try_from(analysis)?;
        let _ = self
            .collection
            .insert_one(entity, None)
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

    async fn update(&self, analysis: &Analysis) -> Result<(), String> {
        let bson_id = bson::Uuid::parse_str(analysis.header.id.to_string())
            .map_err(|e| format!("Error parsing uuid: {e}"))?;
        let entity = Entity::try_from(analysis.clone())?;
        let filter = doc! { "_id": bson_id };
        let _ = self
            .collection
            .replace_one(filter, entity, None)
            .await
            .map_err(|e| format!("{e}"))?;
        Ok(())
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Analysis>, String> {
        let bson_id = bson::Uuid::parse_str(id.to_string())
            .map_err(|e| format!("Error parsing uuid: {e}"))?;
        let query = Some(doc! { "_id": bson_id });
        let result = self.query(&self.collection, query, None).await;
        let analyses: Vec<Analysis> = result.map_err(|e| format!("{e}"))?;
        let found = analyses.first().cloned();
        Ok(found)
    }

    async fn find_by_criteria(&self, criteria: &Criteria) -> Result<Option<Analyses>, String> {
        self.query_criteria(&self.collection, criteria)
            .await
            .map(|v| Some(Analyses::from(v)))
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
