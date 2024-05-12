use serde::Deserialize;

#[derive(PartialEq, Eq, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct GearRatios {
    pub gear_stack: Option<String>,
    pub speed_in_first: Option<String>,
    pub speed_in_second: Option<String>,
    pub speed_in_third: Option<String>,
    pub speed_in_fourth: Option<String>,
    pub speed_in_fifth: Option<String>,
    pub speed_in_sixth: Option<String>,
    pub speed_in_seventh: Option<String>,
}
