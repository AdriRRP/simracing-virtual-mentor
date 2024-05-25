use crate::lap::application::create::service::Creator as LapCreator;
use crate::lap::application::delete::service::Deleter as LapDeleter;
use crate::lap::application::find::by_criteria::service::Finder as ByCriteriaLapFinder;
use crate::lap::application::find::by_id::service::Finder as ByIdLapFinder;
use crate::lap::infrastructure::repository::in_memory::InMemory as InMemoryLapRepository;
use crate::shared::infrastructure::event::tokio_bus::TokioBus;

use std::sync::Arc;

pub struct Assembler {
    pub creator: Arc<LapCreator<InMemoryLapRepository>>,
    pub by_id_finder: Arc<ByIdLapFinder<InMemoryLapRepository>>,
    pub by_criteria_finder: Arc<ByCriteriaLapFinder<InMemoryLapRepository>>,
    pub deleter: Arc<LapDeleter<InMemoryLapRepository>>,
}

impl Assembler {
    #[must_use]
    pub fn new(_event_bus: &Arc<TokioBus>) -> Self {
        let repository = Arc::new(InMemoryLapRepository::default());
        let creator = Arc::new(LapCreator::new(Arc::clone(&repository)));
        let by_id_finder = Arc::new(ByIdLapFinder::new(Arc::clone(&repository)));
        let by_criteria_finder = Arc::new(ByCriteriaLapFinder::new(Arc::clone(&repository)));
        let deleter = Arc::new(LapDeleter::new(Arc::clone(&repository)));
        Self {
            creator,
            by_id_finder,
            by_criteria_finder,
            deleter,
        }
    }
}
