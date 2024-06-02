use crate::file::domain::file::File;
use crate::file::domain::files::Files;

use async_trait::async_trait;

/// Trait for asynchronous file operations.
#[async_trait]
pub trait Repository: Send + Sync {
    /// Creates a new file asynchronously.
    ///
    /// # Errors
    ///
    /// This asynchronous function will return an `Err` if there is an error while attempting to create the file.
    async fn create(&self, file: File) -> Result<(), String>;

    /// Deletes a file asynchronously given its ID.
    ///
    /// # Errors
    ///
    /// This asynchronous function will return an `Err` if there is an error while attempting to delete the file.
    async fn delete(&self, id: &str) -> Result<(), String>;

    /// Finds a file asynchronously by its ID.
    ///
    /// # Errors
    ///
    /// This asynchronous function will return an `Err` if there is an error while attempting to find the file.
    async fn find_by_id(&self, id: &str) -> Result<Option<File>, String>;

    /// Finds files asynchronously based on specific criteria.
    ///
    /// # Errors
    ///
    /// This asynchronous function will return an `Err` if there is an error while attempting to find the files.
    async fn find_by_criteria(&self, criteria: &str) -> Result<Option<Files>, String>;

    /// Validates a file asynchronously given its ID.
    ///
    /// # Errors
    ///
    /// This asynchronous function will return an `Err` if there is an error while attempting to validate the file.
    async fn validate(&self, id: &str) -> Result<(), String>;
}
