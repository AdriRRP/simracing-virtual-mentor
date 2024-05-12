use serde::Deserialize;

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Sector {
    pub sector_num: Option<i32>,
    pub sector_start_pct: Option<f32>,
}
