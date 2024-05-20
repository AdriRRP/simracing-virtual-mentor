use crate::api::domain::converter::ibt_metrics2laps;
use crate::file::application::create::service::Creator as FileCreator;
use crate::file::application::find::by_id::service::Finder as FileFinder;
use crate::file::domain::file::File;
use crate::file::infrastructure::repository::in_memory::InMemory as InMemoryFileRepository;
use crate::ibt::domain::file::File as IbtFile;
use crate::lap::application::create::service::Creator as LapCreator;
use crate::lap::infrastructure::repository::in_memory::InMemory as InMemoryLapRepository;

use axum::extract::{Multipart, State};
use axum::http::StatusCode;
use futures_util::TryFutureExt;
use std::io::Cursor;
use std::sync::Arc;

/// # Errors
///
/// Will return `Err` if file upload fails or if a file with same Sha256 exists
#[allow(clippy::type_complexity)]
pub async fn upload_ibt(
    // TODO: very complex type used. Consider factoring parts into `type` definitions
    State((file_creator, lap_creator, finder)): State<(
        Arc<FileCreator<InMemoryFileRepository>>,
        Arc<LapCreator<InMemoryLapRepository>>,
        Arc<FileFinder<InMemoryFileRepository>>,
    )>,
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

    match finder
        .find(&id)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err))?
    {
        None => {
            let cursor = Cursor::new(body_bytes);
            let file_creator = Arc::clone(&file_creator);
            let lap_creator = Arc::clone(&lap_creator);

            tokio::spawn(async move {
                parse_ibt(id, name, cursor, file_creator, lap_creator).await;
            });

            Ok(StatusCode::ACCEPTED)
        }
        Some(_) => Err((
            StatusCode::CONFLICT,
            format!("There is already a File in the system with Sha256 `{id}`"),
        )),
    }
}

// TODO: Move logic to application
async fn parse_ibt(
    id: String,
    name: String,
    reader: Cursor<Vec<u8>>,
    file_creator: Arc<FileCreator<InMemoryFileRepository>>,
    lap_creator: Arc<LapCreator<InMemoryLapRepository>>,
) {
    println!("Creating file");
    file_creator.create(File::new(id.clone(), name)).await;
    println!("File created");

    println!("Parsing ibt file");
    let mut reader = reader;
    let ibt_file = IbtFile::from_reader(&mut reader);
    println!("Ibt file parsed");

    println!("Creating laps");

    match ibt_file {
        Ok(ibt_file) => {
            let laps = ibt_metrics2laps(&id, &ibt_file.session_info, &ibt_file.metrics);
            println!("{}", serde_yaml::to_string(&laps).unwrap());
            lap_creator.create(&id, laps).await;
            println!("Laps created");
        }
        Err(e) => {
            println!("Error parsing file: {e}");
        }
    }
}
