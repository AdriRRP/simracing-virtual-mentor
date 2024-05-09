use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Driver {
    pub car_idx: Option<i32>,
    pub user_name: Option<String>,
    pub abbrev_name: Option<String>,
    pub initials: Option<String>,
    #[serde(rename = "UserID")]
    pub user_id: Option<i32>,
    #[serde(rename = "TeamID")]
    pub team_id: Option<i32>,
    pub team_name: Option<String>,
    pub car_number: Option<String>,
    pub car_number_raw: Option<i32>,
    pub car_path: Option<String>,
    #[serde(rename = "CarClassID")]
    pub car_class_id: Option<i32>,
    #[serde(rename = "CarID")]
    pub car_id: Option<i32>,
    pub car_is_pace_car: Option<i32>,
    #[serde(rename = "CarIsAI")]
    pub car_is_ai: Option<i32>,
    pub car_is_electric: Option<i32>,
    pub car_screen_name: Option<String>,
    pub car_screen_name_short: Option<String>,
    pub car_class_short_name: Option<String>,
    pub car_class_rel_speed: Option<i32>,
    pub car_class_license_level: Option<i32>,
    pub car_class_max_fuel_pct: Option<String>,
    pub car_class_weight_penalty: Option<String>,
    pub car_class_power_adjust: Option<String>,
    pub car_class_dry_tire_set_limit: Option<String>,
    pub car_class_color: Option<i32>,
    pub car_class_est_lap_time: Option<f32>,
    #[serde(rename = "IRating")]
    pub i_rating: Option<i32>,
    pub lic_level: Option<i32>,
    pub lic_sub_level: Option<i32>,
    pub lic_string: Option<String>,
    pub lic_color: Option<String>,
    pub is_spectator: Option<i32>,
    pub car_design_str: Option<String>,
    pub helmet_design_str: Option<String>,
    pub suit_design_str: Option<String>,
    pub body_type: Option<i32>,
    pub face_type: Option<i32>,
    pub helmet_type: Option<i32>,
    pub car_number_design_str: Option<String>,
    #[serde(rename = "CarSponsor_1")]
    pub car_sponsor_1: Option<i32>,
    #[serde(rename = "CarSponsor_2")]
    pub car_sponsor_2: Option<i32>,
    pub cur_driver_incident_count: Option<i32>,
    pub team_incident_count: Option<i32>,
}
