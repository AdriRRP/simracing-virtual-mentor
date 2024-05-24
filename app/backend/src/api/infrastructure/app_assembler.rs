pub mod file;
pub mod ibt;
pub mod lap;

use crate::api::infrastructure::app_assembler::file::Assembler as FileAssembler;
use crate::api::infrastructure::app_assembler::ibt::Assembler as IbtAssembler;
use crate::api::infrastructure::app_assembler::lap::Assembler as LapAssembler;
use crate::api::infrastructure::settings::Settings;
use crate::shared::infrastructure::event::tokio_bus::TokioBus as TokioEventBus;

use std::sync::Arc;

pub struct AppAssembler {
    pub event_bus: Arc<TokioEventBus>,
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
        let ibt = IbtAssembler::new(&event_bus, &file.creator, &lap.creator);

        Self {
            event_bus,
            file,
            lap,
            ibt,
        }
    }
}
