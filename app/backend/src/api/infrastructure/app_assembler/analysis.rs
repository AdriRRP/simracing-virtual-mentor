use crate::api::infrastructure::event::tokio_bus::TokioBus;

use shared::analysis::application::analyze::service::Analyzer;
use shared::analysis::application::create::service::Creator as AnalysisCreator;
use shared::analysis::application::delete::service::Deleter as AnalysisDeleter;
use shared::analysis::application::find::by_criteria::service::Finder as AnalysisByCriteriaFinder;
use shared::analysis::application::find::by_id::service::Finder as AnalysisByIdFinder;
use shared::analysis::application::find::header_by_criteria::service::Finder as AnalysisHeaderByCriteriaFinder;
use shared::analysis::application::find::header_by_id::service::Finder as AnalysisHeaderByIdFinder;
use shared::analysis::application::update::service::Updater as AnalysisUpdater;
use shared::analysis::infrastructure::repository::in_memory::InMemory as InMemoryAnalysisRepository;
use shared::lap::infrastructure::repository::in_memory::InMemory as InMemoryLapRepository;

use std::sync::Arc;

pub struct Assembler {
    pub analyzer: Arc<Analyzer<InMemoryAnalysisRepository, InMemoryLapRepository>>,
    pub creator: Arc<AnalysisCreator<InMemoryAnalysisRepository, InMemoryLapRepository, TokioBus>>,
    pub updater: Arc<AnalysisUpdater<InMemoryAnalysisRepository>>,
    pub deleter: Arc<AnalysisDeleter<InMemoryAnalysisRepository>>,
    pub by_id_finder: Arc<AnalysisByIdFinder<InMemoryAnalysisRepository>>,
    pub by_criteria_finder: Arc<AnalysisByCriteriaFinder<InMemoryAnalysisRepository>>,
    pub by_id_header_finder: Arc<AnalysisHeaderByIdFinder<InMemoryAnalysisRepository>>,
    pub by_criteria_header_finder: Arc<AnalysisHeaderByCriteriaFinder<InMemoryAnalysisRepository>>,
}

impl Assembler {
    #[must_use]
    pub fn new(
        event_bus: &Arc<TokioBus>,
        in_memory_lap_repository: &Arc<InMemoryLapRepository>,
    ) -> Self {
        let repository = Arc::new(InMemoryAnalysisRepository::default());
        let lap_repository = Arc::clone(in_memory_lap_repository);
        let analyzer = Arc::new(Analyzer::new(
            Arc::clone(&repository),
            Arc::clone(&lap_repository),
        ));
        let creator = Arc::new(AnalysisCreator::new(
            Arc::clone(&repository),
            Arc::clone(&lap_repository),
            Arc::clone(event_bus),
        ));
        let updater = Arc::new(AnalysisUpdater::new(Arc::clone(&repository)));
        let deleter = Arc::new(AnalysisDeleter::new(Arc::clone(&repository)));
        let by_id_finder = Arc::new(AnalysisByIdFinder::new(Arc::clone(&repository)));
        let by_criteria_finder = Arc::new(AnalysisByCriteriaFinder::new(Arc::clone(&repository)));
        let by_id_header_finder = Arc::new(AnalysisHeaderByIdFinder::new(Arc::clone(&repository)));
        let by_criteria_header_finder =
            Arc::new(AnalysisHeaderByCriteriaFinder::new(Arc::clone(&repository)));
        Self {
            analyzer,
            creator,
            updater,
            deleter,
            by_id_finder,
            by_criteria_finder,
            by_id_header_finder,
            by_criteria_header_finder,
        }
    }
}
