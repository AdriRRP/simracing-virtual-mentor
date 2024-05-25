use crate::ibt::domain::file::metrics::Metrics as IbtMetrics;
use crate::ibt::domain::file::session_info::driver_info::driver::Driver;
use crate::ibt::domain::file::session_info::SessionInfo;
use crate::ibt::domain::file::var_value::primitive::Primitive;
use crate::ibt::domain::file::var_value::VarValue;
use crate::lap::domain::lap::metrics::Metrics;
use crate::lap::domain::lap::Lap;
use crate::lap::domain::laps::Laps;

use chrono::{DateTime, NaiveDateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;

macro_rules! extract_values {
    ($metrics:expr, $metric_name:expr, $primitive_variant:ident $(, $output_type:ty)?) => {{
        $metrics
            .iter()
            .find(|m| m.var_header.name() == $metric_name)
            .map(|metric| {
                metric
                    .var_values
                    .iter()
                    .flat_map(|vv| match vv {
                        VarValue::Single(Primitive::$primitive_variant(val)) => {
                                let output = Some(*val);
                                $(
                                    let output = output.map(|original| original as $output_type);
                                )?
                                output
                            },
                        _ => None, // TODO: Log the error
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_else(|| Vec::new())
    }};
}

#[must_use]
pub fn ibt_metrics2laps(file_id: &str, session_info: &SessionInfo, metrics: &IbtMetrics) -> Laps {
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

    let metrics_by_lap: HashMap<u16, Metrics> = group_metrics_by_lap(metrics);

    let laps_vec: Vec<Lap> = metrics_by_lap
        .iter()
        .map(|(lap_number, metrics)| {
            Lap::new(
                Uuid::new_v4(),
                file_id.to_string(),
                *lap_number,
                driver_name.clone(),
                category.clone(),
                car.clone(),
                circuit.clone(),
                date,
                metrics.clone(),
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

fn group_metrics_by_lap(metrics: &IbtMetrics) -> HashMap<u16, Metrics> {
    let lap: Vec<u16> = extract_values!(metrics, "Lap", Int, u16);
    // TODO: Check all vectors have the same size
    let speed: Vec<f32> = extract_values!(metrics, "Speed", Float);
    let throttle: Vec<f32> = extract_values!(metrics, "Throttle", Float);
    let brake: Vec<f32> = extract_values!(metrics, "Brake", Float);
    let clutch: Vec<f32> = extract_values!(metrics, "Clutch", Float);
    let gear: Vec<u8> = extract_values!(metrics, "Gear", Int, u8);
    let rpm: Vec<f32> = extract_values!(metrics, "RPM", Float);
    let distance: Vec<f32> = extract_values!(metrics, "LapDist", Float);
    let distance_pct: Vec<f32> = extract_values!(metrics, "LapDistPct", Float);
    let track_temp: Vec<f32> = extract_values!(metrics, "TrackTempCrew", Float);
    let latitude: Vec<f64> = extract_values!(metrics, "Lat", Double);
    let longitude: Vec<f64> = extract_values!(metrics, "Lon", Double);
    let altitude: Vec<f32> = extract_values!(metrics, "Alt", Float);
    let steering_wheel_angle: Vec<f32> = extract_values!(metrics, "SteeringWheelAngle", Float);
    let fuel_level: Vec<f32> = extract_values!(metrics, "FuelLevel", Float);
    let lap_current_lap_time: Vec<f32> = extract_values!(metrics, "LapCurrentLapTime", Float);

    let mut groups = HashMap::new();

    (0..lap.len()).for_each(|i| {
        let lap_metrics = groups.entry(lap[i]).or_insert_with(Metrics::default);
        lap_metrics.speed.push(speed[i]);
        lap_metrics.throttle.push(throttle[i]);
        lap_metrics.brake.push(brake[i]);
        lap_metrics.clutch.push(clutch[i]);
        lap_metrics.gear.push(gear[i]);
        lap_metrics.rpm.push(rpm[i]);
        lap_metrics.distance.push(distance[i]);
        lap_metrics.distance_pct.push(distance_pct[i]);
        lap_metrics.track_temp.push(track_temp[i]);
        lap_metrics.latitude.push(latitude[i]);
        lap_metrics.longitude.push(longitude[i]);
        lap_metrics.altitude.push(altitude[i]);
        lap_metrics
            .steering_wheel_angle
            .push(steering_wheel_angle[i]);
        lap_metrics.fuel_level.push(fuel_level[i]);
        lap_metrics
            .lap_current_lap_time
            .push(lap_current_lap_time[i]);
    });

    groups
}
