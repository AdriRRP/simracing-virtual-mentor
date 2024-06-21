use crate::infrastructure::settings::Settings;
use reqwest::{multipart::Part, Client};
use urlencoding::encode;

#[derive(Clone, Debug, PartialEq)]
pub struct Http {
    pub upload: String,
}

impl Http {
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
    pub async fn upload(
        &self,
        name: String,
        file_name: String,
        data: Vec<u8>,
    ) -> Result<(), String> {
        let name = encode(name.as_ref()).to_string();
        let endpoint = format!("{}/{}", self.upload, name);

        let part = Part::bytes(data).file_name(format!("{file_name}.ibt"));
        let file = reqwest::multipart::Form::new().part("file", part);

        let response = Client::new()
            .post(&endpoint)
            .multipart(file)
            .send()
            .await
            .map_err(|e| format!("{e}"))?;

        if response.status().is_success() {
            Ok(())
        } else if response.status().is_client_error() || response.status().is_server_error() {
            Err(format!(
                "{}: {}",
                response.status().to_string(),
                response
                    .text()
                    .await
                    .unwrap_or("Cannot get Response message...".to_string())
            ))
        } else if response.status().is_informational() {
            Err("Requested endpoint is informational".to_string())
        } else if response.status().is_redirection() {
            Err("Requested endpoint is a redirection".to_string())
        } else {
            Err("Unknown server error".to_string())
        }
    }
}
