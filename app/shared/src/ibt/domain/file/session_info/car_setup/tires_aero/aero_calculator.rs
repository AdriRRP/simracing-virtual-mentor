use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AeroCalculator {
    pub front_rh_at_speed: Option<String>,
    pub rear_rh_at_speed: Option<String>,
    pub downforce_balance: Option<String>,
    #[serde(rename = "LD")]
    pub ld: Option<f32>,
}
