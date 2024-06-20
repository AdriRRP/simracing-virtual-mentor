pub mod header;
pub mod headers;
pub mod reference_lap;

use crate::analysis::domain::analysis::header::Header;
use crate::analysis::domain::analysis::reference_lap::ReferenceLap;
use crate::lap::domain::lap::metrics::Metrics;
use crate::lap::domain::lap::Lap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::ops::Sub;
use uuid::Uuid;

/// Represents an analysis entity, containing an identifier, name, and a list of laps.
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Analysis {
    /// Information about analysis
    pub header: Header,

    /// Laps to compare each other
    pub reference: ReferenceLap,
    pub target: ReferenceLap,

    /// Interpolated common distance
    pub union_distances: Vec<f32>,

    /// Difference metrics: reference - target
    pub differences: Metrics,
}

impl Analysis {
    /// Analyzes the difference between a reference lap and a target lap, generating various metrics.
    ///
    /// # Arguments
    ///
    /// * `id` - A `Uuid` representing the unique identifier for the analysis.
    /// * `name` - A `String` representing the name of the analysis.
    /// * `ref_lap` - A `Lap` representing the reference lap.
    /// * `target_lap` - A `Lap` representing the target lap.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing `Self` on success, or an `Error` on failure.
    ///
    /// # Errors
    ///
    /// This function returns an error if:
    ///
    /// * The reference lap and the target lap are from different circuits. In this case,
    ///   an `Error::DifferentCircuits` is returned with the circuit information of both laps.
    /// * Any error occurs during the interpolation of metrics. This might be due to issues
    ///   with the data points not aligning properly between the reference and target laps.
    ///
    /// # Note
    ///
    /// Ensure that both `ref_lap` and `target_lap` are from the same circuit to avoid the `DifferentCircuits` error.
    ///
    pub fn analyze(
        id: Uuid,
        name: String,
        date: DateTime<Utc>,
        ref_lap: Lap,
        target_lap: Lap,
    ) -> Result<Self, Error> {
        if ref_lap.header.circuit != target_lap.header.circuit {
            return Err(Error::DifferentCircuits(
                ref_lap.header.circuit,
                target_lap.header.circuit,
            ));
        }

        let circuit = ref_lap.header.circuit.clone();
        let union_distances = Self::generate_union_distances(&ref_lap, &target_lap);
        let ref_metrics = Self::interpolate_metrics(&ref_lap.metrics, &union_distances)?;
        let target_metrics = Self::interpolate_metrics(&target_lap.metrics, &union_distances)?;
        let differences = Self::calculate_differences(&ref_metrics, &target_metrics);

        Ok(Self {
            header: Header::new(id, name, date, circuit),
            reference: ReferenceLap::new(
                ref_lap.header.number,
                ref_lap.header.driver.clone(),
                ref_lap.header.category.clone(),
                ref_lap.header.car,
                ref_metrics,
            ),
            target: ReferenceLap::new(
                target_lap.header.number,
                target_lap.header.driver.clone(),
                target_lap.header.category.clone(),
                target_lap.header.car,
                target_metrics,
            ),
            union_distances,
            differences,
        })
    }

    fn generate_union_distances(lap1: &Lap, lap2: &Lap) -> Vec<f32> {
        let mut distances: Vec<f32> = lap1
            .metrics
            .distance
            .iter()
            .chain(lap2.metrics.distance.iter())
            .copied()
            .collect();
        distances.sort_by(|a, b| a.partial_cmp(b).unwrap()); // TODO: Posible error!
        distances.dedup();
        distances
    }

    fn interpolate_metrics(metrics: &Metrics, distances: &[f32]) -> Result<Metrics, Error> {
        let distances_f64: Vec<f64> = distances.iter().map(|&x| f64::from(x)).collect();

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
            distance: distances.to_owned(),
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
                &metrics
                    .distance
                    .iter()
                    .map(|&d| f64::from(d))
                    .collect::<Vec<f64>>(),
                &distances_f64,
                false,
            ),
            longitude: Self::interpolate_vector(
                &metrics.longitude,
                &metrics
                    .distance
                    .iter()
                    .map(|&d| f64::from(d))
                    .collect::<Vec<f64>>(),
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
        Self::interpolate_vector(&target, &original_distances, distances, is_discrete)
            .iter()
            .map(|&x| Ok(x as f32)) // TODO: Revisar
            //.map(|&x| f32::try_from(x).map_err(|e| Error::InterpolatingMetrics(format!("{e}"))))
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
        Self::interpolate_vector(&target, &original_distances, distances, is_discrete)
            .iter()
            .map(|&x| Ok(x as i8)) // TODO: Revisar
            //.map(|x| i8::try_from(x).map_err(|e| Error::InterpolatingMetrics(format!("{e}"))))
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
        T: Sub<Output = T> + Copy,
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
