pub mod driver;

use crate::ibt::domain::file::session_info::driver_info::driver::Driver;

use serde::Deserialize;

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DriverInfo {
    pub driver_car_idx: Option<i32>,
    #[serde(rename = "DriverUserID")]
    pub driver_user_id: Option<String>,
    pub pace_car_idx: Option<String>,
    pub driver_head_pos_x: Option<f32>,
    pub driver_head_pos_y: Option<f32>,
    pub driver_head_pos_z: Option<f32>,
    pub driver_car_is_electric: Option<i32>,
    #[serde(rename = "DriverCarIdleRPM")]
    pub driver_car_idle_rpm: Option<f32>,
    pub driver_car_red_line: Option<f32>,
    pub driver_car_eng_cylinder_count: Option<i32>,
    pub driver_car_fuel_kg_per_ltr: Option<f32>,
    pub driver_car_fuel_max_ltr: Option<f32>,
    pub driver_car_max_fuel_pct: Option<f32>,
    pub driver_car_gear_num_forward: Option<i32>,
    pub driver_car_gear_neutral: Option<i32>,
    pub driver_car_gear_reverse: Option<i32>,
    #[serde(rename = "DriverCarSLFirstRPM")]
    pub driver_car_sl_first_rpm: Option<f32>,
    #[serde(rename = "DriverCarSLShiftRPM")]
    pub driver_car_sl_shift_rpm: Option<f32>,
    #[serde(rename = "DriverCarSLLastRPM")]
    pub driver_car_sl_last_rpm: Option<f32>,
    #[serde(rename = "DriverCarSLBlinkRPM")]
    pub driver_car_sl_blink_rpm: Option<f32>,
    pub driver_car_version: Option<String>,
    pub driver_pit_trk_pct: Option<f32>,
    pub driver_car_est_lap_time: Option<f32>,
    pub driver_setup_name: Option<String>,
    pub driver_setup_is_modified: Option<f32>,
    pub driver_setup_load_type_name: Option<String>,
    pub driver_setup_passed_tech: Option<i32>,
    pub driver_incident_count: Option<i32>,
    pub drivers: Option<Vec<Driver>>,
}
