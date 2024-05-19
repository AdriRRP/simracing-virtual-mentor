use crate::file::application::create::service::Creator as FileCreator;
use crate::file::application::delete::service::Deleter as FileDeleter;
use crate::file::application::find::by_criteria::service::Finder as FileByCriteriaFinder;
use crate::file::application::find::by_id::service::Finder as FileByIdFinder;
use crate::file::infrastructure::repository::in_memory::InMemory as InMemoryFileRepository;
use crate::shared::domain::event::bus::Bus as EventBus;

use std::sync::Arc;

pub struct FileAssembler {
    pub creator: Arc<FileCreator<InMemoryFileRepository>>,
    pub deleter: Arc<FileDeleter<InMemoryFileRepository>>,
    pub by_id_finder: Arc<FileByIdFinder<InMemoryFileRepository>>,
    pub by_criteria_finder: Arc<FileByCriteriaFinder<InMemoryFileRepository>>,
}

impl FileAssembler {
    pub fn new(event_bus: Arc<dyn EventBus>) -> Self {
        let repository = Arc::new(InMemoryFileRepository::default());
        let creator = Arc::new(FileCreator::new(
            Arc::clone(&repository),
            Arc::clone(&event_bus),
        ));
        let deleter = Arc::new(FileDeleter::new(
            Arc::clone(&repository),
            Arc::clone(&event_bus),
        ));
        let by_id_finder = Arc::new(FileByIdFinder::new(Arc::clone(&repository)));
        let by_criteria_finder = Arc::new(FileByCriteriaFinder::new(Arc::clone(&repository)));

        Self {
            creator,
            deleter,
            by_id_finder,
            by_criteria_finder,
        }
    }
}
