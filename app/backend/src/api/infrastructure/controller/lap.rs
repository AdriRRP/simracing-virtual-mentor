use shared::lap::application::delete::service::Deleter;
use shared::lap::application::find::by_criteria::service::Finder as ByCriteriaFinder;
use shared::lap::application::find::by_id::service::Finder as ByIdFinder;
use shared::lap::application::find::header_by_id::service::Finder as ByIdHeaderFinder;
use shared::lap::application::find::headers_by_criteria::service::Finder as ByCriteriaHeadersFinder;
use shared::lap::domain::lap::header::Header;
use shared::lap::domain::lap::headers::Headers;
use shared::lap::domain::lap::Lap;
use shared::lap::domain::laps::Laps;
use shared::lap::infrastructure::repository::in_memory::InMemory;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use shared::common::domain::criteria::Criteria;
use std::sync::Arc;
use uuid::Uuid;

/// # Errors
///
/// Will return `Err` if the service call produces any error
pub async fn find_by_criteria(
    State(finder): State<Arc<ByCriteriaFinder<InMemory>>>,
    Json(criteria): Json<Criteria>,
) -> Result<Json<Laps>, (StatusCode, String)> {
    let laps = finder.find(&criteria).await;
    match laps {
        Ok(Some(laps)) => Ok(Json(laps)),
        Ok(None) => {
            let msg = if criteria.is_empty() {
                "No laps found"
            } else {
                "No laps found with given criteria"
            };
            Err((StatusCode::NOT_FOUND, msg.to_string()))
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

/// # Errors
///
/// Will return `Err` if the service call produces any error
pub async fn find_by_id(
    State(finder): State<Arc<ByIdFinder<InMemory>>>,
    Path(lap_id): Path<Uuid>,
) -> Result<Json<Lap>, (StatusCode, String)> {
    let lap = finder.find(&lap_id).await;
    match lap {
        Ok(Some(lap)) => Ok(Json(lap)),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            format!("No lap found with id `{lap_id}`"),
        )),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

/// # Errors
///
/// Will return `Err` if the service call produces any error
pub async fn find_headers_by_criteria(
    State(finder): State<Arc<ByCriteriaHeadersFinder<InMemory>>>,
    Json(criteria): Json<Criteria>,
) -> Result<Json<Headers>, (StatusCode, String)> {
    let headers = finder.find(&criteria).await;
    match headers {
        Ok(Some(headers)) => Ok(Json(headers)),
        Ok(None) => {
            let msg = if criteria.is_empty() {
                "No headers found"
            } else {
                "No headers found with given criteria"
            };
            Err((StatusCode::NOT_FOUND, msg.to_string()))
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

/// # Errors
///
/// Will return `Err` if the service call produces any error
pub async fn find_header_by_id(
    State(finder): State<Arc<ByIdHeaderFinder<InMemory>>>,
    Path(lap_id): Path<Uuid>,
) -> Result<Json<Header>, (StatusCode, String)> {
    let header = finder.find(&lap_id).await;
    match header {
        Ok(Some(header)) => Ok(Json(header)),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            format!("No lap header found with lap id `{lap_id}`"),
        )),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

/// # Errors
///
/// Will return `Err` if the service call produces any error
pub async fn delete(
    State(deleter): State<Arc<Deleter<InMemory>>>,
    Path(lap_id): Path<Uuid>,
) -> Result<(), (StatusCode, String)> {
    let result = deleter.delete(&lap_id).await;
    result.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
}
