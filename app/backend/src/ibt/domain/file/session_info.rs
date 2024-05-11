pub mod camera_info;
pub mod car_setup;
pub mod driver_info;
pub mod radio_info;
pub mod sessions;
pub mod split_time_info;
pub mod weekend_info;

use crate::ibt::domain::file::from_reader::FromReaderVarSize;
use crate::ibt::domain::file::session_info::camera_info::CameraInfo;
use crate::ibt::domain::file::session_info::car_setup::CarSetup;
use crate::ibt::domain::file::session_info::driver_info::DriverInfo;
use crate::ibt::domain::file::session_info::radio_info::RadioInfo;
use crate::ibt::domain::file::session_info::sessions::Sessions;
use crate::ibt::domain::file::session_info::split_time_info::SplitTimeInfo;
use crate::ibt::domain::file::session_info::weekend_info::WeekendInfo;

use serde::Deserialize;
use std::io::{Read, Seek};

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SessionInfo {
    pub weekend_info: Option<WeekendInfo>,
    pub session_info: Option<Sessions>,
    pub camera_info: Option<CameraInfo>,
    pub radio_info: Option<RadioInfo>,
    pub driver_info: Option<DriverInfo>,
    pub split_time_info: Option<SplitTimeInfo>,
    pub car_setup: Option<CarSetup>,
}

impl TryFrom<&Vec<u8>> for SessionInfo {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self, Self::Error> {
        // Fix session info to String replacing invalid utf-8 characters with �
        let session_info_str = String::from_utf8_lossy(value);

        // Parse str with yaml
        serde_yaml::from_str(session_info_str.as_ref())
            .map_err(|e| Error::ParseYaml(format!("{e}")))
    }
}

impl<ReadSeek> FromReaderVarSize<ReadSeek, Error> for SessionInfo where ReadSeek: Read + Seek {}

