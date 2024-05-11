use serde::Deserialize;

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LeftTire {
    pub starting_pressure: Option<String>,
    pub last_hot_pressure: Option<String>,
    #[serde(rename = "LastTempsOMI")]
    pub last_temps_omi: Option<String>,
    pub tread_remaining: Option<String>,
}
