use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LeftRear {
    pub corner_weight: Option<String>,
    pub ride_height: Option<String>,
    pub shock_defl: Option<String>,
    pub spring_defl: Option<String>,
    pub spring_perch_offset: Option<String>,
    pub spring_rate: Option<String>,
    pub ls_comp_damping: Option<String>,
    pub hs_comp_damping: Option<String>,
    pub hs_comp_damp_slope: Option<String>,
    pub ls_rbd_damping: Option<String>,
    pub hs_rbd_damping: Option<String>,
    pub camber: Option<String>,
    pub toe_in: Option<String>,
    pub bump_stiffness: Option<String>,
    pub rebound_stiffness: Option<String>,
}
