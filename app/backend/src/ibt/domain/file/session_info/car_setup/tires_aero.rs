pub mod aero_calculator;
pub mod aero_settings;
pub mod left_tire;
pub mod right_tire;

use crate::ibt::domain::file::session_info::car_setup::tires_aero::aero_calculator::AeroCalculator;
use crate::ibt::domain::file::session_info::car_setup::tires_aero::aero_settings::AeroSettings;
use crate::ibt::domain::file::session_info::car_setup::tires_aero::left_tire::LeftTire;
use crate::ibt::domain::file::session_info::car_setup::tires_aero::right_tire::RightTire;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TiresAero {
    pub left_front_tire: Option<LeftTire>,
    pub left_rear_tire: Option<LeftTire>,
    pub right_front_tire: Option<RightTire>,
    pub right_rear_tire: Option<RightTire>,
    pub aero_settings: Option<AeroSettings>,
    pub aero_calculator: Option<AeroCalculator>,
}
