use crate::lap::domain::lap::metrics::Metrics;
use crate::lap::domain::lap::Lap;

use std::ops::Sub;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents an analysis entity, containing an identifier, name, and a list of laps.
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Analysis {
    /// The unique identifier for the analysis.
    pub id: Uuid,
    /// The name of the analysis.
    pub name: String,
    /// Date on which the analysis was created
    pub date: DateTime<Utc>,
    /// The circuit on which the analysis is performed
    pub circuit: String,

    /// Interpolated common distance
    pub union_distance: Vec<f32>,

    /// Reference lap
    pub ref_lap_number: u16,
    pub ref_lap_driver: String,
    pub ref_lap_category: String,
    pub ref_lap_car: String,
    pub ref_lap_metrics: Metrics,

    /// Reference lap
    pub target_lap_number: u16,
    pub target_lap_driver: String,
    pub target_lap_category: String,
    pub target_lap_car: String,
    pub target_lap_metrics: Metrics,

    /// Difference
    pub difference_metrics: Metrics,
}

impl Analysis {
    pub fn analyze(id: Uuid, name: String, ref_lap: Lap, target_lap: Lap) -> Result<Self, Error> {
        if ref_lap.header.circuit != target_lap.header.circuit {
            return Err(Error::DifferentCircuits(
                ref_lap.header.circuit,
                target_lap.header.circuit,
            ));
        }

        let union_distances = Self::generate_union_distances(&ref_lap, &target_lap);
        let ref_metrics = Self::interpolate_metrics(&ref_lap.metrics, &union_distances)?;
        let target_metrics = Self::interpolate_metrics(&target_lap.metrics, &union_distances)?;
        let difference_metrics = Self::calculate_differences(&ref_metrics, &target_metrics);

        Ok(Self {
            id,
            name,
            date: Utc::now(),
            circuit: ref_lap.header.circuit.clone(),
            union_distance: union_distances,
            ref_lap_number: ref_lap.header.number,
            ref_lap_driver: ref_lap.header.driver.clone(),
            ref_lap_category: ref_lap.header.category.clone(),
            ref_lap_car: ref_lap.header.car.clone(),
            ref_lap_metrics: ref_metrics,
            target_lap_number: target_lap.header.number,
            target_lap_driver: target_lap.header.driver.clone(),
            target_lap_category: target_lap.header.category.clone(),
            target_lap_car: target_lap.header.car.clone(),
            target_lap_metrics: target_metrics,
            difference_metrics,
        })
    }

    fn generate_union_distances(lap1: &Lap, lap2: &Lap) -> Vec<f32> {
        let mut distances: Vec<f32> = lap1
            .metrics
            .distance
            .iter()
            .chain(lap2.metrics.distance.iter())
            .cloned()
            .collect();
        distances.sort_by(|a, b| a.partial_cmp(b).unwrap()); // TODO: Posible error!
        distances.dedup();
        distances
    }

    fn interpolate_metrics(metrics: &Metrics, distances: &Vec<f32>) -> Result<Metrics, Error> {
        let distances_f64 = distances.iter().map(|&x| f64::from(x)).collect();

        Ok(Metrics {
            speed: Self::try_f32_interpolation(
                &metrics.speed,
                &metrics.distance,
                &distances_f64,
                false,
            )?,
            throttle: Self::try_f32_interpolation(
                &metrics.throttle,
                &metrics.distance,
                &distances_f64,
                false,
            )?,
            brake: Self::try_f32_interpolation(
                &metrics.brake,
                &metrics.distance,
                &distances_f64,
                false,
            )?,
            clutch: Self::try_f32_interpolation(
                &metrics.clutch,
                &metrics.distance,
                &distances_f64,
                false,
            )?,
            gear: Self::try_i8_interpolation(
                &metrics.gear,
                &metrics.distance,
                &distances_f64,
                true,
            )?,
            rpm: Self::try_f32_interpolation(
                &metrics.rpm,
                &metrics.distance,
                &distances_f64,
                false,
            )?,
            distance: distances.clone(),
            distance_pct: Self::try_f32_interpolation(
                &metrics.distance_pct,
                &metrics.distance,
                &distances_f64,
                false,
            )?,
            track_temp: Self::try_f32_interpolation(
                &metrics.track_temp,
                &metrics.distance,
                &distances_f64,
                false,
            )?,
            latitude: Self::interpolate_vector(
                &metrics.latitude,
                &metrics.distance.iter().map(|&d| f64::from(d)).collect(),
                &distances_f64,
                false,
            ),
            longitude: Self::interpolate_vector(
                &metrics.longitude,
                &metrics.distance.iter().map(|&d| f64::from(d)).collect(),
                &distances_f64,
                false,
            ),
            altitude: Self::try_f32_interpolation(
                &metrics.altitude,
                &metrics.distance,
                &distances_f64,
                false,
            )?,
            steering_wheel_angle: Self::try_f32_interpolation(
                &metrics.steering_wheel_angle,
                &metrics.distance,
                &distances_f64,
                false,
            )?,
            fuel_level: Self::try_f32_interpolation(
                &metrics.fuel_level,
                &metrics.distance,
                &distances_f64,
                false,
            )?,
            lap_current_lap_time: Self::try_f32_interpolation(
                &metrics.lap_current_lap_time,
                &metrics.distance,
                &distances_f64,
                false,
            )?,
        })
    }

