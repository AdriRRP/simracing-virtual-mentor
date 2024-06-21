pub mod brakes_drive_unit;
pub mod chassis;
pub mod tires_aero;

use crate::ibt::domain::file::session_info::car_setup::brakes_drive_unit::BrakesDriveUnit;
use crate::ibt::domain::file::session_info::car_setup::chassis::Chassis;
use crate::ibt::domain::file::session_info::car_setup::tires_aero::TiresAero;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CarSetup {
    pub update_count: Option<i32>,
    pub tires_aero: Option<TiresAero>,
    pub chassis: Option<Chassis>,
    pub brakes_drive_unit: Option<BrakesDriveUnit>,
}
