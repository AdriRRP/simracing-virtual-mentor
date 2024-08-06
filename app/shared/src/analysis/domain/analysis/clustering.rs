use crate::analysis::domain::analysis::fuzzy_c_means::FuzzyCMeans;
use crate::lap::domain::lap::variables::Variables;
use ndarray::Array1;
use serde::{Deserialize, Serialize};

pub mod options;

#[derive(Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct Clustering {
    pub speed: Vec<Vec<f64>>,
    pub throttle: Vec<Vec<f64>>,
    pub brake: Vec<Vec<f64>>,
    pub gear: Vec<Vec<f64>>,
    pub steering_wheel_angle: Vec<Vec<f64>>,
}

impl Clustering {
    pub fn new(diff_variables: Variables) -> Self {
        let num_clusters = 5;
        let fuzziness = 2.0;
        let max_iter = 1000;
        let tolerance = 1e-6;

        let variables: Vec<(&str, Vec<f64>)> = vec![
            (
                "speed",
                diff_variables.speed.iter().map(|&g| g as f64).collect(),
            ),
            (
                "throttle",
                diff_variables.throttle.iter().map(|&g| g as f64).collect(),
            ),
            (
                "brake",
                diff_variables.brake.iter().map(|&g| g as f64).collect(),
            ),
            (
                "gear",
                diff_variables.gear.iter().map(|&g| g as f64).collect(),
            ),
            (
                "steering_wheel_angle",
                diff_variables
                    .steering_wheel_angle
                    .iter()
                    .map(|&g| g as f64)
                    .collect(),
            ),
        ];

        let mut result = Self::default();

        for (name, data) in variables {
            let mut fcm = FuzzyCMeans::new(num_clusters, 1, fuzziness);
            let nd_data = Array1::from(data);
            fcm.initialize_centroids(&nd_data, num_clusters);
            fcm.fit(&nd_data, max_iter, tolerance);
            let memberships = fcm.predict();

            match name {
                "speed" => result.speed = memberships,
                "throttle" => result.throttle = memberships,
                "brake" => result.brake = memberships,
                "gear" => result.gear = memberships,
                "steering_wheel_angle" => result.steering_wheel_angle = memberships,
                _ => (),
            }
        }

        result
    }
}
