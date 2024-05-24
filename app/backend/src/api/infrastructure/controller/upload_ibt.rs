use crate::file::application::find::by_id::service::Finder as FileFinder;
use crate::file::infrastructure::repository::in_memory::InMemory as InMemoryFileRepository;
use crate::ibt_extractor::application::extract::service::Extractor as IbtExtractor;
use crate::lap::infrastructure::repository::in_memory::InMemory as InMemoryLapRepository;
use crate::shared::infrastructure::event::tokio_bus::TokioBus;

use axum::extract::{Multipart, State};
use axum::http::StatusCode;
use futures_util::TryFutureExt;
use std::io::Cursor;
use std::sync::Arc;

#[derive(Clone)]
pub struct ControllerState {
    pub ibt_parser: Arc<IbtExtractor<InMemoryFileRepository, InMemoryLapRepository, TokioBus>>,
    pub file_finder: Arc<FileFinder<InMemoryFileRepository>>,
}

/// # Errors
///
/// Will return `Err` if file upload fails or if a file with same Sha256 exists
pub async fn upload_ibt(
    State(services): State<ControllerState>,
    mut multipart: Multipart,
) -> Result<StatusCode, (StatusCode, String)> {
    let mut body_bytes = Vec::<u8>::new();

    while let Ok(Some(field)) = multipart.next_field().await {
        let byte_slice = field
            .bytes()
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
            .await?;
        body_bytes.extend_from_slice(&byte_slice);
    }

    let id = sha256::digest(&body_bytes);
    let name = "take it from arguments".to_string();

    match services
        .file_finder
        .find(&id)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err))?
    {
        None => {
            let cursor = Cursor::new(body_bytes);
            tokio::spawn(async move {
                services.ibt_parser.parse(id, name, cursor).await;
            });
            Ok(StatusCode::ACCEPTED)
        }
        Some(_) => Err((
            StatusCode::CONFLICT,
            format!("There is already a File in the system with Sha256 `{id}`"),
        )),
    }
}
