#[allow(clippy::module_name_repetitions)]
pub mod radio_frequency;

use crate::ibt::domain::file::session_info::radio_info::radio::radio_frequency::RadioFrequency;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Radio {
    pub radio_num: Option<i32>,
    pub hop_count: Option<i32>,
    pub num_frequencies: Option<i32>,
    pub tuned_to_frequency_num: Option<i32>,
    pub scanning_is_on: Option<i32>,
    pub frequencies: Option<Vec<RadioFrequency>>,
}
