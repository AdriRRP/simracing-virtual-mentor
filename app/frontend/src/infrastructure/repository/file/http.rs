use log::info;
use crate::infrastructure::settings::Settings;

use shared::file::domain::file::File;
use shared::file::domain::files::Files;

use reqwest::Client;
use shared::common::domain::criteria::Criteria;

#[derive(Clone, Debug, PartialEq)]
pub struct Http {
    pub delete: String,
    pub find_by_id: String,
    pub find_by_criteria: String,
}

impl Http {
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
    fn default() -> Self {
        Self::new(&Settings::default())
    }
}

impl Http {
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
