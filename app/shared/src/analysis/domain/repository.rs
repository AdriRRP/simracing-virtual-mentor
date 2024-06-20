use crate::analysis::domain::analyses::Analyses;
use crate::analysis::domain::analysis::Analysis;

use crate::analysis::domain::analysis::header::Header;
use crate::analysis::domain::analysis::headers::Headers;
use crate::common::domain::criteria::Criteria;
use async_trait::async_trait;
use uuid::Uuid;

/// Trait for asynchronous analysis operations.
#[async_trait]
pub trait Repository: Send + Sync {
    /// Creates new analysis data asynchronously.
    ///
    /// # Errors
    ///
    /// This asynchronous function will return an `Err` if there is an error while attempting to create the analysis data.
    async fn create(&self, analysis: Analysis) -> Result<(), String>;

    /// Deletes analysis data asynchronously given its ID.
    ///
    /// # Errors
    ///
    /// This asynchronous function will return an `Err` if there is an error while attempting to delete the analysis data.
    async fn delete(&self, id: &Uuid) -> Result<(), String>;

    /// Finds analysis data asynchronously by its ID.
    ///
    /// # Errors
    ///
    /// This asynchronous function will return an `Err` if there is an error while attempting to find the analysis data.
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Analysis>, String>;

    /// Finds analysis data asynchronously based on specific criteria.
    ///
    /// # Errors
    ///
    /// This asynchronous function will return an `Err` if there is an error while attempting to find the analysis data.
    async fn find_by_criteria(&self, criteria: &Criteria) -> Result<Option<Analyses>, String>;

    /// Finds analysis header asynchronously by its ID.
    ///
    /// # Errors
    ///
    /// This asynchronous function will return an `Err` if there is an error while attempting to find the analysis data.
    async fn find_header_by_id(&self, id: &Uuid) -> Result<Option<Header>, String>;

    /// Finds analysis header asynchronously based on specific criteria.
    ///
    /// # Errors
    ///
    /// This asynchronous function will return an `Err` if there is an error while attempting to find the analysis data.
    async fn find_header_by_criteria(&self, criteria: &Criteria)
        -> Result<Option<Headers>, String>;
}
