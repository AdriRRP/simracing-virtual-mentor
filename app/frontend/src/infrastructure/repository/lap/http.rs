use shared::common::domain::criteria::Criteria;
use shared::lap::domain::lap::headers::Headers;
use shared::lap::domain::lap::Lap;
use shared::lap::domain::laps::Laps;

use crate::infrastructure::settings::Settings;

use reqwest::Client;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Http {
    pub delete: String,
    pub find_by_id: String,
    pub find_by_criteria: String,
    pub find_header_by_id: String,
    pub find_header_by_criteria: String,
}

impl Http {
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

    pub async fn delete(&self, id: &uuid::Uuid) -> Result<(), String> {
        let endpoint = format!("{}/{id}", self.delete);
        Client::new()
            .delete(endpoint)
            .send()
            .await
            .map_err(|e| format!("{e}"))?;

        Ok(())
    }

    pub async fn find_by_id(&self, id: &uuid::Uuid) -> Result<Option<Lap>, String> {
        let endpoint = format!("{}/{id}", self.find_by_id);
        let response = Client::new()
            .get(&endpoint)
            .send()
            .await
            .map_err(|e| format!("{e}"))?;

        if response.status().is_success() {
            let lap: Lap = response.json().await.map_err(|e| format!("{e}"))?;
            Ok(Some(lap))
        } else if response.status() == 404 {
            Ok(None)
        } else {
            Err(response.status().to_string())
        }
    }

    pub async fn find_by_criteria(&self, criteria: &str) -> Result<Option<Laps>, String> {
        let response = Client::new()
            .post(&self.find_by_criteria)
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

    pub(crate) async fn find_header_by_criteria(
        &self,
        criteria: &Criteria,
    ) -> Result<Option<Headers>, String> {
        let response = Client::new()
            .post(&self.find_header_by_criteria)
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
