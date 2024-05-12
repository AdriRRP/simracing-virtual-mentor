use serde::Deserialize;

#[derive(PartialEq, Eq, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Lighting {
    pub roof_id_light_color: Option<String>,
}
