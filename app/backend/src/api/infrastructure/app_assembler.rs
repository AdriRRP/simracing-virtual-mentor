pub mod file;
pub mod lap;

use crate::api::infrastructure::app_assembler::file::FileAssembler;
use crate::api::infrastructure::app_assembler::lap::LapAssembler;
use crate::shared::domain::event::bus::Bus as EventBus;
use crate::shared::infrastructure::event::tokio_bus::TokioBus as TokioEventBus;

use std::sync::Arc;

pub struct AppAssembler {
    pub event_bus: Arc<dyn EventBus>,
    pub file: FileAssembler,
    pub lap: LapAssembler,
}

impl AppAssembler {
    pub fn new() -> Self {
        let event_bus: Arc<dyn EventBus> = Arc::new(TokioEventBus::new(200));
        let file = FileAssembler::new(Arc::clone(&event_bus));
        let lap = LapAssembler::new(Arc::clone(&event_bus));
        Self {
            event_bus,
            file,
            lap,
        }
    }
}