    fn try_f32_interpolation(
        target: &Vec<f32>,
        original_distances: &Vec<f32>,
        distances: &Vec<f64>,
        is_discrete: bool,
    ) -> Result<Vec<f32>, Error> {
        let target = target.iter().map(|&x| f64::from(x)).collect();
        let original_distances = original_distances.iter().map(|&x| f64::from(x)).collect();
        Self::interpolate_vector(
            &target,
            &original_distances,
            distances,
            is_discrete,
        )
        .iter()
        .map(|&x| Ok(x as f32)) // TODO: Revisar
        //.map(|&x| f32::try_from(x).map_err(|e| Error::InterpolatingMetrics(e.to_string())))
        .collect()
    }

    fn try_i8_interpolation(
        target: &Vec<i8>,
        original_distances: &Vec<f32>,
        distances: &Vec<f64>,
        is_discrete: bool,
    ) -> Result<Vec<i8>, Error> {
        let target = target.iter().map(|&x| f64::from(x)).collect();
        let original_distances = original_distances.iter().map(|&x| f64::from(x)).collect();
        Self::interpolate_vector(
            &target,
            &original_distances,
            distances,
            is_discrete,
        )
        .iter()
            .map(|&x| Ok(x as i8)) // TODO: Revisar
        //.map(|x| u8::try_from(x).map_err(|e| Error::InterpolatingMetrics(e.to_string())))
        .collect()
    }

    fn interpolate_vector(
        values: &Vec<f64>,
        distances: &Vec<f64>,
        new_distances: &Vec<f64>,
        is_discrete: bool,
    ) -> Vec<f64> {
        let mut interpolated_values = Vec::new();

        for &new_distance in new_distances.iter() {
            let value = match distances.iter().position(|&d| d >= new_distance) {
                Some(0) => values[0],
                Some(pos) if pos == distances.len() => values[values.len() - 1],
                Some(pos) => {
                    let d0 = distances[pos - 1];
                    let d1 = distances[pos];
                    let v0 = values[pos - 1];
                    let v1 = values[pos];
                    let interpolated = v0 + (new_distance - d0) / (d1 - d0) * (v1 - v0);
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

    fn calculate_differences(ref_metrics: &Metrics, target_metrics: &Metrics) -> Metrics {
        Metrics {
            speed: Self::calculate_difference(&ref_metrics.speed, &target_metrics.speed),
            throttle: Self::calculate_difference(&ref_metrics.throttle, &target_metrics.throttle),
            brake: Self::calculate_difference(&ref_metrics.brake, &target_metrics.brake),
            clutch: Self::calculate_difference(&ref_metrics.clutch, &target_metrics.clutch),
            gear: Self::calculate_difference(&ref_metrics.gear, &target_metrics.gear),
            rpm: Self::calculate_difference(&ref_metrics.rpm, &target_metrics.rpm),
            distance: Self::calculate_difference(&ref_metrics.distance, &target_metrics.distance),
            distance_pct: Self::calculate_difference(
                &ref_metrics.distance_pct,
                &target_metrics.distance_pct,
            ),
            track_temp: Self::calculate_difference(
                &ref_metrics.track_temp,
                &target_metrics.track_temp,
            ),
            latitude: Self::calculate_difference(&ref_metrics.latitude, &target_metrics.latitude),
            longitude: Self::calculate_difference(
                &ref_metrics.longitude,
                &target_metrics.longitude,
            ),
            altitude: Self::calculate_difference(&ref_metrics.altitude, &target_metrics.altitude),
            steering_wheel_angle: Self::calculate_difference(
                &ref_metrics.steering_wheel_angle,
                &target_metrics.steering_wheel_angle,
            ),
            fuel_level: Self::calculate_difference(
                &ref_metrics.fuel_level,
                &target_metrics.fuel_level,
            ),
            lap_current_lap_time: Self::calculate_difference(
                &ref_metrics.lap_current_lap_time,
                &target_metrics.lap_current_lap_time,
            ),
        }
    }

    fn calculate_difference<T>(vec1: &[T], vec2: &[T]) -> Vec<T::Output>
        where
            T: Sub<Output = T>  + Copy,
    {
        vec1.iter().zip(vec2).map(|(&v1, &v2)| v1 - v2).collect()
    }
}

#[derive(PartialEq, Eq, Debug, thiserror::Error)]
pub enum Error {
    #[error("the reference lap has been run on Circuit `{0}` while the target lap has been run on Circuit `{0}`")]
    DifferentCircuits(String, String),
    #[error("error interpolating metrics: {0}")]
    InterpolatingMetrics(String),
}
