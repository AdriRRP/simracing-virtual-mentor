use serde::Deserialize;

#[derive(PartialEq, Eq, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Fuel {
    pub fuel_level: Option<String>,
}
