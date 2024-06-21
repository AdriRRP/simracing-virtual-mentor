pub mod entity;

use crate::api::infrastructure::repository::mongo::file::entity::Entity;
use crate::api::infrastructure::repository::mongo::Mongo as MongoTrait;
use crate::api::infrastructure::settings::Settings;

use shared::common::domain::criteria::Criteria;
use shared::file::domain::file::File;
use shared::file::domain::file::Status;
use shared::file::domain::files::Files;
use shared::file::domain::repository::Repository;

use async_trait::async_trait;
use bson::{doc, Bson};
use mongodb::Collection;

pub struct Mongo {
    collection: Collection<Entity>,
}

impl MongoTrait<File, Entity> for Mongo {
    fn map_criteria_field(name: &str, value: &str) -> Result<Bson, String> {
        match name {
            "id" | "name" => Ok(Bson::from(value)),
            "status" => {
                let status: Status = serde_json::from_str(value).map_err(|e| e.to_string())?;
                bson::to_bson(&status).map_err(|e| e.to_string())
            }
            "created_on" => bson::DateTime::parse_rfc3339_str(value)
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
        let collection = Self::get_collection(settings, &settings.mongo.collections.file)
            .await
            .map_err(|e| e.to_string())?;
        Ok(Self { collection })
    }
}

#[async_trait]
impl Repository for Mongo {
    async fn create(&self, file: File) -> Result<(), String> {
        let entity = Entity::try_from(file.clone())?;
        let _ = self
            .collection
            .insert_one(entity, None)
            .await
            .map_err(|e| format!("{e}"))?;
        Ok(())
    }

    async fn delete(&self, id: &str) -> Result<(), String> {
        let filter = doc! { "_id": id };
        let _ = self
            .collection
            .delete_one(filter, None)
            .await
            .map_err(|e| format!("{e}"))?;
        Ok(())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<File>, String> {
        let query = Some(doc! { "_id": id });
        let result = self.query(&self.collection, query, None).await;
        let analyses: Vec<File> = result.map_err(|e| format!("{e}"))?;
        let found = analyses.first().cloned();
        Ok(found)
    }

    async fn find_by_criteria(&self, criteria: &Criteria) -> Result<Option<Files>, String> {
        self.query_criteria(&self.collection, criteria)
            .await
            .map(|v| Some(Files::from(v)))
            .map_err(|e| e.to_string())
    }

    async fn validate(&self, id: &str) -> Result<(), String> {
        let filter = doc! { "_id": id };

        let status = Status::Success;
        let status = bson::to_bson(&status).map_err(|e| e.to_string())?;

        let update = doc! { "$set": doc!{ "status": status } };
        let _ = self
            .collection
            .update_one(filter, update, None)
            .await
            .map_err(|e| format!("{e}"))?;
        Ok(())
    }

    async fn mark_as_error(&self, id: &str, msg: &str) -> Result<(), String> {
        let filter = doc! { "_id": id };

        let status = Status::Fail(msg.to_owned());
        let status = bson::to_bson(&status).map_err(|e| e.to_string())?;

        let update = doc! { "$set": doc!{ "status": status } };
        let _ = self
            .collection
            .update_one(filter, update, None)
            .await
            .map_err(|e| format!("{e}"))?;
        Ok(())
    }
}
