use crate::infrastructure::settings::Settings;

use shared::common::domain::criteria::Criteria;
use shared::file::domain::file::File;
use shared::file::domain::files::Files;

use reqwest::Client;

/// Struct representing the HTTP client for performing file-related operations.
///
/// This struct holds the endpoints for various file operations such as deleting,
/// finding files by ID, and searching files based on criteria.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Http {
    pub delete: String,
    pub find_by_id: String,
    pub find_by_criteria: String,
}

impl Http {
    /// Creates a new `Http` instance using the provided settings.
    ///
    /// This function initializes the `Http` struct by constructing the URLs for
    /// various file operations such as deletion, finding by ID, and finding by criteria.
    ///
    /// # Arguments
    ///
    /// * `settings` - A reference to `Settings` that contains the server and endpoint configuration.
    ///
    /// # Returns
    ///
    /// Returns a new `Http` instance with the `delete`, `find_by_id`, and `find_by_criteria` endpoints initialized.
    #[must_use]
    pub fn new(settings: &Settings) -> Self {
        Self {
            delete: format!(
                "{}{}",
                settings.endpoints.file.server, settings.endpoints.file.delete
            ),
            find_by_id: format!(
                "{}{}",
                settings.endpoints.file.server, settings.endpoints.file.find_by_id
            ),
            find_by_criteria: format!(
                "{}{}",
                settings.endpoints.file.server, settings.endpoints.file.find_by_criteria
            ),
        }
    }
}

impl Default for Http {
    /// Provides a default implementation for the `Http` struct.
    ///
    /// This function uses the default `Settings` to initialize the `Http` instance.
    ///
    /// # Returns
    ///
    /// Returns a new `Http` instance using the default settings.
    fn default() -> Self {
        Self::new(&Settings::default())
    }
}

impl Http {
    /// Deletes a file by its ID.
    ///
    /// This function sends a DELETE request to remove a file based on its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - A string reference representing the ID of the file to be deleted.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the deletion is successful and the server responds with a success status (2xx).
    /// * `Err(String)` - If the deletion fails, an error message with the response status is returned.
    ///
    /// # Errors
    ///
    /// This function can return various types of errors:
    ///
    /// * If the HTTP request fails (e.g., due to network issues), it returns an error with the failure message.
    /// * If the server responds with a non-success status, the function returns the response status.
    pub async fn delete(&self, id: &str) -> Result<(), String> {
        let endpoint = format!("{}/{id}", self.delete);
        let response = Client::new()
            .delete(&endpoint)
            .send()
            .await
            .map_err(|e| format!("{e}"))?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(response.status().to_string())
        }
    }

    /// Finds a file by its ID.
    ///
    /// This function sends a GET request to retrieve a file based on its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - A string reference representing the ID of the file to be fetched.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(File))` - If the file is found, it returns the file wrapped in `Some`.
    /// * `Ok(None)` - If the file is not found (404 status), it returns `None`.
    /// * `Err(String)` - If an error occurs during the request or response parsing.
    ///
    /// # Errors
    ///
    /// This function can return various types of errors:
    ///
    /// * If the HTTP request fails (e.g., due to network issues), it returns an error with the failure message.
    /// * If the server responds with a non-success status, the function returns the response status.
    /// * If the response cannot be parsed into a `File`, it returns a parsing error.
    pub async fn find_by_id(&self, id: &str) -> Result<Option<File>, String> {
        let endpoint = format!("{}/{id}", self.find_by_id);
        let response = Client::new()
            .get(&endpoint)
            .send()
            .await
            .map_err(|e| format!("{e}"))?;

        if response.status().is_success() {
            let file: File = response.json().await.map_err(|e| format!("{e}"))?;
            Ok(Some(file))
        } else if response.status() == 404 {
            Ok(None)
        } else {
            Err(response.status().to_string())
        }
    }

    /// Finds files based on the provided criteria.
    ///
    /// This function sends a POST request with the provided `criteria` to search for files.
    ///
    /// # Arguments
    ///
    /// * `criteria` - A reference to `Criteria` that specifies the search filters for the files.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(Files))` - If files matching the criteria are found, they are returned in `Some`.
    /// * `Ok(None)` - If no files match the criteria (404 status), it returns `None`.
    /// * `Err(String)` - If an error occurs during the request or response parsing.
    ///
    /// # Errors
    ///
    /// This function can return various types of errors:
    ///
    /// * If the HTTP request fails (e.g., due to network issues), it returns an error with the failure message.
    /// * If the server responds with a non-success status, the function returns the response status.
    /// * If the response cannot be parsed into a `Files` collection, it returns a parsing error.
    pub async fn find_by_criteria(&self, criteria: &Criteria) -> Result<Option<Files>, String> {
        let response = Client::new()
            .post(&self.find_by_criteria)
            .json(criteria)
            .send()
            .await
            .map_err(|e| format!("{e}"))?;

        if response.status().is_success() {
            let files: Files = response.json().await.map_err(|e| format!("{e}"))?;
            if files.is_empty() {
                Ok(None)
            } else {
                Ok(Some(files))
            }
        } else if response.status().as_u16() == 404 {
            Ok(None)
        } else {
            Err(response.status().to_string())
        }
    }
}
