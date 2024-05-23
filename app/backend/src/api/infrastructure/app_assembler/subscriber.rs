use crate::api::infrastructure::subscriber::on_ibt_extracted::FileUpdater;
use crate::file::domain::event::created::Created;
use crate::lap::infrastructure::repository::in_memory::InMemory as InMemoryLapRepository;
use crate::shared::domain::event::Event;
use crate::shared::infrastructure::event::tokio_bus::TokioBus;

use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Assembler {
    pub file_updater: Arc<Mutex<FileUpdater>>,
}

impl Assembler {
    #[must_use]
    pub async fn new(event_bus: &Arc<TokioBus>) -> Self {
        let _repository = Arc::new(InMemoryLapRepository::default());
        let file_updater = Arc::new(Mutex::new(FileUpdater::new(
            event_bus.receiver(Created::id()).await,
        )));
        Self { file_updater }
    }
}
