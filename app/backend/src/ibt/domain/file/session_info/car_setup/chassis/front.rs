use serde::Deserialize;

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Front {
    pub heave_spring: Option<String>,
    pub heave_perch_offset: Option<String>,
    pub heave_spring_defl: Option<String>,
    pub heave_slider_defl: Option<String>,
    pub arb_size: Option<String>,
    pub arb_blades: Option<i32>,
    pub arb_setting: Option<i32>,
    pub toe_in: Option<String>,
    pub pushrod_length_offset: Option<String>,
    pub cross_weight: Option<String>,
    pub nose_weight: Option<String>,
}
