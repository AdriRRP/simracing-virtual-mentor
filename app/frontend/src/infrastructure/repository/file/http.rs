use shared::file::domain::file::File;
use shared::file::domain::files::Files;

use crate::infrastructure::settings::Settings;

use gloo_net::http::Request;

#[derive(Debug)]
pub struct Http {
    pub(crate) delete: String,
    pub(crate) find_by_id: String,
    pub(crate) find_by_criteria: String,
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

impl Http {
    pub(crate) async fn delete(&self, id: &str) -> Result<(), String> {
        let endpoint = format!("{}/{id}", self.delete);
        let response = Request::delete(&endpoint)
            .send()
            .await
            .map_err(|e| format!("{e}"))?;

        if response.ok() {
            Ok(())
        } else {
            Err(response.status().to_string())
        }
    }

    pub(crate) async fn find_by_id(&self, id: &str) -> Result<Option<File>, String> {
        let endpoint = format!("{}/{id}", self.find_by_id);
        let response = Request::get(&endpoint)
            .send()
            .await
            .map_err(|e| format!("{e}"))?;

        if response.ok() {
            let file: File = response.json().await.map_err(|e| format!("{e}"))?;
            Ok(Some(file))
        } else if response.status() == 404 {
            Ok(None)
        } else {
            Err(response.status().to_string())
        }
    }

    pub(crate) async fn find_by_criteria(&self, criteria: &str) -> Result<Option<Files>, String> {
        let response = Request::post(&self.find_by_criteria)
            .json(criteria)
            .map_err(|e| format!("{e}"))?
            .send()
            .await
            .map_err(|e| format!("{e}"))?;

        if response.ok() {
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
}
