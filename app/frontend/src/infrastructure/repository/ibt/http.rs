use crate::infrastructure::settings::Settings;

use reqwest::{multipart::Part, Client};
use urlencoding::encode;

/// Struct representing the HTTP client for performing file uploads.
///
/// This struct holds the endpoint for uploading files to a remote server.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Http {
    pub upload: String,
}

impl Http {
    /// Creates a new `Http` instance using the provided settings.
    ///
    /// This function initializes the `Http` struct by constructing the `upload`
    /// endpoint URL using the server and endpoint paths provided in the settings.
    ///
    /// # Arguments
    ///
    /// * `settings` - A reference to `Settings` that contains the server and endpoint configuration.
    ///
    /// # Returns
    ///
    /// Returns a new `Http` instance with the `upload` endpoint initialized.
    #[must_use]
    pub fn new(settings: &Settings) -> Self {
        Self {
            upload: format!(
                "{}{}",
                settings.endpoints.ibt_extractor.server, settings.endpoints.ibt_extractor.upload
            ),
        }
    }
}

impl Http {
    /// Uploads a file to the specified endpoint using multipart form data.
    ///
    /// This function uploads a file to a remote server. The file is sent as part of
    /// a multipart form request, where the file is represented by the given `data`
    /// and associated with the provided `name` and `file_name`.
    ///
    /// # Arguments
    ///
    /// * `name` - A `String` representing the name for the upload. This will be URL-encoded.
    /// * `file_name` - The name of the file to be uploaded. The extension ".ibt" will be appended to the file name.
    /// * `data` - A vector of bytes (`Vec<u8>`) representing the file data that will be uploaded.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the file is successfully uploaded and the server responds with a success status (2xx).
    /// * `Err(String)` - If the upload fails due to client errors, server errors, or other issues,
    ///                   an error message describing the failure will be returned.
    ///
    /// # Errors
    ///
    /// This function can return various types of errors in the `Err` variant of the `Result`:
    ///
    /// * If the HTTP request fails (e.g., due to network issues or the server being unreachable),
    ///   it returns an error with the request failure message.
    /// * If the server responds with a client error (4xx) or server error (5xx),
    ///   it returns an error with the status code and response message from the server.
    /// * If the server responds with informational (1xx) or redirection (3xx) status codes,
    ///   the function returns an appropriate error message describing the situation.
    pub async fn upload(
        &self,
        name: String,
        file_name: String,
        data: Vec<u8>,
    ) -> Result<(), String> {
        // URL-encode the `name` and construct the full endpoint URL.
        let name = encode(name.as_ref()).to_string();
        let endpoint = format!("{}/{}", self.upload, name);

        // Prepare the file part for the multipart request. Appends ".ibt" to the `file_name`.
        let part = Part::bytes(data).file_name(format!("{file_name}.ibt"));

        // Create a multipart form with the file part.
        let file = reqwest::multipart::Form::new().part("file", part);

        // Send the POST request with the multipart form data.
        let response = Client::new()
            .post(&endpoint)
            .multipart(file)
            .send()
            .await
            .map_err(|e| format!("{e}"))?; // Capture any errors from the HTTP request.

        // Check if the response was successful (2xx status codes).
        if response.status().is_success() {
            Ok(()) // Return `Ok` if the upload was successful.
        }
        // Handle client (4xx) or server (5xx) errors.
        else if response.status().is_client_error() || response.status().is_server_error() {
            Err(format!(
                "{}: {}", // Return the status code and response body in case of error.
                response.status(),
                response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Cannot get Response message...".to_string())
            ))
        }
        // Handle informational responses (1xx status codes).
        else if response.status().is_informational() {
            Err("Requested endpoint is informational".to_string())
        }
        // Handle redirection responses (3xx status codes).
        else if response.status().is_redirection() {
            Err("Requested endpoint is a redirection".to_string())
        }
        // Handle any other unknown server responses.
        else {
            Err("Unknown server error".to_string())
        }
    }
}
