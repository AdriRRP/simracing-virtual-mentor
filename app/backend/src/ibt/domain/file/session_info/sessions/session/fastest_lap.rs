use serde::Deserialize;

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FastestLap {
    pub car_idx: Option<String>,
    pub fastest_lap: Option<i32>,
    pub fastest_time: Option<f32>,
}
