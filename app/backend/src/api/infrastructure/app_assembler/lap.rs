use crate::api::infrastructure::event::tokio_bus::TokioBus;
use crate::api::infrastructure::repository::mongo::lap::Mongo as LapRepository;

use shared::lap::application::create::service::Creator as LapCreator;
use shared::lap::application::delete::service::Deleter as LapDeleter;
use shared::lap::application::find::by_criteria::service::Finder as ByCriteriaLapFinder;
use shared::lap::application::find::by_id::service::Finder as ByIdLapFinder;
use shared::lap::application::find::header_by_id::service::Finder as ByIdLapHeaderFinder;
use shared::lap::application::find::headers_by_criteria::service::Finder as ByCriteriaLapHeaderFinder;

use crate::api::infrastructure::settings::Settings;
use std::sync::Arc;

pub struct Assembler {
    pub repository: Arc<LapRepository>,
    pub creator: Arc<LapCreator<LapRepository>>,
    pub by_id_finder: Arc<ByIdLapFinder<LapRepository>>,
    pub by_criteria_finder: Arc<ByCriteriaLapFinder<LapRepository>>,
    pub by_id_header_finder: Arc<ByIdLapHeaderFinder<LapRepository>>,
    pub by_criteria_header_finder: Arc<ByCriteriaLapHeaderFinder<LapRepository>>,
    pub deleter: Arc<LapDeleter<LapRepository>>,
}

impl Assembler {
    /// # Errors
    ///
    /// This function will return an error in the following situations:
    ///
    /// * If `LapRepository::new` fails, it will return an error string propagated from that function.
    /// * If any of the following initializations fail, they will return an error string propagated
    ///   from their respective constructors:
    ///     * `LapCreator::new`
    ///     * `ByIdLapFinder::new`
    ///     * `ByCriteriaLapFinder::new`
    ///     * `ByIdLapHeaderFinder::new`
    ///     * `ByCriteriaLapHeaderFinder::new`
    ///     * `LapDeleter::new`
    ///
    /// Each of these functions could fail due to various reasons such as configuration issues,
    /// resource allocation failures, or other runtime errors specific to the initialization process
    /// of each component.
    pub async fn new(settings: &Settings, _event_bus: &Arc<TokioBus>) -> Result<Self, String> {
        let repository = Arc::new(LapRepository::new(settings).await?);
        let creator = Arc::new(LapCreator::new(Arc::clone(&repository)));
        let by_id_finder = Arc::new(ByIdLapFinder::new(Arc::clone(&repository)));
        let by_criteria_finder = Arc::new(ByCriteriaLapFinder::new(Arc::clone(&repository)));
        let by_id_header_finder = Arc::new(ByIdLapHeaderFinder::new(Arc::clone(&repository)));
        let by_criteria_header_finder =
            Arc::new(ByCriteriaLapHeaderFinder::new(Arc::clone(&repository)));
        let deleter = Arc::new(LapDeleter::new(Arc::clone(&repository)));
        Ok(Self {
            repository,
            creator,
            by_id_finder,
            by_criteria_finder,
            by_id_header_finder,
            by_criteria_header_finder,
            deleter,
        })
    }
}
