use serde::Deserialize;

#[derive(PartialEq, Eq, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BrakeSpec {
    pub pad_compound: Option<String>,
    pub front_master_cyl: Option<String>,
    pub rear_master_cyl: Option<String>,
    pub brake_pressure_bias: Option<String>,
}
