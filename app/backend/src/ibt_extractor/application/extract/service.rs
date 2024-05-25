use shared::file::application::create::service::Creator as FileCreator;
use shared::file::domain::file::File;
use shared::file::domain::repository::Repository as FileRepository;
use shared::lap::application::create::service::Creator as LapCreator;
use shared::lap::domain::repository::Repository as LapRepository;
use shared::ibt::domain::file::File as IbtFile;
use shared::common::domain::event::bus::Bus as EventBus;

use crate::ibt_extractor::domain::event::extracted::Extracted as IbtExtracted;
use crate::ibt_extractor::domain::converter::ibt_metrics2laps;

use std::io::{Read, Seek};
use std::sync::Arc;

#[derive(Debug)]
pub struct Extractor<FR: FileRepository, LR: LapRepository, E: EventBus> {
    file_creator: Arc<FileCreator<FR, E>>,
    lap_creator: Arc<LapCreator<LR>>,
    event_bus: Arc<E>,
}

impl<FR: FileRepository, LR: LapRepository, E: EventBus> Extractor<FR, LR, E> {
    pub fn new(
        file_creator: Arc<FileCreator<FR, E>>,
        lap_creator: Arc<LapCreator<LR>>,
        event_bus: Arc<E>,
    ) -> Self {
        Self {
            file_creator,
            lap_creator,
            event_bus,
        }
    }

    pub async fn parse<ReadSeek: Read + Seek + Send + Sync>(
        &self,
        id: String,
        name: String,
        reader: ReadSeek,
    ) {
        tracing::debug!("Creating file `{}` ({})", name.clone(), id.clone());
        self.file_creator
            .create(File::new(id.clone(), name.clone()))
            .await;

        tracing::debug!("Reading file `{}` ({})", name.clone(), id.clone());
        let mut reader = reader;
        let ibt_file = IbtFile::from_reader(&mut reader);

        tracing::debug!("Creating laps for file `{}` ({})", name.clone(), id.clone());

        match ibt_file {
            Ok(ibt_file) => {
                let laps = ibt_metrics2laps(&id, &ibt_file.session_info, &ibt_file.metrics);
                self.lap_creator.create(laps).await;

                tracing::debug!("Laps for file `{}` ({}) created", name.clone(), id.clone());

                let event = Arc::new(IbtExtracted::new(&id));
                let _ = self.event_bus.dispatch(event).await;
            }
            Err(e) => {
                tracing::error!(
                    "Error Extracting file `{}` ({}): {e}",
                    name.clone(),
                    id.clone()
                );
            }
        }
    }
}
