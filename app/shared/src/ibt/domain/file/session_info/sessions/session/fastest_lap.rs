use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FastestLap {
    pub car_idx: Option<String>,
    pub fastest_lap: Option<i32>,
    pub fastest_time: Option<f32>,
}
