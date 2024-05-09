use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Lighting {
    pub roof_id_light_color: Option<String>,
}