/// Errors that can be returned from [`DiskHeader::try_from`].
#[derive(PartialEq, Debug, thiserror::Error)]
pub enum Error {
    #[error("Error trying to extract Session Info from Stream: {0}")]
    FromStream(String),
    #[error("Error trying to parse Session Info to yaml: {0}")]
    ParseYaml(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_bytes() -> Vec<u8> {
        vec![
            45, 45, 45, 10, 87, 101, 101, 107, 101, 110, 100, 73, 110, 102, 111, 58, 10, 32, 84,
            114, 97, 99, 107, 78, 97, 109, 101, 58, 32, 100, 97, 121, 116, 111, 110, 97, 32, 50,
            48, 49, 49, 32, 114, 111, 97, 100, 10, 32, 84, 114, 97, 99, 107, 73, 68, 58, 32, 49,
            57, 50, 10, 32, 84, 114, 97, 99, 107, 76, 101, 110, 103, 116, 104, 58, 32, 53, 46, 54,
            57, 32, 107, 109, 10, 32, 84, 114, 97, 99, 107, 76, 101, 110, 103, 116, 104, 79, 102,
            102, 105, 99, 105, 97, 108, 58, 32, 53, 46, 55, 51, 32, 107, 109, 10, 32, 84, 114, 97,
            99, 107, 68, 105, 115, 112, 108, 97, 121, 78, 97, 109, 101, 58, 32, 68, 97, 121, 116,
            111, 110, 97, 32, 73, 110, 116, 101, 114, 110, 97, 116, 105, 111, 110, 97, 108, 32, 83,
            112, 101, 101, 100, 119, 97, 121, 10, 32, 84, 114, 97, 99, 107, 68, 105, 115, 112, 108,
            97, 121, 83, 104, 111, 114, 116, 78, 97, 109, 101, 58, 32, 68, 97, 121, 116, 111, 110,
            97, 10, 32, 84, 114, 97, 99, 107, 67, 111, 110, 102, 105, 103, 78, 97, 109, 101, 58,
            32, 82, 111, 97, 100, 32, 67, 111, 117, 114, 115, 101, 10, 32, 84, 114, 97, 99, 107,
            67, 105, 116, 121, 58, 32, 68, 97, 121, 116, 111, 110, 97, 32, 66, 101, 97, 99, 104,
            10, 32, 84, 114, 97, 99, 107, 67, 111, 117, 110, 116, 114, 121, 58, 32, 85, 83, 65, 10,
            32, 84, 114, 97, 99, 107, 65, 108, 116, 105, 116, 117, 100, 101, 58, 32, 49, 49, 46,
            49, 51, 32, 109, 10, 32, 84, 114, 97, 99, 107, 76, 97, 116, 105, 116, 117, 100, 101,
            58, 32, 50, 57, 46, 49, 56, 55, 55, 49, 50, 32, 109, 10, 32, 84, 114, 97, 99, 107, 76,
            111, 110, 103, 105, 116, 117, 100, 101, 58, 32, 45, 56, 49, 46, 48, 55, 50, 55, 57, 56,
            32, 109, 10, 32, 84, 114, 97, 99, 107, 78, 111, 114, 116, 104, 79, 102, 102, 115, 101,
            116, 58, 32, 51, 46, 55, 55, 52, 50, 32, 114, 97, 100, 10, 32, 84, 114, 97, 99, 107,
            78, 117, 109, 84, 117, 114, 110, 115, 58, 32, 49, 49, 10, 32, 84, 114, 97, 99, 107, 80,
            105, 116, 83, 112, 101, 101, 100, 76, 105, 109, 105, 116, 58, 32, 56, 55, 46, 57, 56,
            32, 107, 112, 104, 10, 32, 84, 114, 97, 99, 107, 84, 121, 112, 101, 58, 32, 114, 111,
            97, 100, 32, 99, 111, 117, 114, 115, 101, 10, 32, 84, 114, 97, 99, 107, 68, 105, 114,
            101, 99, 116, 105, 111, 110, 58, 32, 110, 101, 117, 116, 114, 97, 108, 10, 32, 84, 114,
            97, 99, 107, 87, 101, 97, 116, 104, 101, 114, 84, 121, 112, 101, 58, 32, 83, 112, 101,
            99, 105, 102, 105, 101, 100, 32, 47, 32, 83, 116, 97, 116, 105, 99, 32, 83, 107, 121,
            10, 32, 84, 114, 97, 99, 107, 83, 107, 105, 101, 115, 58, 32, 77, 111, 115, 116, 108,
            121, 32, 67, 108, 111, 117, 100, 121, 10, 32, 84, 114, 97, 99, 107, 83, 117, 114, 102,
            97, 99, 101, 84, 101, 109, 112, 58, 32, 50, 54, 46, 48, 48, 32, 67, 10, 32, 84, 114,
            97, 99, 107, 65, 105, 114, 84, 101, 109, 112, 58, 32, 50, 48, 46, 48, 48, 32, 67, 10,
            32, 84, 114, 97, 99, 107, 65, 105, 114, 80, 114, 101, 115, 115, 117, 114, 101, 58, 32,
            50, 57, 46, 56, 56, 32, 72, 103, 10, 32, 84, 114, 97, 99, 107, 87, 105, 110, 100, 86,
            101, 108, 58, 32, 49, 46, 55, 57, 32, 109, 47, 115, 10, 32, 84, 114, 97, 99, 107, 87,
            105, 110, 100, 68, 105, 114, 58, 32, 50, 46, 51, 54, 32, 114, 97, 100, 10, 32, 84, 114,
            97, 99, 107, 82, 101, 108, 97, 116, 105, 118, 101, 72, 117, 109, 105, 100, 105, 116,
            121, 58, 32, 54, 51, 32, 37, 10, 32, 84, 114, 97, 99, 107, 70, 111, 103, 76, 101, 118,
            101, 108, 58, 32, 48, 32, 37, 10, 32, 84, 114, 97, 99, 107, 67, 108, 101, 97, 110, 117,
            112, 58, 32, 49, 10, 32, 84, 114, 97, 99, 107, 68, 121, 110, 97, 109, 105, 99, 84, 114,
            97, 99, 107, 58, 32, 49, 10, 32, 84, 114, 97, 99, 107, 86, 101, 114, 115, 105, 111,
            110, 58, 32, 50, 48, 50, 51, 46, 48, 56, 46, 50, 53, 46, 48, 50, 10, 32, 83, 101, 114,
            105, 101, 115, 73, 68, 58, 32, 48, 10, 32, 83, 101, 97, 115, 111, 110, 73, 68, 58, 32,
            48, 10, 32, 83, 101, 115, 115, 105, 111, 110, 73, 68, 58, 32, 48, 10, 32, 83, 117, 98,
            83, 101, 115, 115, 105, 111, 110, 73, 68, 58, 32, 48, 10, 32, 76, 101, 97, 103, 117,
            101, 73, 68, 58, 32, 48, 10, 32, 79, 102, 102, 105, 99, 105, 97, 108, 58, 32, 48, 10,
            32, 82, 97, 99, 101, 87, 101, 101, 107, 58, 32, 48, 10, 32, 69, 118, 101, 110, 116, 84,
            121, 112, 101, 58, 32, 84, 101, 115, 116, 10, 32, 67, 97, 116, 101, 103, 111, 114, 121,
            58, 32, 82, 111, 97, 100, 10, 32, 83, 105, 109, 77, 111, 100, 101, 58, 32, 102, 117,
            108, 108, 10, 32, 84, 101, 97, 109, 82, 97, 99, 105, 110, 103, 58, 32, 48, 10, 32, 77,
            105, 110, 68, 114, 105, 118, 101, 114, 115, 58, 32, 48, 10, 32, 77, 97, 120, 68, 114,
            105, 118, 101, 114, 115, 58, 32, 48, 10, 32, 68, 67, 82, 117, 108, 101, 83, 101, 116,
            58, 32, 78, 111, 110, 101, 10, 32, 81, 117, 97, 108, 105, 102, 105, 101, 114, 77, 117,
            115, 116, 83, 116, 97, 114, 116, 82, 97, 99, 101, 58, 32, 48, 10, 32, 78, 117, 109, 67,
            97, 114, 67, 108, 97, 115, 115, 101, 115, 58, 32, 49, 10, 32, 78, 117, 109, 67, 97,
            114, 84, 121, 112, 101, 115, 58, 32, 49, 10, 32, 72, 101, 97, 116, 82, 97, 99, 105,
            110, 103, 58, 32, 48, 10, 32, 66, 117, 105, 108, 100, 84, 121, 112, 101, 58, 32, 82,
            101, 108, 101, 97, 115, 101, 10, 32, 66, 117, 105, 108, 100, 84, 97, 114, 103, 101,
            116, 58, 32, 77, 101, 109, 98, 101, 114, 115, 10, 32, 66, 117, 105, 108, 100, 86, 101,
            114, 115, 105, 111, 110, 58, 32, 50, 48, 50, 51, 46, 49, 49, 46, 48, 54, 46, 48, 50,
            10,
        ]
    }

    fn expected_session_info() -> SessionInfo {
        SessionInfo {
            weekend_info: Some(WeekendInfo {
                track_name: Some("daytona 2011 road".to_string()),
                track_id: Some("192".to_string()),
                track_length: Some("5.69 km".to_string()),
                track_length_official: Some("5.73 km".to_string()),
                track_display_name: Some("Daytona International Speedway".to_string()),
                track_display_short_name: Some("Daytona".to_string()),
                track_config_name: Some("Road Course".to_string()),
                track_city: Some("Daytona Beach".to_string()),
                track_country: Some("USA".to_string()),
                track_altitude: Some("11.13 m".to_string()),
                track_latitude: Some("29.187712 m".to_string()),
                track_longitude: Some("-81.072798 m".to_string()),
                track_north_offset: Some("3.7742 rad".to_string()),
                track_num_turns: Some(11),
                track_pit_speed_limit: Some("87.98 kph".to_string()),
                track_type: Some("road course".to_string()),
                track_direction: Some("neutral".to_string()),
                track_weather_type: Some("Specified / Static Sky".to_string()),
                track_skies: Some("Mostly Cloudy".to_string()),
                track_surface_temp: Some("26.00 C".to_string()),
                track_air_temp: Some("20.00 C".to_string()),
                track_air_pressure: Some("29.88 Hg".to_string()),
                track_wind_vel: Some("1.79 m/s".to_string()),
                track_wind_dir: Some("2.36 rad".to_string()),
                track_relative_humidity: Some("63 %".to_string()),
                track_fog_level: Some("0 %".to_string()),
                track_cleanup: Some(1),
                track_dynamic_track: Some(1),
                track_version: Some("2023.08.25.02".to_string()),
                series_id: Some(0),
                season_id: Some(0),
                session_id: Some(0),
                sub_session_id: Some(0),
                league_id: Some(0),
                official: Some(0),
                race_week: Some(0),
                event_type: Some("Test".to_string()),
                category: Some("Road".to_string()),
                sim_mode: Some("full".to_string()),
                team_racing: Some(0),
                min_drivers: Some(0),
                max_drivers: Some(0),
                dc_rule_set: Some("None".to_string()),
                qualifier_must_start_race: Some(0),
                num_car_classes: Some(1),
                num_car_types: Some(1),
                heat_racing: Some(0),
                build_type: Some("Release".to_string()),
                build_target: Some("Members".to_string()),
                build_version: Some("2023.11.06.02".to_string()),
                weekend_options: None,
                telemetry_options: None,
            }),
            session_info: None,
            camera_info: None,
            radio_info: None,
            driver_info: None,
            split_time_info: None,
            car_setup: None,
        }
    }

    #[test]
    fn try_from_u8_vec_ok() {
        let result = SessionInfo::try_from(&test_bytes());
        let expected_result = Ok(expected_session_info());
        assert_eq!(result, expected_result)
    }

    #[test]
    fn try_from_u8_vec_invalid_type_ko() {
        let wrong_test_bytes: Vec<u8> = [0xFF; 5].to_vec();
        let result = SessionInfo::try_from(&wrong_test_bytes);
        let expected_result = Err(Error::ParseYaml(
            "invalid type: string \"�����\", expected struct SessionInfo".to_string(),
        ));
        assert_eq!(result, expected_result)
    }

    #[test]
    fn try_from_u8_vec_control_characters_ko() {
        let wrong_test_bytes: Vec<u8> = [0x2; 3].to_vec();
        let result = SessionInfo::try_from(&wrong_test_bytes);
        let expected_result = Err(Error::ParseYaml(
            "control characters are not allowed".to_string(),
        ));
        assert_eq!(result, expected_result)
    }
}
