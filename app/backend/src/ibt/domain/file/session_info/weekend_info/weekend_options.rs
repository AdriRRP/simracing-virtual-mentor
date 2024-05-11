use serde::Deserialize;

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct WeekendOptions {
    pub num_starters: Option<i32>,
    pub starting_grid: Option<String>,
    pub qualify_scoring: Option<String>,
    pub course_cautions: Option<String>,
    pub standing_start: Option<i32>,
    pub short_parade_lap: Option<i32>,
    pub restarts: Option<String>,
    pub weather_type: Option<String>,
    pub skies: Option<String>,
    pub wind_direction: Option<String>,
    pub wind_speed: Option<String>,
    pub weather_temp: Option<String>,
    pub relative_humidity: Option<String>,
    pub fog_level: Option<String>,
    pub time_of_day: Option<String>,
    pub date: Option<String>,
    pub earth_rotation_speedup_factor: Option<i32>,
    pub unofficial: Option<i32>,
    pub commercial_mode: Option<String>,
    pub night_mode: Option<String>,
    pub is_fixed_setup: Option<i32>,
    pub strict_laps_checking: Option<String>,
    pub has_open_registration: Option<i32>,
    pub hardcore_level: Option<i32>,
    pub num_joker_laps: Option<i32>,
    pub incident_limit: Option<String>,
    pub fast_repairs_limit: Option<String>,
    pub green_white_checkered_limit: Option<i32>,
}
