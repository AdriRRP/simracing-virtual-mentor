use crate::api::infrastructure::event::tokio_bus::TokioBus;

use shared::analysis::application::analyze::service::Analyzer;
use shared::analysis::application::find::by_id::service::Finder;
use shared::analysis::application::update::service::Updater;
use shared::analysis::domain::event::created::Created;
use shared::analysis::infrastructure::repository::in_memory::InMemory as InMemoryAnalysisRepository;
use shared::common::domain::event::subscriber::{Error, Subscriber};
use shared::common::domain::event::Event;
use shared::lap::infrastructure::repository::in_memory::InMemory as InMemoryLapRepository;

use async_trait::async_trait;
use shared::analysis::domain::analysis::Analysis;
use std::sync::Arc;
use tokio::sync::broadcast::Receiver;
use tokio::sync::RwLock;

type EventReceiver = Receiver<Arc<dyn Event>>;

pub struct DoAnalysis {
    receiver: Arc<RwLock<EventReceiver>>,
    finder: Arc<Finder<InMemoryAnalysisRepository>>,
    analyzer: Arc<Analyzer<InMemoryAnalysisRepository, InMemoryLapRepository>>,
    updater: Arc<Updater<InMemoryAnalysisRepository>>,
}

impl DoAnalysis {
    #[must_use]
    pub async fn new(
        event_bus: &Arc<TokioBus>,
        finder: &Arc<Finder<InMemoryAnalysisRepository>>,
        analyzer: &Arc<Analyzer<InMemoryAnalysisRepository, InMemoryLapRepository>>,
        updater: &Arc<Updater<InMemoryAnalysisRepository>>,
    ) -> Self {
        tracing::debug!("Creating subscriber");

        let receiver = event_bus.receiver(Created::event_id()).await;
        let receiver = Arc::new(RwLock::new(receiver));
        let finder = Arc::clone(finder);
        let analyzer = Arc::clone(analyzer);
        let updater = Arc::clone(updater);

        Self {
            receiver,
            finder,
            analyzer,
            updater,
        }
    }
}

#[async_trait]
impl Subscriber for DoAnalysis {
    async fn receive(&self) -> Result<Arc<dyn Event>, Error> {
        tracing::trace!("Waiting for new analyses");
        let mut receiver = self.receiver.write().await;
        receiver
            .recv()
            .await
            .map_err(|e| Error::Receive(format!("{e}")))
    }

    async fn process(&self, event: Arc<dyn Event>) {
        tracing::debug!("Processing new file {}", event.id());
        if let Some(analysis_created) = event.as_any().downcast_ref::<Created>() {
            if let Err(msg) = self.analyzer.analyze(analysis_created.id).await {
                if let Ok(Some(analysis)) = self.finder.find(&analysis_created.id).await {
                    let error_analysis = Analysis::with_error(
                        analysis.header.id,
                        analysis.header.name,
                        analysis.header.date,
                        analysis.header.circuit,
                        msg,
                    );

                    if let Err(msg) = self.updater.updater(&error_analysis).await {
                        tracing::error!(
                            "IMPORTANT: Can't update analysis `{}` to set erroneous status {}",
                            analysis_created.id,
                            msg,
                        );
                    };
                }
            } else {
                tracing::info!("Analysis with id `{}` done!", analysis_created.id);
            }
        } else {
            tracing::error!("Can't downcast {} to {}", event.id(), Created::event_id());
        }
    }
}
