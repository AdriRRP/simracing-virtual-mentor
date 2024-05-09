use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Fuel {
    pub fuel_level: Option<String>,
}
