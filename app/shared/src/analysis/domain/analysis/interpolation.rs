use crate::analysis::domain::analysis::Error;
use crate::lap::domain::lap::variables::Variables;

pub fn interpolate_variables(variables: &Variables, distances: &[f32]) -> Result<Variables, Error> {
    let distances_f64: Vec<f64> = distances.iter().map(|&x| f64::from(x)).collect();

    Ok(Variables {
        speed: try_f32_interpolation(&variables.speed, &variables.distance, &distances_f64, false)?,
        throttle: try_f32_interpolation(
            &variables.throttle,
            &variables.distance,
            &distances_f64,
            false,
        )?,
        brake: try_f32_interpolation(&variables.brake, &variables.distance, &distances_f64, false)?,
        clutch: try_f32_interpolation(
            &variables.clutch,
            &variables.distance,
            &distances_f64,
            false,
        )?,
        gear: try_i8_interpolation(&variables.gear, &variables.distance, &distances_f64, true)?,
        rpm: try_f32_interpolation(&variables.rpm, &variables.distance, &distances_f64, false)?,
        distance: distances.to_owned(),
        distance_pct: try_f32_interpolation(
            &variables.distance_pct,
            &variables.distance,
            &distances_f64,
            false,
        )?,
        track_temp: try_f32_interpolation(
            &variables.track_temp,
            &variables.distance,
            &distances_f64,
            false,
        )?,
        latitude: interpolate_vector(
            &variables.latitude,
            &variables
                .distance
                .iter()
                .map(|&d| f64::from(d))
                .collect::<Vec<f64>>(),
            &distances_f64,
            false,
        ),
        longitude: interpolate_vector(
            &variables.longitude,
            &variables
                .distance
                .iter()
                .map(|&d| f64::from(d))
                .collect::<Vec<f64>>(),
            &distances_f64,
            false,
        ),
        altitude: try_f32_interpolation(
            &variables.altitude,
            &variables.distance,
            &distances_f64,
            false,
        )?,
        steering_wheel_angle: try_f32_interpolation(
            &variables.steering_wheel_angle,
            &variables.distance,
            &distances_f64,
            false,
        )?,
        fuel_level: try_f32_interpolation(
            &variables.fuel_level,
            &variables.distance,
            &distances_f64,
            false,
        )?,
        lap_current_lap_time: try_f32_interpolation(
            &variables.lap_current_lap_time,
            &variables.distance,
            &distances_f64,
            false,
        )?,
    })
}

fn try_f32_interpolation(
    target: &[f32],
    original_distances: &[f32],
    distances: &[f64],
    is_discrete: bool,
) -> Result<Vec<f32>, Error> {
    let target = target.iter().map(|&x| f64::from(x)).collect::<Vec<f64>>();
    let original_distances = original_distances
        .iter()
        .map(|&x| f64::from(x))
        .collect::<Vec<f64>>();
    interpolate_vector(&target, &original_distances, distances, is_discrete)
        .iter()
        .map(|&x| Ok(x as f32)) // TODO: Revisar
        //.map(|&x| f32::try_from(x).map_err(|e| Error::InterpolatingVariables(format!("{e}"))))
        .collect()
}

fn try_i8_interpolation(
    target: &[i8],
    original_distances: &[f32],
    distances: &[f64],
    is_discrete: bool,
) -> Result<Vec<i8>, Error> {
    let target = target.iter().map(|&x| f64::from(x)).collect::<Vec<f64>>();
    let original_distances = original_distances
        .iter()
        .map(|&x| f64::from(x))
        .collect::<Vec<f64>>();
    interpolate_vector(&target, &original_distances, distances, is_discrete)
        .iter()
        .map(|&x| Ok(x as i8)) // TODO: Revisar
        //.map(|x| i8::try_from(x).map_err(|e| Error::InterpolatingVariables(format!("{e}"))))
        .collect()
}

fn interpolate_vector(
    values: &[f64],
    distances: &[f64],
    new_distances: &[f64],
    is_discrete: bool,
) -> Vec<f64> {
    let mut interpolated_values = Vec::new();

    for &new_distance in new_distances {
        let value = match distances.iter().position(|&d| d >= new_distance) {
            Some(0) => values[0],
            Some(pos) if pos == distances.len() => values[values.len() - 1],
            Some(pos) => {
                let d0 = distances[pos - 1];
                let d1 = distances[pos];
                let v0 = values[pos - 1];
                let v1 = values[pos];

                // More efficient than: v0 + (new_distance - d0) / (d1 - d0) * (v1 - v0)
                let interpolated = ((new_distance - d0) / (d1 - d0)).mul_add(v1 - v0, v0);

                if is_discrete {
                    interpolated.round()
                } else {
                    interpolated
                }
            }
            None => values[values.len() - 1],
        };
        interpolated_values.push(value);
    }

    interpolated_values
}
