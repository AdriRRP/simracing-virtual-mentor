pub mod brake_spec;
pub mod fuel;
pub mod gear_ratios;
pub mod hybrid_config;
pub mod lighting;
pub mod rear_diff_spec;
pub mod traction_control;

use crate::ibt::domain::file::session_info::car_setup::brakes_drive_unit::brake_spec::BrakeSpec;
use crate::ibt::domain::file::session_info::car_setup::brakes_drive_unit::fuel::Fuel;
use crate::ibt::domain::file::session_info::car_setup::brakes_drive_unit::gear_ratios::GearRatios;
use crate::ibt::domain::file::session_info::car_setup::brakes_drive_unit::hybrid_config::HybridConfig;
use crate::ibt::domain::file::session_info::car_setup::brakes_drive_unit::lighting::Lighting;
use crate::ibt::domain::file::session_info::car_setup::brakes_drive_unit::rear_diff_spec::RearDiffSpec;
use crate::ibt::domain::file::session_info::car_setup::brakes_drive_unit::traction_control::TractionControl;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BrakesDriveUnit {
    pub lighting: Option<Lighting>,
    pub brake_spec: Option<BrakeSpec>,
    pub hybrid_config: Option<HybridConfig>,
    pub fuel: Option<Fuel>,
    pub traction_control: Option<TractionControl>,
    pub gear_ratios: Option<GearRatios>,
    pub rear_diff_spec: Option<RearDiffSpec>,
}
