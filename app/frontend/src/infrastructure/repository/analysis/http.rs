use crate::infrastructure::settings::Settings;

use shared::analysis::domain::analyses::Analyses;
use shared::analysis::domain::analysis::headers::Headers;
use shared::analysis::domain::analysis::Analysis;
use shared::common::domain::criteria::Criteria;

use log::{error, info, warn};
use reqwest::header::CONTENT_LENGTH;
use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

/// Struct representing the HTTP client for performing various analysis-related operations.
///
/// This struct holds the endpoints for creating, deleting, and retrieving analyses,
/// as well as retrieving headers and searching based on criteria.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Http {
    pub create: String,
    pub delete: String,
    pub find_by_id: String,
    pub find_by_criteria: String,
    pub find_header_by_criteria: String,
}

impl Http {
    /// Creates a new `Http` instance using the provided settings.
    ///
    /// This function initializes the `Http` struct by constructing the URLs for
    /// various operations related to analysis (e.g., create, delete, find by ID).
    ///
    /// # Arguments
    ///
    /// * `settings` - A reference to `Settings` containing the server and endpoint configuration.
    ///
    /// # Returns
    ///
    /// Returns a new `Http` instance with the respective endpoints initialized.
    #[must_use]
    pub fn new(settings: &Settings) -> Self {
        Self {
            create: format!(
                "{}{}",
                settings.endpoints.analysis.server, settings.endpoints.analysis.create
            ),
            delete: format!(
                "{}{}",
                settings.endpoints.analysis.server, settings.endpoints.analysis.delete
            ),
            find_by_id: format!(
                "{}{}",
                settings.endpoints.analysis.server, settings.endpoints.analysis.find_by_id
            ),
            find_by_criteria: format!(
                "{}{}",
                settings.endpoints.analysis.server, settings.endpoints.analysis.find_by_criteria
            ),
            find_header_by_criteria: format!(
                "{}{}",
                settings.endpoints.analysis.server,
                settings.endpoints.analysis.find_header_by_criteria
            ),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Request {
    pub id: Uuid,
    pub name: String,
    pub ref_lap_id: Uuid,
    pub target_lap_id: Uuid,
}

impl Request {
    /// Creates a new `Request` for analysis.
    ///
    /// This function initializes a request by generating a new `Uuid` and
    /// associating it with the provided `name`, `ref_lap_id`, and `target_lap_id`.
    ///
    /// # Arguments
    ///
    /// * `name` - A string representing the name of the analysis.
    /// * `ref_lap_id` - The `Uuid` of the reference lap.
    /// * `target_lap_id` - The `Uuid` of the target lap.
    ///
    /// # Returns
    ///
    /// Returns a new `Request` instance with a newly generated `Uuid`.
    #[must_use]
    pub fn new(name: String, ref_lap_id: Uuid, target_lap_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            ref_lap_id,
            target_lap_id,
        }
    }
}

impl Http {
    /// Creates a new analysis on the server.
    ///
    /// This function sends a PUT request with the analysis data to the server.
    ///
    /// # Arguments
    ///
    /// * `request` - A `Request` containing the data for the analysis to be created.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the analysis was created successfully.
    /// * `Err(String)` - If the creation failed, an error message with the response status is returned.
    ///
    /// # Errors
    ///
    /// This function can return various types of errors:
    ///
    /// * If the HTTP request fails, it returns an error with the failure message.
    /// * If the server responds with a non-success status, the function returns the response status.
    pub async fn create(&self, request: Request) -> Result<(), String> {
        let response = Client::new()
            .put(&self.create)
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("{e}"))?;

        warn!("{request:?}");

        if response.status().is_success() {
            warn!("{response:?}");
            Ok(())
        } else {
            error!("{response:?}");
            Err(response.status().to_string())
        }
    }

    /// Deletes an analysis by its ID.
    ///
    /// This function sends a DELETE request to remove an analysis based on its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - A reference to a `Uuid` representing the ID of the analysis to be deleted.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the deletion was successful.
    /// * `Err(String)` - If the deletion failed, an error message with the response status is returned.
    ///
    /// # Errors
    ///
    /// This function can return various types of errors:
    ///
    /// * If the HTTP request fails, it returns an error with the failure message.
    /// * If the server responds with a non-success status, the function returns the response status.
    pub async fn delete(&self, id: &Uuid) -> Result<(), String> {
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

    /// Finds an analysis by its ID.
    ///
    /// This function sends a GET request to retrieve an analysis based on its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - A reference to a `Uuid` representing the ID of the analysis.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(Analysis))` - If the analysis was found.
    /// * `Ok(None)` - If the analysis was not found (404 status).
    /// * `Err(String)` - If an error occurred during the request or response parsing.
    ///
    /// # Errors
    ///
    /// This function can return various types of errors:
    ///
    /// * If the HTTP request fails, it returns an error with the failure message.
    /// * If the server responds with a non-success status, the function returns the response status.
    pub async fn find_by_id(&self, id: &Uuid) -> Result<Option<Analysis>, String> {
        let endpoint = format!("{}/{id}", self.find_by_id);
        let response = Client::new()
            .get(&endpoint)
            .send()
            .await
            .map_err(|e| format!("{e}"))?;

        if response.status().is_success() {
            let analysis: Analysis = response.json().await.map_err(|e| format!("{e}"))?;
            Ok(Some(analysis))
        } else if response.status() == 404 {
            Ok(None)
        } else {
            Err(response.status().to_string())
        }
    }

    /// Finds analyses based on the provided criteria.
    ///
    /// This function sends a POST request with the search criteria to find matching analyses.
    ///
    /// # Arguments
    ///
    /// * `criteria` - A reference to `Criteria` that specifies the search filters.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(Analyses))` - If matching analyses are found.
    /// * `Ok(None)` - If no analyses match the criteria (404 status).
    /// * `Err(String)` - If an error occurred during the request or response parsing.
    ///
    /// # Errors
    ///
    /// This function can return various types of errors:
    ///
    /// * If the HTTP request fails, it returns an error with the failure message.
    /// * If the server responds with a non-success status, the function returns the response status.
    pub async fn find_by_criteria(&self, criteria: &Criteria) -> Result<Option<Analyses>, String> {
        let response = Client::new()
            .post(&self.find_by_criteria)
            .header(CONTENT_LENGTH, "application/json")
            .json(criteria)
            .send()
            .await
            .map_err(|e| format!("{e}"))?;

        info!("{:?}", response);

        if response.status().is_success() {
            let json = response.json().await.map_err(|e| {
                error!("{:?}", e);
                format!("{e}")
            })?;
            let analyses: Option<Analyses> = json;
            Ok(analyses)
        } else {
            Err(response.status().to_string())
        }
    }

    /// Finds analysis headers based on the provided criteria.
    ///
    /// This function sends a POST request with the search criteria to find matching analysis headers.
    ///
    /// # Arguments
    ///
    /// * `criteria` - A reference to `Criteria` that specifies the search filters for headers.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(Headers))` - If matching headers are found.
    /// * `Ok(None)` - If no headers match the criteria.
    /// * `Err(String)` - If an error occurred during the request or response parsing.
    ///
    /// # Errors
    ///
    /// This function can return various types of errors:
    ///
    /// * If the HTTP request fails, it returns an error with the failure message.
    /// * If the server responds with a non-success status, the function returns the response status.
    pub(crate) async fn find_header_by_criteria(
        &self,
        criteria: &Criteria,
    ) -> Result<Option<Headers>, String> {
        error!("response: {:?}", &self.find_header_by_criteria);

        let response = Client::new()
            .post(&self.find_header_by_criteria)
            .json(criteria)
            .send()
            .await
            .map_err(|e| format!("{e}"))?;

        error!("response: {response:?}");

        if response.status().is_success() {
            let headers: Headers = response.json().await.map_err(|e| format!("{e}"))?;
            if headers.is_empty() {
                Ok(None)
            } else {
                Ok(Some(headers))
            }
        } else {
            Err(response.status().to_string())
        }
    }
}
