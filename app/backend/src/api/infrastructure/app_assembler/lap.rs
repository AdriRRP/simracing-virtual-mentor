use crate::lap::application::create::service::Creator as LapCreator;
use crate::lap::infrastructure::repository::in_memory::InMemory as InMemoryLapRepository;
use crate::shared::domain::event::bus::Bus as EventBus;

use std::sync::Arc;

pub struct LapAssembler {
    pub creator: Arc<LapCreator<InMemoryLapRepository>>,
}

impl LapAssembler {
    pub fn new(event_bus: Arc<dyn EventBus>) -> Self {
        let repository = Arc::new(InMemoryLapRepository::default());
        let creator = Arc::new(LapCreator::new(
            Arc::clone(&repository),
            Arc::clone(&event_bus),
        ));
        Self { creator }
    }
}
