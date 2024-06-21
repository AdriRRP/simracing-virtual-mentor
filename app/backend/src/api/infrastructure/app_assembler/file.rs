use crate::api::infrastructure::event::tokio_bus::TokioBus;
use crate::api::infrastructure::repository::mongo::file::Mongo as FileRepository;
use crate::api::infrastructure::settings::Settings;

use shared::file::application::create::service::Creator as FileCreator;
use shared::file::application::delete::service::Deleter as FileDeleter;
use shared::file::application::find::by_criteria::service::Finder as FileByCriteriaFinder;
use shared::file::application::find::by_id::service::Finder as FileByIdFinder;
use shared::file::application::validate::service::Validator as FileValidator;

use std::sync::Arc;

pub struct Assembler {
    pub creator: Arc<FileCreator<FileRepository, TokioBus>>,
    pub deleter: Arc<FileDeleter<FileRepository, TokioBus>>,
    pub by_id_finder: Arc<FileByIdFinder<FileRepository>>,
    pub by_criteria_finder: Arc<FileByCriteriaFinder<FileRepository>>,
    pub validator: Arc<FileValidator<FileRepository, TokioBus>>,
}

impl Assembler {
    /// # Errors
    ///
    /// This function will return an error in the following situations:
    ///
    /// * If `FileRepository::new` fails, it will return an error string propagated from that function.
    /// * If any of the following initializations fail, they will return an error string propagated
    ///   from their respective constructors:
    ///     * `FileCreator::new`
    ///     * `FileDeleter::new`
    ///     * `FileByIdFinder::new`
    ///     * `FileByCriteriaFinder::new`
    ///     * `FileValidator::new`
    ///
    /// Each of these functions could fail due to various reasons such as configuration issues, 
    /// resource allocation failures, or other runtime errors specific to the initialization process
    /// of each component.
    pub async fn new(settings: &Settings, event_bus: &Arc<TokioBus>) -> Result<Self, String> {
        let repository = Arc::new(FileRepository::new(settings).await?);
        let creator = Arc::new(FileCreator::new(
            Arc::clone(&repository),
            Arc::clone(event_bus),
        ));
        let deleter = Arc::new(FileDeleter::new(
            Arc::clone(&repository),
            Arc::clone(event_bus),
        ));
        let by_id_finder = Arc::new(FileByIdFinder::new(Arc::clone(&repository)));
        let by_criteria_finder = Arc::new(FileByCriteriaFinder::new(Arc::clone(&repository)));
        let validator = Arc::new(FileValidator::new(
            Arc::clone(&repository),
            Arc::clone(event_bus),
        ));

        Ok(Self {
            creator,
            deleter,
            by_id_finder,
            by_criteria_finder,
            validator,
        })
    }
}
