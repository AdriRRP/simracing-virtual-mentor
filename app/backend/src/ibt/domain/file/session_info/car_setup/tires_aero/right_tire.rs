use serde::Deserialize;

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RightTire {
    pub starting_pressure: Option<String>,
    pub last_hot_pressure: Option<String>,
    #[serde(rename = "LastTempsIMO")]
    pub last_temps_imo: Option<String>,
    pub tread_remaining: Option<String>,
}
