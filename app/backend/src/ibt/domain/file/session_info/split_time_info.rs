pub mod sector;

use crate::ibt::domain::file::session_info::split_time_info::sector::Sector;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SplitTimeInfo {
    pub sectors: Option<Vec<Sector>>,
}
