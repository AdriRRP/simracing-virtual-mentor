pub mod analysis;
pub mod file;
pub mod ibt;
pub mod lap;

use crate::api::infrastructure::app_assembler::analysis::Assembler as AnalysisAssembler;
use crate::api::infrastructure::app_assembler::file::Assembler as FileAssembler;
use crate::api::infrastructure::app_assembler::ibt::Assembler as IbtAssembler;
use crate::api::infrastructure::app_assembler::lap::Assembler as LapAssembler;
use crate::api::infrastructure::settings::Settings;

use crate::api::infrastructure::event::tokio_bus::TokioBus as TokioEventBus;

use std::sync::Arc;

pub struct AppAssembler {
    pub event_bus: Arc<TokioEventBus>,
    pub analysis: AnalysisAssembler,
    pub file: FileAssembler,
    pub lap: LapAssembler,
    pub ibt: IbtAssembler,
}

impl AppAssembler {
    #[must_use]
    pub fn new(settings: &Settings) -> Self {
        let event_bus = Arc::new(TokioEventBus::new(settings.event_bus.capacity));

        let file = FileAssembler::new(&event_bus);
        let lap = LapAssembler::new(&event_bus);
        let analysis = AnalysisAssembler::new(&event_bus, &lap.repository);
        let ibt = IbtAssembler::new(&event_bus, &file.creator, &lap.creator);

        Self {
            event_bus,
            analysis,
            file,
            lap,
            ibt,
        }
    }
}
