pub mod camera_info;
pub mod car_setup;
pub mod driver_info;
pub mod radio_info;
pub mod sessions;
pub mod split_time_info;
pub mod weekend_info;

use crate::ibt::domain::file::session_info::camera_info::CameraInfo;
use crate::ibt::domain::file::session_info::car_setup::CarSetup;
use crate::ibt::domain::file::session_info::driver_info::DriverInfo;
use crate::ibt::domain::file::session_info::radio_info::RadioInfo;
use crate::ibt::domain::file::session_info::sessions::Sessions;
use crate::ibt::domain::file::session_info::split_time_info::SplitTimeInfo;
use crate::ibt::domain::file::session_info::weekend_info::WeekendInfo;

use serde::Deserialize;
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SessionInfo {
    pub weekend_info: Option<WeekendInfo>,
    pub session_info: Option<Sessions>,
    pub camera_info: Option<CameraInfo>,
    pub radio_info: Option<RadioInfo>,
    pub driver_info: Option<DriverInfo>,
    pub split_time_info: Option<SplitTimeInfo>,
    pub car_setup: Option<CarSetup>,
}

impl SessionInfo {
    pub fn from_stream(
        reader: &mut (impl Read + Seek),
        offset: u64,
        length: usize,
    ) -> Result<SessionInfo, Error> {
        reader
            .seek(SeekFrom::Start(offset))
            .map_err(|e| Error::FromStream(format!("{e}")))?;

        let mut session_info_buffer = vec![0u8; length];

        reader
            .read_exact(&mut session_info_buffer)
            .map_err(|e| Error::FromStream(format!("{e}")))?;

        // Fix session info to String replacing invalid utf-8 characters with �
        let session_info_str = String::from_utf8_lossy(&session_info_buffer);

        // Parse str with yaml
        serde_yaml::from_str(session_info_str.as_ref())
            .map_err(|e| Error::ParseYaml(format!("{e}")))
    }
}

/// Errors that can be returned from [`DiskHeader::try_from`].
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error trying to extract Session Info from Stream: {0}")]
    FromStream(String),
    #[error("Error trying to parse Session Info to yaml: {0}")]
    ParseYaml(String),
}
