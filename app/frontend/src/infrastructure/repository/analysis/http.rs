use crate::infrastructure::settings::Settings;

use shared::analysis::domain::analyses::Analyses;
use shared::analysis::domain::analysis::Analysis;

use gloo_net::http::Request;
use uuid::Uuid;

#[derive(Debug)]
pub struct Http {
    pub(crate) create: String,
    pub(crate) delete: String,
    pub(crate) find_by_id: String,
    pub(crate) find_by_criteria: String,
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
    pub(crate) async fn create(&self, analysis: Analysis) -> Result<(), String> {
        let response = Request::put(&self.create)
            .json(&analysis)
            .map_err(|e| format!("{e}"))?
            .send()
            .await
            .map_err(|e| format!("{e}"))?;

        if response.ok() {
            Ok(())
        } else {
            Err(response.status().to_string())
        }
    }

    pub(crate) async fn delete(&self, id: &Uuid) -> Result<(), String> {
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

    pub(crate) async fn find_by_id(&self, id: &Uuid) -> Result<Option<Analysis>, String> {
        let endpoint = format!("{}/{id}", self.find_by_id);
        let response = Request::get(&endpoint)
            .send()
            .await
            .map_err(|e| format!("{e}"))?;

        if response.ok() {
            let analysis: Analysis = response.json().await.map_err(|e| format!("{e}"))?;
            Ok(Some(analysis))
        } else if response.status() == 404 {
            Ok(None)
        } else {
            Err(response.status().to_string())
        }
    }

    pub(crate) async fn find_by_criteria(
        &self,
        criteria: &str,
    ) -> Result<Option<Analyses>, String> {
        let response = Request::post(&self.find_by_criteria)
            .json(criteria)
            .map_err(|e| format!("{e}"))?
            .send()
            .await
            .map_err(|e| format!("{e}"))?;

        if response.ok() {
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
