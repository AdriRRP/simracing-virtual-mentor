use crate::api::infrastructure::event::tokio_bus::TokioBus;
use crate::api::infrastructure::repository::mongo::file::Mongo as FileRepository;
use crate::api::infrastructure::repository::mongo::lap::Mongo as LapRepository;
use crate::ibt_extractor::application::extract::service::Extractor as IbtExtractor;

use shared::file::application::find::by_id::service::Finder as FileFinder;

use axum::extract::{Multipart, Path, State};
use axum::http::StatusCode;
use futures_util::TryFutureExt;
use std::io::Cursor;
use std::sync::Arc;

#[derive(Clone)]
pub struct ControllerState {
    pub ibt_parser: Arc<IbtExtractor<FileRepository, LapRepository, TokioBus>>,
    pub file_finder: Arc<FileFinder<FileRepository>>,
}

/// # Errors
///
/// Will return `Err` if file upload fails or if a file with same Sha256 exists
pub async fn upload(
    State(services): State<ControllerState>,
    Path(name): Path<String>,
    mut multipart: Multipart,
) -> Result<StatusCode, (StatusCode, String)> {
    let mut body_bytes = Vec::<u8>::new();

    tracing::debug!("Uploading `.ibt` file `{name}`");

    while let Ok(Some(field)) = multipart.next_field().await {
        tracing::trace!("Receiving `.ibt` data: {field:?}");

        let byte_slice = field
            .bytes()
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
            .await?;

        body_bytes.extend_from_slice(&byte_slice);
    }

    let id = sha256::digest(&body_bytes);

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
