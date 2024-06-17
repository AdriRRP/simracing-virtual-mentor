use crate::api::infrastructure::event::tokio_bus::TokioBus;

use shared::analysis::application::create::service::Creator as AnalysisCreator;
use shared::analysis::application::delete::service::Deleter as AnalysisDeleter;
use shared::analysis::application::find::by_criteria::service::Finder as AnalysisByCriteriaFinder;
use shared::analysis::application::find::by_id::service::Finder as AnalysisByIdFinder;
use shared::analysis::infrastructure::repository::in_memory::InMemory as InMemoryAnalysisRepository;
use shared::lap::infrastructure::repository::in_memory::InMemory as InMemoryLapRepository;

use std::sync::Arc;

pub struct Assembler {
    pub creator: Arc<AnalysisCreator<InMemoryAnalysisRepository, InMemoryLapRepository>>,
    pub deleter: Arc<AnalysisDeleter<InMemoryAnalysisRepository>>,
    pub by_id_finder: Arc<AnalysisByIdFinder<InMemoryAnalysisRepository>>,
    pub by_criteria_finder: Arc<AnalysisByCriteriaFinder<InMemoryAnalysisRepository>>,
}

impl Assembler {
    #[must_use]
    pub fn new(_event_bus: &Arc<TokioBus>, in_memory_lap_repository: &Arc<InMemoryLapRepository>) -> Self {
        let repository = Arc::new(InMemoryAnalysisRepository::default());
        let lap_repository = Arc::clone(in_memory_lap_repository);
        let creator = Arc::new(AnalysisCreator::new(
            Arc::clone(&repository),
            Arc::clone(&lap_repository),
        ));
        let deleter = Arc::new(AnalysisDeleter::new(
            Arc::clone(&repository),
        ));
        let by_id_finder = Arc::new(AnalysisByIdFinder::new(
            Arc::clone(&repository),
        ));
        let by_criteria_finder = Arc::new(AnalysisByCriteriaFinder::new(
            Arc::clone(&repository),
        ));
        Self {
            creator,
            deleter,
            by_id_finder,
            by_criteria_finder,
        }
    }
}