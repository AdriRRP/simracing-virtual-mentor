use serde::Deserialize;

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AeroSettings {
    pub rear_wing_angle: Option<String>,
}
