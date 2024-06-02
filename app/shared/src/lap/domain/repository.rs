use crate::lap::domain::lap::header::Header;
use crate::lap::domain::lap::headers::Headers;
use crate::lap::domain::lap::Lap;
use crate::lap::domain::laps::Laps;

use async_trait::async_trait;
use uuid::Uuid;

/// Trait for asynchronous repository operations.
#[async_trait]
pub trait Repository: Send + Sync {
    /// Creates new laps data asynchronously.
    ///
    /// # Errors
    ///
    /// This asynchronous function will return an `Err` if there is an error while attempting to create the laps data.
    async fn create(&self, laps: Laps) -> Result<(), String>;

    /// Deletes laps data asynchronously given its ID.
    ///
    /// # Errors
    ///
    /// This asynchronous function will return an `Err` if there is an error while attempting to delete the laps data.
    async fn delete(&self, id: &Uuid) -> Result<(), String>;

    /// Finds laps data asynchronously by its ID.
    ///
    /// # Errors
    ///
    /// This asynchronous function will return an `Err` if there is an error while attempting to find the laps data.
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Lap>, String>;

    /// Finds laps data asynchronously based on specific criteria.
    ///
    /// # Errors
    ///
    /// This asynchronous function will return an `Err` if there is an error while attempting to find the laps data.
    async fn find_by_criteria(&self, criteria: &str) -> Result<Option<Laps>, String>;

    /// Finds the header data asynchronously by its ID.
    ///
    /// # Errors
    ///
    /// This asynchronous function will return an `Err` if there is an error while attempting to find the header data.
    async fn find_header_by_id(&self, id: &Uuid) -> Result<Option<Header>, String>;

    /// Finds header data asynchronously based on specific criteria.
    ///
    /// # Errors
    ///
    /// This asynchronous function will return an `Err` if there is an error while attempting to find the header data.
    async fn find_header_by_criteria(&self, criteria: &str) -> Result<Option<Headers>, String>;
}
