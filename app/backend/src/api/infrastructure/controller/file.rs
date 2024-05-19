use crate::file::application::find::by_criteria::service::Finder;
use crate::file::infrastructure::repository::in_memory::InMemory;

use axum::extract::State;
use axum::http::StatusCode;
use std::sync::Arc;

pub async fn find_file_by_criteria(
    State(finder): State<Arc<Finder<InMemory>>>,
) -> Result<StatusCode, (StatusCode, String)> {
    let files = finder.find(&"".to_string()).await;
    println!("{:?}", files);
    Ok(StatusCode::ACCEPTED)
}
