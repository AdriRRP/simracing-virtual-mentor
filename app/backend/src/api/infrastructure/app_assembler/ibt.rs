use crate::api::application::parse_ibt::service::IbtParser;
use crate::file::application::create::service::Creator as FileCreator;
use crate::file::infrastructure::repository::in_memory::InMemory as InMemoryFileRepository;
use crate::lap::application::create::service::Creator as LapCreator;
use crate::lap::infrastructure::repository::in_memory::InMemory as InMemoryLapRepository;
use crate::shared::domain::event::bus::Bus as EventBus;

use std::sync::Arc;

pub struct Assembler {
    pub parser: Arc<IbtParser<InMemoryFileRepository, InMemoryLapRepository>>,
}

impl Assembler {
    pub fn new(event_bus: &Arc<dyn EventBus>) -> Self {
        let file_repository = Arc::new(InMemoryFileRepository::default());
        let file_creator = Arc::new(FileCreator::new(file_repository, Arc::clone(event_bus)));
        let lap_repository = Arc::new(InMemoryLapRepository::default());
        let lap_creator = Arc::new(LapCreator::new(lap_repository));
        let parser = Arc::new(IbtParser::new(
            Arc::clone(event_bus),
            file_creator,
            lap_creator,
        ));
        Self { parser }
    }
}
