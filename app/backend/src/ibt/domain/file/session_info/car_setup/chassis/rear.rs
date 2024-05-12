use serde::Deserialize;

#[derive(PartialEq, Eq, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Rear {
    pub third_spring: Option<String>,
    pub third_perch_offset: Option<String>,
    pub third_spring_defl: Option<String>,
    pub third_slider_defl: Option<String>,
    pub arb_size: Option<String>,
    pub arb_blades: Option<String>,
    pub pushrod_length_offset: Option<String>,
    pub cross_weight: Option<String>,
    pub fuel_level: Option<String>,
    pub arb_setting: Option<i32>,
    pub wing_setting: Option<i32>,
    pub diff_clutches: Option<i32>,
}
