use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Fuel {
    pub fuel_level: Option<String>,
}
