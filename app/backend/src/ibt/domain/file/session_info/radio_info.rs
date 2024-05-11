pub mod radio;

use crate::ibt::domain::file::session_info::radio_info::radio::Radio;
use serde::Deserialize;

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RadioInfo {
    pub selected_radio_num: Option<i32>,
    pub radios: Option<Vec<Radio>>,
}
