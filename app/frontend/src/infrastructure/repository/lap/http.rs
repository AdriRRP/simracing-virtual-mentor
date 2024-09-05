use crate::infrastructure::settings::Settings;

use shared::common::domain::criteria::Criteria;
use shared::lap::domain::lap::headers::Headers;
use shared::lap::domain::lap::Lap;
use shared::lap::domain::laps::Laps;

use reqwest::Client;

/// Struct representing the HTTP client for performing various Lap-related requests.
/// It holds the endpoints for different operations such as deleting, finding by ID,
/// or retrieving laps and headers based on certain criteria.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Http {
    pub delete: String,
    pub find_by_id: String,
    pub find_by_criteria: String,
    pub find_header_by_id: String,
    pub find_header_by_criteria: String,
}

impl Http {
    /// Creates a new `Http` instance using the provided settings to set the endpoints.
    ///
    /// # Arguments
    ///
    /// * `settings` - A reference to `Settings` containing the server and endpoint paths.
    ///
    /// # Returns
    ///
    /// Returns a new `Http` instance with the appropriate endpoints initialized.
    #[must_use]
    pub fn new(settings: &Settings) -> Self {
        Self {
            delete: format!(
                "{}{}",
                settings.endpoints.lap.server, settings.endpoints.lap.delete
            ),
            find_by_id: format!(
                "{}{}",
                settings.endpoints.lap.server, settings.endpoints.lap.find_by_id
            ),
            find_by_criteria: format!(
                "{}{}",
                settings.endpoints.lap.server, settings.endpoints.lap.find_by_criteria
            ),
            find_header_by_id: format!(
                "{}{}",
                settings.endpoints.lap.server, settings.endpoints.lap.find_header_by_id
            ),
            find_header_by_criteria: format!(
                "{}{}",
                settings.endpoints.lap.server, settings.endpoints.lap.find_header_by_criteria
            ),
        }
    }

    /// Deletes a Lap by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - A reference to a `uuid::Uuid` representing the Lap to be deleted.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the deletion was successful, or `Err(String)` if an error occurred.
    ///
    /// # Errors
    ///
    /// * Returns an error if the HTTP request fails (e.g., network issue or server unavailable).
    pub async fn delete(&self, id: &uuid::Uuid) -> Result<(), String> {
        let endpoint = format!("{}/{id}", self.delete);
        Client::new()
            .delete(endpoint)
            .send()
            .await
            .map_err(|e| format!("{e}"))?; // Return error if the request fails

        Ok(())
    }

    /// Finds a Lap by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - A reference to a `uuid::Uuid` representing the Lap to be fetched.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Some(Lap))` if the Lap was found, `Ok(None)` if not, or `Err(String)` if an error occurred.
    ///
    /// # Errors
    ///
    /// * Returns an error if the HTTP request fails.
    /// * Returns an error if the response can't be parsed into a `Lap`.
    pub async fn find_by_id(&self, id: &uuid::Uuid) -> Result<Option<Lap>, String> {
        let endpoint = format!("{}/{id}", self.find_by_id);
        let response = Client::new()
            .get(&endpoint)
            .send()
            .await
            .map_err(|e| format!("{e}"))?; // Return error if the request fails

        // If response is successful, parse the Lap
        if response.status().is_success() {
            let lap: Lap = response.json().await.map_err(|e| format!("{e}"))?;
            Ok(Some(lap))
        }
        // Return None if the Lap is not found (404)
        else if response.status() == 404 {
            Ok(None)
        }
        // Return error for other non-success responses
        else {
            Err(response.status().to_string())
        }
    }

    /// Finds laps that match the provided criteria.
    ///
    /// # Arguments
    ///
    /// * `criteria` - A `&str` containing the criteria in JSON format.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Some(Laps))` if laps were found, `Ok(None)` if no laps match the criteria,
    /// or `Err(String)` if an error occurred.
    ///
    /// # Errors
    ///
    /// * Returns an error if the HTTP request fails.
    /// * Returns an error if the response can't be parsed into a `Laps` struct.
    pub async fn find_by_criteria(&self, criteria: &str) -> Result<Option<Laps>, String> {
        let response = Client::new()
            .post(&self.find_by_criteria)
            .json(criteria)
            .send()
            .await
            .map_err(|e| format!("{e}"))?; // Return error if the request fails

        // If successful, parse the Laps
        if response.status().is_success() {
            let laps: Laps = response.json().await.map_err(|e| format!("{e}"))?;
            if laps.is_empty() {
                Ok(None) // No laps found
            } else {
                Ok(Some(laps)) // Laps found
            }
        } else {
            Err(response.status().to_string()) // Return error for non-success responses
        }
    }

    /// Finds lap headers that match the provided criteria.
    ///
    /// # Arguments
    ///
    /// * `criteria` - A reference to `Criteria` used to filter the lap headers.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Some(Headers))` if headers were found, `Ok(None)` if no headers match the criteria,
    /// or `Err(String)` if an error occurred.
    ///
    /// # Errors
    ///
    /// * Returns an error if the HTTP request fails.
    /// * Returns an error if the response can't be parsed into a `Headers` struct.
    pub(crate) async fn find_header_by_criteria(
        &self,
        criteria: &Criteria,
    ) -> Result<Option<Headers>, String> {
        let response = Client::new()
            .post(&self.find_header_by_criteria)
            .json(criteria)
            .send()
            .await
            .map_err(|e| format!("{e}"))?; // Return error if the request fails

        // If successful, parse the Headers
        if response.status().is_success() {
            let headers: Headers = response.json().await.map_err(|e| format!("{e}"))?;
            if headers.is_empty() {
                Ok(None) // No headers found
            } else {
                Ok(Some(headers)) // Headers found
            }
        } else {
            Err(response.status().to_string()) // Return error for non-success responses
        }
    }
}
