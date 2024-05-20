use crate::api::domain::converter::ibt_metrics2laps;
use crate::api::domain::event::ibt_parsed::IbtParsed;
use crate::file::application::create::service::Creator as FileCreator;
use crate::file::domain::file::File;
use crate::file::domain::repository::Repository as FileRepository;
use crate::ibt::domain::file::File as IbtFile;
use crate::lap::application::create::service::Creator as LapCreator;
use crate::lap::domain::repository::Repository as LapRepository;
use crate::shared::domain::event::bus::Bus as EventBus;

use std::io::{Read, Seek};
use std::sync::Arc;

pub struct IbtParser<FR: FileRepository, LR: LapRepository> {
    event_bus: Arc<dyn EventBus>,
    file_creator: Arc<FileCreator<FR>>,
    lap_creator: Arc<LapCreator<LR>>,
}

impl<FR: FileRepository, LR: LapRepository> IbtParser<FR, LR> {
    pub fn new(
        event_bus: Arc<dyn EventBus>,
        file_creator: Arc<FileCreator<FR>>,
        lap_creator: Arc<LapCreator<LR>>,
    ) -> Self {
        Self {
            event_bus,
            file_creator,
            lap_creator,
        }
    }

    pub async fn parse<ReadSeek: Read + Seek + Send + Sync>(
        &self,
        id: String,
        name: String,
        reader: ReadSeek,
    ) {
        println!("Creating file");
        self.file_creator.create(File::new(id.clone(), name)).await;
        println!("File created");

        println!("Parsing ibt file");
        let mut reader = reader;
        let ibt_file = IbtFile::from_reader(&mut reader);
        println!("Ibt file parsed");

        println!("Creating laps");

        match ibt_file {
            Ok(ibt_file) => {
                let laps = ibt_metrics2laps(&id, &ibt_file.session_info, &ibt_file.metrics);
                self.lap_creator.create(laps).await;
                println!("Laps created");
                let event = Arc::new(IbtParsed::new(&id));
                let _ = self.event_bus.dispatch(event).await;
            }
            Err(e) => {
                println!("Error parsing file: {e}");
            }
        }
    }
}
