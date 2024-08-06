use crate::lap::domain::lap::variables::Variables;
use std::ops::Sub;

#[must_use]
pub fn calculate(ref_variables: &Variables, target_variables: &Variables) -> Variables {
    Variables {
        speed: calculate_difference(&ref_variables.speed, &target_variables.speed),
        throttle: calculate_difference(&ref_variables.throttle, &target_variables.throttle),
        brake: calculate_difference(&ref_variables.brake, &target_variables.brake),
        clutch: calculate_difference(&ref_variables.clutch, &target_variables.clutch),
        gear: calculate_difference(&ref_variables.gear, &target_variables.gear),
        rpm: calculate_difference(&ref_variables.rpm, &target_variables.rpm),
        distance: calculate_difference(&ref_variables.distance, &target_variables.distance),
        distance_pct: calculate_difference(
            &ref_variables.distance_pct,
            &target_variables.distance_pct,
        ),
        track_temp: calculate_difference(&ref_variables.track_temp, &target_variables.track_temp),
        latitude: calculate_difference(&ref_variables.latitude, &target_variables.latitude),
        longitude: calculate_difference(&ref_variables.longitude, &target_variables.longitude),
        altitude: calculate_difference(&ref_variables.altitude, &target_variables.altitude),
        steering_wheel_angle: calculate_difference(
            &ref_variables.steering_wheel_angle,
            &target_variables.steering_wheel_angle,
        ),
        fuel_level: calculate_difference(&ref_variables.fuel_level, &target_variables.fuel_level),
        lap_current_lap_time: calculate_difference(
            &ref_variables.lap_current_lap_time,
            &target_variables.lap_current_lap_time,
        ),
    }
}

fn calculate_difference<T>(vec1: &[T], vec2: &[T]) -> Vec<T::Output>
where
    T: Sub<Output = T> + Copy,
{
    vec1.iter().zip(vec2).map(|(&v1, &v2)| v1 - v2).collect()
}
