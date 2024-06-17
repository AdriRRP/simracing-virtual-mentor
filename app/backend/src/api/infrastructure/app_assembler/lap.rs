use shared::lap::application::create::service::Creator as LapCreator;
use shared::lap::application::delete::service::Deleter as LapDeleter;
use shared::lap::application::find::by_criteria::service::Finder as ByCriteriaLapFinder;
use shared::lap::application::find::by_id::service::Finder as ByIdLapFinder;
use shared::lap::application::find::header_by_id::service::Finder as ByIdLapHeaderFinder;
use shared::lap::application::find::headers_by_criteria::service::Finder as ByCriteriaLapHeaderFinder;
use shared::lap::infrastructure::repository::in_memory::InMemory as InMemoryLapRepository;

use crate::api::infrastructure::event::tokio_bus::TokioBus;

use std::sync::Arc;

pub struct Assembler {
    pub repository: Arc<InMemoryLapRepository>,
    pub creator: Arc<LapCreator<InMemoryLapRepository>>,
    pub by_id_finder: Arc<ByIdLapFinder<InMemoryLapRepository>>,
    pub by_criteria_finder: Arc<ByCriteriaLapFinder<InMemoryLapRepository>>,
    pub by_id_header_finder: Arc<ByIdLapHeaderFinder<InMemoryLapRepository>>,
    pub by_criteria_header_finder: Arc<ByCriteriaLapHeaderFinder<InMemoryLapRepository>>,
    pub deleter: Arc<LapDeleter<InMemoryLapRepository>>,
}

impl Assembler {
    #[must_use]
    pub fn new(_event_bus: &Arc<TokioBus>) -> Self {
        let repository = Arc::new(InMemoryLapRepository::default());
        let creator = Arc::new(LapCreator::new(Arc::clone(&repository)));
        let by_id_finder = Arc::new(ByIdLapFinder::new(Arc::clone(&repository)));
        let by_criteria_finder = Arc::new(ByCriteriaLapFinder::new(Arc::clone(&repository)));
        let by_id_header_finder = Arc::new(ByIdLapHeaderFinder::new(Arc::clone(&repository)));
        let by_criteria_header_finder =
            Arc::new(ByCriteriaLapHeaderFinder::new(Arc::clone(&repository)));
        let deleter = Arc::new(LapDeleter::new(Arc::clone(&repository)));
        Self {
            repository,
            creator,
            by_id_finder,
            by_criteria_finder,
            by_id_header_finder,
            by_criteria_header_finder,
            deleter,
        }
    }
}
