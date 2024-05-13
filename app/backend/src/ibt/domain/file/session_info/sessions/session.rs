pub mod fastest_lap;

use crate::ibt::domain::file::session_info::sessions::session::fastest_lap::FastestLap;

use serde::Deserialize;

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Session {
    pub session_num: Option<i32>,
    pub session_laps: Option<String>,
    pub session_time: Option<String>,
    pub session_num_laps_to_avg: Option<i32>,
    pub session_type: Option<String>,
    pub session_track_rubber_state: Option<String>,
    pub session_name: Option<String>,
    pub session_sub_type: Option<String>,
    pub session_skipped: Option<i32>,
    pub session_run_groups_used: Option<i32>,
    pub session_enforce_tire_compound_change: Option<i32>,
    pub results_fastest_lap: Vec<FastestLap>,
    pub results_average_lap_time: Option<f32>,
    pub results_num_caution_flags: Option<i32>,
    pub results_num_caution_laps: Option<i32>,
    pub results_num_lead_changes: Option<i32>,
    pub results_laps_complete: Option<i32>,
    pub results_official: Option<i32>,
}
