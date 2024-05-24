use crate::file::application::find::by_criteria::service::Finder;
use crate::file::domain::files::Files;
use crate::file::infrastructure::repository::in_memory::InMemory;

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use std::sync::Arc;

/// # Errors
///
/// Will return `Err` if the service call produces any error
pub async fn find_file_by_criteria(
    State(finder): State<Arc<Finder<InMemory>>>,
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
