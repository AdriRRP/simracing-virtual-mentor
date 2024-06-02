use shared::lap::domain::lap::header::Header;
use shared::lap::domain::lap::headers::Headers;
use shared::lap::domain::lap::Lap;
use shared::lap::domain::laps::Laps;
use shared::lap::domain::repository::Repository;

use crate::infrastructure::settings::Settings;

use async_trait::async_trait;
use reqwest::Client;

#[derive(Debug)]
pub struct Http {
    pub(crate) client: Client,
    pub(crate) delete: String,
    pub(crate) find_by_id: String,
    pub(crate) find_by_criteria: String,
    pub(crate) find_header_by_id: String,
    pub(crate) find_header_by_criteria: String,
}

impl Http {
    #[must_use]
    pub fn new(settings: &Settings) -> Self {
        Self {
            client: Client::new(),
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
}

#[async_trait]
impl Repository for Http {
    async fn create(&self, _laps: Laps) {
        unimplemented!()
    }

    async fn delete(&self, id: &uuid::Uuid) -> Result<(), String> {
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

    async fn find_by_id(&self, id: &uuid::Uuid) -> Result<Option<Lap>, String> {
        let endpoint = format!("{}/{id}", self.find_by_id);
        let response = self
            .client
            .get(endpoint)
            .send()
            .await
            .map_err(|e| format!("{e}"))?;

        if response.status().is_success() {
            let lap: Lap = response.json().await.map_err(|e| format!("{e}"))?;
            Ok(Some(lap))
        } else {
            Err(response.status().to_string())
        }
    }

    async fn find_by_criteria(&self, criteria: &str) -> Result<Option<Laps>, String> {
        let response = self
            .client
            .get(&self.find_by_criteria)
            .json(criteria)
            .send()
            .await
            .map_err(|e| format!("{e}"))?;

        if response.status().is_success() {
            let laps: Laps = response.json().await.map_err(|e| format!("{e}"))?;
            if laps.is_empty() {
                Ok(None)
            } else {
                Ok(Some(laps))
            }
        } else {
            Err(response.status().to_string())
        }
    }

    async fn find_header_by_id(&self, id: &uuid::Uuid) -> Result<Option<Header>, String> {
        let endpoint = format!("{}/{id}", self.find_header_by_id);
        let response = self
            .client
            .get(endpoint)
            .send()
            .await
            .map_err(|e| format!("{e}"))?;

        if response.status().is_success() {
            let header: Header = response.json().await.map_err(|e| format!("{e}"))?;
            Ok(Some(header))
        } else {
            Err(response.status().to_string())
        }
    }

    async fn find_header_by_criteria(&self, criteria: &str) -> Result<Option<Headers>, String> {
        let response = self
            .client
            .get(&self.find_header_by_criteria)
            .json(criteria)
            .send()
            .await
            .map_err(|e| format!("{e}"))?;

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
