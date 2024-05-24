use crate::file::application::create::service::Creator as FileCreator;
use crate::file::infrastructure::repository::in_memory::InMemory as InMemoryFileRepository;
use crate::ibt_extractor::application::extract::service::Extractor as IbtExtractor;
use crate::lap::application::create::service::Creator as LapCreator;
use crate::lap::infrastructure::repository::in_memory::InMemory as InMemoryLapRepository;
use crate::shared::infrastructure::event::tokio_bus::TokioBus;

use std::sync::Arc;

pub struct Assembler {
    pub parser: Arc<IbtExtractor<InMemoryFileRepository, InMemoryLapRepository, TokioBus>>,
}

impl Assembler {
    #[must_use]
    pub fn new(
        event_bus: &Arc<TokioBus>,
        file_creator: &Arc<FileCreator<InMemoryFileRepository, TokioBus>>,
        lap_creator: &Arc<LapCreator<InMemoryLapRepository>>,
    ) -> Self {
        let parser = Arc::new(IbtExtractor::new(
            Arc::clone(file_creator),
            Arc::clone(lap_creator),
            Arc::clone(event_bus),
        ));
        Self { parser }
    }
}
