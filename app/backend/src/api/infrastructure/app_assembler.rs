pub mod analysis;
pub mod file;
pub mod ibt;
pub mod lap;

use crate::api::infrastructure::app_assembler::analysis::Assembler as AnalysisAssembler;
use crate::api::infrastructure::app_assembler::file::Assembler as FileAssembler;
use crate::api::infrastructure::app_assembler::ibt::Assembler as IbtAssembler;
use crate::api::infrastructure::app_assembler::lap::Assembler as LapAssembler;
use crate::api::infrastructure::event::tokio_bus::TokioBus as TokioEventBus;
use crate::api::infrastructure::settings::Settings;

use std::sync::Arc;

pub struct AppAssembler {
    pub event_bus: Arc<TokioEventBus>,
    pub analysis: AnalysisAssembler,
    pub file: FileAssembler,
    pub lap: LapAssembler,
    pub ibt: IbtAssembler,
}

impl AppAssembler {
    /// # Errors
    ///
    /// This function will return an error in the following situations:
    ///
    /// * If `FileAssembler::new` fails, it will return an error string propagated from that function.
    /// * If `LapAssembler::new` fails, it will return an error string propagated from that function.
    /// * If `AnalysisAssembler::new` fails, it will return an error string propagated from that function.
    /// * If `IbtAssembler::new` fails, it will return an error string propagated from that function.
    ///
    /// Each of these functions (`FileAssembler::new`, `LapAssembler::new`, `AnalysisAssembler::new`, 
    /// and `IbtAssembler::new`) could fail due to various reasons such as configuration issues, 
    /// resource allocation failures, or other runtime errors specific to the initialization process
    /// of each component.
    pub async fn new(settings: &Settings) -> Result<Self, String> {
        let event_bus = Arc::new(TokioEventBus::new(settings.event_bus.capacity));

        let file = FileAssembler::new(settings, &event_bus).await?;
        let lap = LapAssembler::new(settings, &event_bus).await?;
        let analysis = AnalysisAssembler::new(settings, &event_bus, &lap.repository).await?;
        let ibt = IbtAssembler::new(&event_bus, &file.creator, &lap.creator)?;

        Ok(Self {
            event_bus,
            analysis,
            file,
            lap,
            ibt,
        })
    }
}
