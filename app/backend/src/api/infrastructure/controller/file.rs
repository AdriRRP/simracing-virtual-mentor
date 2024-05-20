use crate::file::application::find::by_criteria::service::Finder;
use crate::file::infrastructure::repository::in_memory::InMemory;

use axum::extract::State;
use axum::http::StatusCode;
use std::sync::Arc;

/// # Errors
///
/// Will return `Err` if the service call produces any error
pub async fn find_file_by_criteria(
    State(finder): State<Arc<Finder<InMemory>>>,
) -> Result<StatusCode, (StatusCode, String)> {
    let files = finder.find("put criteria here").await;
    println!("{files:?}");
    Ok(StatusCode::ACCEPTED)
}
