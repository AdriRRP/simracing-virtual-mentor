use crate::api::infrastructure::settings::Settings;
use crate::shared::common::domain::criteria::order::r#type::Type as OrderType;
use crate::shared::common::domain::criteria::Criteria;

use shared::common::domain::criteria::filter::condition::Condition;

use async_trait::async_trait;
use bson::doc;
use bson::Bson;
use bson::Document;
use futures::TryStreamExt;
use mongodb::options::FindOptions;
use mongodb::options::{ClientOptions, Credential};
use mongodb::{Client, Collection};
use serde::de::DeserializeOwned;
use std::fmt::{Debug, Display};
use tracing::warn;

pub mod analysis;
pub mod file;
pub mod lap;

#[async_trait]
pub trait Mongo<T, MongoT>
where
    T: Sized,
    MongoT: DeserializeOwned + Sync + Send + Unpin + TryInto<T>,
    MongoT: 'static,
    <MongoT as TryInto<T>>::Error: Display,
{
    async fn get_collection(
        settings: &Settings,
        collection: &str,
    ) -> Result<Collection<MongoT>, Error> {
        let mut client_options = match ClientOptions::parse_async(&settings.mongo.uri).await {
            Ok(opts) => opts,
            Err(e) => return Err(Error::ParseClientOptions(e.to_string())),
        };

        let default_cred = Credential::builder()
            .username(settings.mongo.user.clone())
            .password(settings.mongo.pass.clone())
            .source(settings.mongo.database.clone())
            .build();

        client_options.credential = Some(default_cred);

        let client = match Client::with_options(client_options) {
            Ok(cl) => cl,
            Err(e) => {
                return Err(Error::CreateClient(e.to_string()));
            }
        };

        let database = client.database(&settings.mongo.database);
        Ok(database.collection(collection))
    }

    async fn query(
        &self,
        collection: &Collection<MongoT>,
        query: Option<Document>,
        options: Option<FindOptions>,
    ) -> Result<Vec<T>, Error> {
        let cursor = collection
            .find(query, options)
            .await
            .map_err(|e| Error::Finding(e.to_string()))?;

        let mongo_type_vec: Vec<MongoT> = cursor
            .try_collect()
            .await
            .map_err(|e| Error::Collecting(e.to_string()))?;

        mongo_type_vec
            .into_iter()
            .map(|x| x.try_into().map_err(|e| Error::Converting(format!("{e}"))))
            .collect()
    }

    async fn query_criteria(
        &self,
        collection: &Collection<MongoT>,
        criteria: &Criteria,
    ) -> Result<Vec<T>, Error> {
        let limit = criteria.limit().and_then(|limit| i64::try_from(limit).ok());

        let sort = match criteria.order() {
            Some(order)
                if order.by().field_name().is_empty()
                    && matches!(order.r#type(), OrderType::None) =>
            {
                None
            }
            Some(order) => {
                let field = order.by().field_name();
                match order.r#type() {
                    OrderType::None => None,
                    OrderType::Asc => Some(doc! { field : 1 }),
                    OrderType::Desc => Some(doc! { field : -1 }),
                }
            }
            None => None,
        };

        let find_opts = sort.map_or_else(
            || FindOptions::builder().limit(limit).build(),
            |s| FindOptions::builder().limit(limit).sort(s).build(),
        );

        let filters_bson_result: Result<Vec<Bson>, String> = criteria
            .filters()
            .unwrap_or_default()
            .iter()
            .map(|filter| {
                let field = filter.field().name().to_owned(); // TODO: Semantic access in all accesses of this type
                let value = match Self::map_criteria_field(&field, filter.value().get()) {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                };
                let doc = match filter.condition() {
                    Condition::Equal => doc!(field: value),
                    Condition::NotEqual => doc!(field: doc!("$ne": value)),
                    Condition::GreaterThan => doc!(field: doc!("$gt": value)),
                    Condition::LowerThan => doc!(field: doc!("$lt": value)),
                    Condition::Contains => {
                        //doc!(field : doc!( "$regex" : doc!("$toString" : value)))
                        doc!(field : doc!( "$regex" : value, "$options": "i" ))
                    }
                    // TODO: Revisar filtro
                    Condition::NotContains => {
                        doc!(field : doc!( "$not" : doc!("$toString" : value)))
                    }
                };
                tracing::warn!("{doc:?}");
                Ok(bson::Bson::from(doc))
            })
            .collect();

        let filters_bson = filters_bson_result.map_err(Error::Filtering)?;
        
        tracing::trace!("Criteria BSON: {:?}", filters_bson.clone());

        let query = if filters_bson.is_empty() {
            Some(doc! {})
        } else {
            Some(doc!("$and": filters_bson))
        };

        tracing::trace!("Query: {:?}", query.clone());

        let result = self.query(collection, query, Some(find_opts)).await;
        if let Err(ref e) = result {
            tracing::error!("{:?}", e.to_string());
        }
        Ok(result?)
        //Ok(self.query(collection, query, Some(find_opts)).await?)
    }

    /// # Errors
    ///
    /// This function will return an error in the following situations:
    ///
    /// * If the provided field name or value cannot be converted into a valid BSON format, an error
    ///   string describing the problem will be returned.
    fn map_criteria_field(name: &str, value: &str) -> Result<Bson, String>;
}

#[derive(PartialEq, Eq, Debug, thiserror::Error)]
pub enum Error {
    #[error("Error parsing client options: {0}")]
    ParseClientOptions(String),
    #[error("Error creating Mongo client: {0}")]
    CreateClient(String),
    #[error("Error finding in Mongo: {0}")]
    Finding(String),
    #[error("Error collecting in Mongo: {0}")]
    Collecting(String),
    #[error("Failed converting mongo model to domain model: {0}")]
    Converting(String),
    #[error("Failed filtering mongo model by criteria: {0}")]
    Filtering(String),
}
