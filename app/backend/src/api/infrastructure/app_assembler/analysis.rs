use crate::api::infrastructure::event::tokio_bus::TokioBus;
use crate::api::infrastructure::repository::mongo::analysis::Mongo as AnalysisRepository;
use crate::api::infrastructure::repository::mongo::lap::Mongo as LapRepository;
use crate::api::infrastructure::settings::Settings;

use shared::analysis::application::analyze::service::Analyzer;
use shared::analysis::application::create::service::Creator as AnalysisCreator;
use shared::analysis::application::delete::service::Deleter as AnalysisDeleter;
use shared::analysis::application::find::by_criteria::service::Finder as AnalysisByCriteriaFinder;
use shared::analysis::application::find::by_id::service::Finder as AnalysisByIdFinder;
use shared::analysis::application::find::header_by_criteria::service::Finder as AnalysisHeaderByCriteriaFinder;
use shared::analysis::application::find::header_by_id::service::Finder as AnalysisHeaderByIdFinder;
use shared::analysis::application::update::service::Updater as AnalysisUpdater;

use std::sync::Arc;
use shared::analysis::domain::analysis::fcm_grid::Config;

pub struct Assembler {
    pub analyzer: Arc<Analyzer<AnalysisRepository, LapRepository>>,
    pub creator: Arc<AnalysisCreator<AnalysisRepository, LapRepository, TokioBus>>,
    pub updater: Arc<AnalysisUpdater<AnalysisRepository>>,
    pub deleter: Arc<AnalysisDeleter<AnalysisRepository>>,
    pub by_id_finder: Arc<AnalysisByIdFinder<AnalysisRepository>>,
    pub by_criteria_finder: Arc<AnalysisByCriteriaFinder<AnalysisRepository>>,
    pub by_id_header_finder: Arc<AnalysisHeaderByIdFinder<AnalysisRepository>>,
    pub by_criteria_header_finder: Arc<AnalysisHeaderByCriteriaFinder<AnalysisRepository>>,
    pub fcm_grid_config: Config,
}

impl Assembler {
    /// # Errors
    ///
    /// This function will return an error in the following situations:
    ///
    /// * If `AnalysisRepository::new` fails, it will return an error string propagated from that function.
    /// * If any of the following initializations fail, they will return an error string propagated
    ///   from their respective constructors:
    ///     * `Analyzer::new`
    ///     * `AnalysisCreator::new`
    ///     * `AnalysisUpdater::new`
    ///     * `AnalysisDeleter::new`
    ///     * `AnalysisByIdFinder::new`
    ///     * `AnalysisByCriteriaFinder::new`
    ///     * `AnalysisHeaderByIdFinder::new`
    ///     * `AnalysisHeaderByCriteriaFinder::new`
    ///
    /// Each of these functions could fail due to various reasons such as configuration issues, 
    /// resource allocation failures, or other runtime errors specific to the initialization process
    /// of each component.
    pub async fn new(
        settings: &Settings,
        event_bus: &Arc<TokioBus>,
        in_memory_lap_repository: &Arc<LapRepository>,
    ) -> Result<Self, String> {
        let repository = Arc::new(AnalysisRepository::new(settings).await?);
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

        let fcm_grid_config = Config::new(
            (settings.fcm_grid.c.init, settings.fcm_grid.c.max, settings.fcm_grid.c.inc),
            (settings.fcm_grid.m.init, settings.fcm_grid.m.max, settings.fcm_grid.m.inc),
            (settings.fcm_grid.max_iter.init, settings.fcm_grid.max_iter.max, settings.fcm_grid.max_iter.inc),
            (settings.fcm_grid.error.init, settings.fcm_grid.error.max, settings.fcm_grid.error.inc),
        );

        Ok(Self {
            analyzer,
            creator,
            updater,
            deleter,
            by_id_finder,
            by_criteria_finder,
            by_id_header_finder,
            by_criteria_header_finder,
            fcm_grid_config,
        })
    }
}
