use shared::analysis::domain::analyses::Analyses;
use shared::analysis::domain::analysis::Analysis;
use shared::analysis::domain::repository::Repository;

use crate::infrastructure::settings::Settings;

use async_trait::async_trait;
use reqwest::Client;
use uuid::Uuid;

#[derive(Debug)]
pub struct Http {
    pub(crate) client: Client,
    pub(crate) create: String,
    pub(crate) delete: String,
    pub(crate) find_by_id: String,
    pub(crate) find_by_criteria: String,
}

impl Http {
    #[must_use]
    pub fn new(settings: &Settings) -> Self {
        Self {
            client: Client::new(),
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
        }
    }
}

#[async_trait]
impl Repository for Http {
    async fn create(&self, analysis: Analysis) -> Result<(), String> {
        let response = &self
            .client
            .put(&self.create)
            .json(&analysis)
            .send()
            .await
            .map_err(|e| format!("{e}"))?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(response.status().to_string())
        }
    }

    async fn delete(&self, id: &Uuid) -> Result<(), String> {
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

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Analysis>, String> {
        let endpoint = format!("{}/{id}", self.find_by_id);
        let response = self
            .client
            .get(endpoint)
            .send()
            .await
            .map_err(|e| format!("{e}"))?;

        if response.status().is_success() {
            let analysis: Analysis = response.json().await.map_err(|e| format!("{e}"))?;
            Ok(Some(analysis))
        } else {
            Err(response.status().to_string())
        }
    }

    async fn find_by_criteria(&self, criteria: &str) -> Result<Option<Analyses>, String> {
        let response = self
            .client
            .get(&self.find_by_criteria)
            .json(criteria)
            .send()
            .await
            .map_err(|e| format!("{e}"))?;

        if response.status().is_success() {
            let analyses: Analyses = response.json().await.map_err(|e| format!("{e}"))?;
            if analyses.is_empty() {
                Ok(None)
            } else {
                Ok(Some(analyses))
            }
        } else {
            Err(response.status().to_string())
        }
    }
}
