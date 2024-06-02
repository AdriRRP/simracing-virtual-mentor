use shared::file::domain::file::File;
use shared::file::domain::files::Files;
use shared::file::domain::repository::Repository;

use crate::infrastructure::settings::Settings;

use async_trait::async_trait;
use reqwest::Client;

#[derive(Debug)]
pub struct Http {
    pub(crate) client: Client,
    pub(crate) delete: String,
    pub(crate) find_by_id: String,
    pub(crate) find_by_criteria: String,
}

impl Http {
    #[must_use]
    pub fn new(settings: &Settings) -> Self {
        Self {
            client: Client::new(),
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

#[async_trait]
impl Repository for Http {
    async fn create(&self, _file: File) {
        unimplemented!()
    }

    async fn delete(&self, id: &str) -> Result<(), String> {
        let endpoint = format!("{}/{id}", self.delete);
        let response = &self
            .client
            .delete(endpoint)
            .send()
            .await
            .map_err(|e| format!("{e}"))?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(response.status().to_string())
        }
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<File>, String> {
        let endpoint = format!("{}/{id}", self.find_by_id);
        let response = self
            .client
            .get(endpoint)
            .send()
            .await
            .map_err(|e| format!("{e}"))?;

        if response.status().is_success() {
            let file: File = response.json().await.map_err(|e| format!("{e}"))?;
            Ok(Some(file))
        } else {
            Err(response.status().to_string())
        }
    }

    async fn find_by_criteria(&self, criteria: &str) -> Result<Option<Files>, String> {
        let response = self
            .client
            .get(&self.find_by_criteria)
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
        } else {
            Err(response.status().to_string())
        }
    }

    async fn validate(&self, _id: &str) -> Result<(), String> {
        unimplemented!()
    }
}
