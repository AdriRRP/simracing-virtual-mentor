use crate::api::infrastructure::event::tokio_bus::TokioBus;
use shared::file::application::delete::service::Deleter;
use shared::file::application::find::by_criteria::service::Finder as ByCriteriaFinder;
use shared::file::application::find::by_id::service::Finder as ByIdFinder;
use shared::file::domain::file::File;
use shared::file::domain::files::Files;
use shared::file::infrastructure::repository::in_memory::InMemory;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use std::sync::Arc;

/// # Errors
///
/// Will return `Err` if the service call produces any error
pub async fn find_by_criteria(
    State(finder): State<Arc<ByCriteriaFinder<InMemory>>>,
) -> Result<Json<Files>, (StatusCode, String)> {
    let criteria = "put criteria here";
    let files = finder.find(criteria).await;
    match files {
        Ok(Some(files)) => Ok(Json(files)),
        Ok(None) => {
            let msg = if criteria.is_empty() {
                "No files found"
            } else {
                "No files found with given criteria"
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
    Path(user_id): Path<String>,
) -> Result<Json<File>, (StatusCode, String)> {
    let file = finder.find(&user_id).await;
    match file {
        Ok(Some(file)) => Ok(Json(file)),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            format!("No file found with id `{user_id}`"),
        )),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

/// # Errors
///
/// Will return `Err` if the service call produces any error
pub async fn delete(
    State(deleter): State<Arc<Deleter<InMemory, TokioBus>>>,
    Path(user_id): Path<String>,
) -> Result<(), (StatusCode, String)> {
    let result = deleter.delete(&user_id).await;
    result.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
}
