use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AeroSettings {
    pub rear_wing_angle: Option<String>,
}
