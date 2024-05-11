use serde::Deserialize;

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct InCarDials {
    pub dash_display_page: Option<String>,
    pub brake_pressure_bias: Option<String>,
    pub brake_pads: Option<String>,
    pub abs_setting: Option<String>,
    pub tc_setting: Option<String>,
}
