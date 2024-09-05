use crate::infrastructure::settings::Settings;
use log::{error, info, warn};
use reqwest::header::CONTENT_LENGTH;
use reqwest::Client;
use serde::Serialize;

use shared::analysis::domain::analyses::Analyses;
use shared::analysis::domain::analysis::headers::Headers;
use shared::analysis::domain::analysis::Analysis;

use serde::Deserialize;
use shared::common::domain::criteria::Criteria;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Http {
    pub create: String,
    pub delete: String,
    pub find_by_id: String,
    pub find_by_criteria: String,
    pub find_header_by_criteria: String,
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
            find_header_by_criteria: format!(
                "{}{}",
                settings.endpoints.analysis.server,
                settings.endpoints.analysis.find_header_by_criteria
            ),
        }
    }
}

// TODO: Move from here! //////////////////

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Request {
    pub id: Uuid,
    pub name: String,
    pub ref_lap_id: Uuid,
    pub target_lap_id: Uuid,
}

impl Request {
    pub fn new(name: String, ref_lap_id: Uuid, target_lap_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            ref_lap_id,
            target_lap_id,
        }
    }
}

// TODO: ////////////////////////////////

impl Http {
    pub async fn create(&self, request: Request) -> Result<(), String> {
        let response = Client::new()
            .put(&self.create)
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("{e}"))?;

        warn!("{request:?}");

        if response.status().is_success() {
            warn!("{response:?}");
            Ok(())
        } else {
            error!("{response:?}");
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

    pub async fn find_by_criteria(&self, criteria: &Criteria) -> Result<Option<Analyses>, String> {
        let response = Client::new()
            .post(&self.find_by_criteria)
            .header(CONTENT_LENGTH, "application/json")
            .json(criteria)
            .send()
            .await
            .map_err(|e| format!("{e}"))?;

        info!("{:?}", response);

        if response.status().is_success() {
            let json = response.json().await.map_err(|e| {
                error!("{:?}", e);
                format!("{e}")
            })?;
            let analyses: Option<Analyses> = json;
            Ok(analyses)
            //if analyses.is_empty() {
            //    Ok(None)
            //} else {
            //    Ok(Some(analyses))
            //}
        } else {
            Err(response.status().to_string())
        }
    }

    pub(crate) async fn find_header_by_criteria(
        &self,
        criteria: &Criteria,
    ) -> Result<Option<Headers>, String> {
        error!("response: {:?}", &self.find_header_by_criteria);

        let response = Client::new()
            .post(&self.find_header_by_criteria)
            .json(criteria)
            .send()
            .await
            .map_err(|e| format!("{e}"))?;

        error!("response: {response:?}");

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
