use serde::Deserialize;

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TractionControl {
    pub traction_control_gain: Option<String>,
    pub traction_control_slip: Option<String>,
}
