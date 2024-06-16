use crate::infrastructure::settings::Settings;
use reqwest::Client;

use shared::analysis::domain::analyses::Analyses;
use shared::analysis::domain::analysis::Analysis;

use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct Http {
    pub create: String,
    pub delete: String,
    pub find_by_id: String,
    pub find_by_criteria: String,
}

impl Http {
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
        }
    }
}

impl Http {
    pub async fn create(&self, analysis: Analysis) -> Result<(), String> {
        let response = Client::new()
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

    pub async fn find_by_criteria(&self, criteria: &str) -> Result<Option<Analyses>, String> {
        let response = Client::new()
            .post(&self.find_by_criteria)
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
