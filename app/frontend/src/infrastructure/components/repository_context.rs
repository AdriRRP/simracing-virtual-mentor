use crate::infrastructure::repository::analysis::http::Http as AnalysisRepository;
use crate::infrastructure::repository::file::http::Http as FileRepository;
use crate::infrastructure::repository::ibt::http::Http as IbtRepository;
use crate::infrastructure::repository::lap::http::Http as LapRepository;
use crate::infrastructure::settings::Settings;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Repositories {
    pub analysis: AnalysisRepository,
    pub file: FileRepository,
    pub ibt: IbtRepository,
    pub lap: LapRepository,
}

impl Default for Repositories {
    fn default() -> Self {
        let settings = Settings::default();
        Self {
            analysis: AnalysisRepository::new(&settings),
            file: FileRepository::new(&settings),
            ibt: IbtRepository::new(&settings),
            lap: LapRepository::new(&settings),
        }
    }
}

impl Repositories {
    #[must_use]
    pub fn new(settings: &Settings) -> Self {
        Self {
            analysis: AnalysisRepository::new(settings),
            file: FileRepository::new(settings),
            ibt: IbtRepository::new(settings),
            lap: LapRepository::new(settings),
        }
    }
}
