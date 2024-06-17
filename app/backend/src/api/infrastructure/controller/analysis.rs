use shared::analysis::application::delete::service::Deleter;
use shared::analysis::application::create::service::Creator;
use shared::analysis::application::find::by_criteria::service::Finder as ByCriteriaFinder;
use shared::analysis::application::find::by_id::service::Finder as ByIdFinder;
use shared::analysis::domain::analysis::Analysis;
use shared::analysis::domain::analyses::Analyses;
use shared::analysis::infrastructure::repository::in_memory::InMemory;
use shared::lap::infrastructure::repository::in_memory::InMemory as InMemoryLap;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use std::sync::Arc;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct Args {
    id: Uuid,
    name: String,
    ref_lap_id: Uuid,
    target_lap_id: Uuid,
}

/// # Errors
///
/// Will return `Err` if the service call produces any error
pub async fn creator(
    State(creator): State<Arc<Creator<InMemory, InMemoryLap>>>,
    Json(args): Json<Args>
) -> Result<StatusCode, (StatusCode, String)> {
    
    match creator.create(
        args.id,
        args.name,
        args.ref_lap_id,
        args.target_lap_id,
    ).await {
        Ok(()) => {
            Ok(StatusCode::CREATED)
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

/// # Errors
///
/// Will return `Err` if the service call produces any error
pub async fn find_by_criteria(
    State(finder): State<Arc<ByCriteriaFinder<InMemory>>>,
) -> Result<Json<Analyses>, (StatusCode, String)> {
    let criteria = "put criteria here";
    let analyses = finder.find(criteria).await;
    match analyses {
        Ok(Some(analyses)) => Ok(Json(analyses)),
        Ok(None) => {
            let msg = if criteria.is_empty() {
                "No analyses found"
            } else {
                "No analyses found with given criteria"
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
    Path(analysis_id): Path<Uuid>,
) -> Result<Json<Analysis>, (StatusCode, String)> {
    let analysis = finder.find(&analysis_id).await;
    match analysis {
        Ok(Some(analysis)) => Ok(Json(analysis)),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            format!("No analysis found with id `{analysis_id}`"),
        )),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

/// # Errors
///
/// Will return `Err` if the service call produces any error
pub async fn delete(
    State(deleter): State<Arc<Deleter<InMemory>>>,
    Path(analysis_id): Path<Uuid>,
) -> Result<(), (StatusCode, String)> {
    let result = deleter.delete(&analysis_id).await;
    result.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
}
