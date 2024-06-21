use crate::api::infrastructure::event::tokio_bus::TokioBus;
use crate::api::infrastructure::repository::mongo::file::Mongo as FileRepository;
use crate::api::infrastructure::repository::mongo::lap::Mongo as LapRepository;
use crate::ibt_extractor::application::extract::service::Extractor as IbtExtractor;

use shared::file::application::create::service::Creator as FileCreator;
use shared::lap::application::create::service::Creator as LapCreator;

use std::sync::Arc;

pub struct Assembler {
    pub parser: Arc<IbtExtractor<FileRepository, LapRepository, TokioBus>>,
}

impl Assembler {
    /// # Errors
    ///
    /// This function will return an error in the following situations:
    ///
    /// * If the creation of `IbtExtractor` fails, it will return an error string propagated from that operation.
    ///
    /// The failure could be due to various reasons such as configuration issues, resource allocation failures,
    /// or other runtime errors specific to the initialization process of `IbtExtractor`.
    pub fn new(
        event_bus: &Arc<TokioBus>,
        file_creator: &Arc<FileCreator<FileRepository, TokioBus>>,
        lap_creator: &Arc<LapCreator<LapRepository>>,
    ) -> Result<Self, String> {
        let parser = Arc::new(IbtExtractor::new(
            Arc::clone(file_creator),
            Arc::clone(lap_creator),
            Arc::clone(event_bus),
        ));
        Ok(Self { parser })
    }
}
