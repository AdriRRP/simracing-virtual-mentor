use shared::ibt::domain::file::session_info::driver_info::driver::Driver;
use shared::ibt::domain::file::session_info::SessionInfo;
use shared::ibt::domain::file::var_value::primitive::Primitive;
use shared::ibt::domain::file::var_value::VarValue;
use shared::ibt::domain::file::variables::Variables as IbtVariables;
use shared::lap::domain::lap::variables::Variables;
use shared::lap::domain::lap::Lap;
use shared::lap::domain::laps::Laps;

use chrono::{DateTime, NaiveDateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;

macro_rules! extract_values {
    ($variables:expr, $variable_name:expr, $primitive_variant:ident $(, $output_type:ty)?) => {{
        $variables
            .iter()
            .find(|m| m.var_header.name() == $variable_name)
            .map(|variable| {
                variable
                    .var_values
                    .iter()
                    .flat_map(|vv| match vv {
                        VarValue::Single(Primitive::$primitive_variant(val)) => {
                                let output = Some(*val);
                                $(
                                    #[allow(clippy::cast_possible_truncation)]
                                    #[allow(clippy::cast_sign_loss)]
                                    let output = output.map(|original| original as $output_type);
                                )?
                                output
                            }
                        _ => None, // TODO: Log the error
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_else(|| Vec::new())
    }};
}

#[must_use]
pub fn ibt_variables2laps(
    file_id: &str,
    session_info: &SessionInfo,
    variables: &IbtVariables,
) -> Laps {
    let driver = get_driver_or_none(session_info);

    let driver_name = driver
        .clone()
        .and_then(|x| x.user_name)
        .unwrap_or_else(|| "Unknown".to_string()); // TODO: Log the error

    let category = session_info
        .weekend_info
        .clone()
        .and_then(|wi| wi.category)
        .unwrap_or_else(|| "Unknown".to_string()); // TODO: Log the error

    let car = driver
        .and_then(|x| x.car_screen_name)
        .unwrap_or_else(|| "Unknown".to_string()); // TODO: Log the error

    let circuit = session_info
        .weekend_info
        .clone()
        .and_then(|wi| wi.track_display_name)
        .unwrap_or_else(|| "Unknown".to_string()); // TODO: Log the error

    let date = get_datetime_or_now(session_info);

    let variables_by_lap: HashMap<u16, Variables> = group_variables_by_lap(variables);

    let laps_vec: Vec<Lap> = variables_by_lap
        .iter()
        .map(|(lap_number, variables)| {
            Lap::new(
                Uuid::new_v4(),
                file_id.to_string(),
                *lap_number,
                driver_name.clone(),
                category.clone(),
                car.clone(),
                circuit.clone(),
                date,
                variables.clone(),
            )
        })
        .collect();

    Laps::from(laps_vec)
}

fn get_driver_or_none(session_info: &SessionInfo) -> Option<Driver> {
    session_info
        .driver_info
        .clone()
        .and_then(|di| di.driver_user_id)
        .and_then(|driver_id| {
            // TODO: Log the error
            session_info
                .driver_info
                .clone()
                .and_then(|di| di.drivers)
                .and_then(|drivers| {
                    drivers
                        .iter()
                        .find(|driver| driver.user_id == Some(driver_id))
                        .cloned()
                })
        })
}

fn get_datetime_or_now(session_info: &SessionInfo) -> DateTime<Utc> {
    let date = session_info
        .weekend_info
        .clone()
        .and_then(|wi| wi.weekend_options)
        .and_then(|wo| wo.date)
        .unwrap_or_else(|| "Unknown".to_string()); // TODO: Log the error

    let time = session_info
        .weekend_info
        .clone()
        .and_then(|wi| wi.weekend_options)
        .and_then(|wo| wo.time_of_day)
        .unwrap_or_else(|| "Unknown".to_string()); // TODO: Log the error

    match NaiveDateTime::parse_from_str(
        format!("{date} {time}").to_uppercase().as_str(),
        "%Y-%m-%d %I:%M %p",
    ) {
        Ok(datetime) => Some(datetime.and_utc()),
        Err(_e) => {
            // TODO: Log the error
            // eprintln!("{e}");
            None
        }
    }
    .unwrap_or_else(Utc::now)
}

fn group_variables_by_lap(variables: &IbtVariables) -> HashMap<u16, Variables> {
    let lap: Vec<u16> = extract_values!(variables, "Lap", Int, u16);
    // TODO: Check all vectors have the same size
    let speed: Vec<f32> = extract_values!(variables, "Speed", Float);
    let throttle: Vec<f32> = extract_values!(variables, "Throttle", Float);
    let brake: Vec<f32> = extract_values!(variables, "Brake", Float);
    let clutch: Vec<f32> = extract_values!(variables, "Clutch", Float);
    let gear: Vec<i8> = extract_values!(variables, "Gear", Int, i8);
    let rpm: Vec<f32> = extract_values!(variables, "RPM", Float);
    let distance: Vec<f32> = extract_values!(variables, "LapDist", Float);
    let distance_pct: Vec<f32> = extract_values!(variables, "LapDistPct", Float);
    let track_temp: Vec<f32> = extract_values!(variables, "TrackTempCrew", Float);
    let latitude: Vec<f64> = extract_values!(variables, "Lat", Double);
    let longitude: Vec<f64> = extract_values!(variables, "Lon", Double);
    let altitude: Vec<f32> = extract_values!(variables, "Alt", Float);
    let steering_wheel_angle: Vec<f32> = extract_values!(variables, "SteeringWheelAngle", Float);
    let fuel_level: Vec<f32> = extract_values!(variables, "FuelLevel", Float);
    let lap_current_lap_time: Vec<f32> = extract_values!(variables, "LapCurrentLapTime", Float);

    let mut groups = HashMap::new();

    (0..lap.len()).for_each(|i| {
        let lap_variables = groups.entry(lap[i]).or_insert_with(Variables::default);
        lap_variables.speed.push(speed[i]);
        lap_variables.throttle.push(throttle[i]);
        lap_variables.brake.push(brake[i]);
        lap_variables.clutch.push(clutch[i]);
        lap_variables.gear.push(gear[i]);
        lap_variables.rpm.push(rpm[i]);
        lap_variables.distance.push(distance[i]);
        lap_variables.distance_pct.push(distance_pct[i]);
        lap_variables.track_temp.push(track_temp[i]);
        lap_variables.latitude.push(latitude[i]);
        lap_variables.longitude.push(longitude[i]);
        lap_variables.altitude.push(altitude[i]);
        lap_variables
            .steering_wheel_angle
            .push(steering_wheel_angle[i]);
        lap_variables.fuel_level.push(fuel_level[i]);
        lap_variables
            .lap_current_lap_time
            .push(lap_current_lap_time[i]);
    });

    groups
}
