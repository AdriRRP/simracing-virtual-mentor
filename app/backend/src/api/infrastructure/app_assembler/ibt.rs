use crate::api::application::parse_ibt::service::IbtParser;
use crate::file::application::create::service::Creator as FileCreator;
use crate::file::infrastructure::repository::in_memory::InMemory as InMemoryFileRepository;
use crate::lap::application::create::service::Creator as LapCreator;
use crate::lap::infrastructure::repository::in_memory::InMemory as InMemoryLapRepository;
use crate::shared::infrastructure::event::tokio_bus::TokioBus;

use std::sync::Arc;

pub struct Assembler {
    pub parser: Arc<IbtParser<InMemoryFileRepository, InMemoryLapRepository, TokioBus>>,
}

impl Assembler {
    #[must_use]
    pub fn new(event_bus: &Arc<TokioBus>) -> Self {
        let file_repository = Arc::new(InMemoryFileRepository::default());
        let file_creator = Arc::new(FileCreator::new(file_repository, Arc::clone(event_bus)));
        
        let lap_repository = Arc::new(InMemoryLapRepository::default());
        let lap_creator = Arc::new(LapCreator::new(lap_repository));
        
        let parser = Arc::new(IbtParser::new(
            file_creator,
            lap_creator,
            Arc::clone(event_bus),
        ));
        
        Self { parser }
    }
}
