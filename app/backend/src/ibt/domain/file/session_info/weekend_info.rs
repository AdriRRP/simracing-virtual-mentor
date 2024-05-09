pub mod telemetry_options;
pub mod weekend_options;

use serde::Deserialize;
use telemetry_options::TelemetryOptions;
use weekend_options::WeekendOptions;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WeekendInfo {
    pub track_name: Option<String>,
    #[serde(rename = "TrackID")]
    pub track_id: Option<String>,
    pub track_length: Option<String>,
    pub track_length_official: Option<String>,
    pub track_display_name: Option<String>,
    pub track_display_short_name: Option<String>,
    pub track_config_name: Option<String>,
    pub track_city: Option<String>,
    pub track_country: Option<String>,
    pub track_altitude: Option<String>,
    pub track_latitude: Option<String>,
    pub track_longitude: Option<String>,
    pub track_north_offset: Option<String>,
    pub track_num_turns: Option<i32>,
    pub track_pit_speed_limit: Option<String>,
    pub track_type: Option<String>,
    pub track_direction: Option<String>,
    pub track_weather_type: Option<String>,
    pub track_skies: Option<String>,
    pub track_surface_temp: Option<String>,
    pub track_air_temp: Option<String>,
    pub track_air_pressure: Option<String>,
    pub track_wind_vel: Option<String>,
    pub track_wind_dir: Option<String>,
    pub track_relative_humidity: Option<String>,
    pub track_fog_level: Option<String>,
    pub track_cleanup: Option<i32>,
    pub track_dynamic_track: Option<i32>,
    pub track_version: Option<String>,
    #[serde(rename = "SeriesID")]
    pub series_id: Option<i32>,
    #[serde(rename = "SeasonID")]
    pub season_id: Option<i32>,
    #[serde(rename = "SessionID")]
    pub session_id: Option<i32>,
    #[serde(rename = "SubSessionID")]
    pub sub_session_id: Option<i32>,
    #[serde(rename = "LeagueID")]
    pub league_id: Option<i32>,
    pub official: Option<i32>,
    pub race_week: Option<i32>,
    pub event_type: Option<String>,
    pub category: Option<String>,
    pub sim_mode: Option<String>,
    pub team_racing: Option<i32>,
    pub min_drivers: Option<i32>,
    pub max_drivers: Option<i32>,
    #[serde(rename = "DCRuleSet")]
    pub dc_rule_set: Option<String>,
    pub qualifier_must_start_race: Option<i32>,
    pub num_car_classes: Option<i32>,
    pub num_car_types: Option<i32>,
    pub heat_racing: Option<i32>,
    pub build_type: Option<String>,
    pub build_target: Option<String>,
    pub build_version: Option<String>,
    pub weekend_options: Option<WeekendOptions>,
    pub telemetry_options: Option<TelemetryOptions>,
}
