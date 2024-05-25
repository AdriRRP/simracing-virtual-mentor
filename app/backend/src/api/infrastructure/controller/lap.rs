use crate::lap::application::delete::service::Deleter;
use crate::lap::application::find::by_criteria::service::Finder as ByCriteriaFinder;
use crate::lap::application::find::by_id::service::Finder as ByIdFinder;
use crate::lap::domain::lap::Lap;
use crate::lap::domain::laps::Laps;
use crate::lap::infrastructure::repository::in_memory::InMemory;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use std::sync::Arc;
use uuid::Uuid;

/// # Errors
///
/// Will return `Err` if the service call produces any error
pub async fn find_by_criteria(
    State(finder): State<Arc<ByCriteriaFinder<InMemory>>>,
) -> Result<Json<Laps>, (StatusCode, String)> {
    let criteria = "put criteria here";
    let laps = finder.find(criteria).await;
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
pub async fn delete(
    State(deleter): State<Arc<Deleter<InMemory>>>,
    Path(lap_id): Path<Uuid>,
) -> Result<(), (StatusCode, String)> {
    let result = deleter.delete(&lap_id).await;
    result.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
}
