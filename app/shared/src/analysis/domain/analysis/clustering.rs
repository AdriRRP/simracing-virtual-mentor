use crate::analysis::domain::analysis::fuzzy_c_means::FuzzyCMeans;
use crate::lap::domain::lap::variables::Variables;
use ndarray::Array2;
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
    #[must_use]
    pub fn new(diff_variables: &Variables) -> Self {
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
            let mut fcm = FuzzyCMeans::new(num_clusters, fuzziness, max_iter, tolerance);
            let data = Array2::from_shape_vec((data.len(), 1), data).expect("shit");
            fcm.fit(&data);
            let memberships = fcm.fit(&data);

            match name {
                "speed" => {
                    result.speed = memberships.outer_iter().map(|row| row.to_vec()).collect();
                }
                "throttle" => {
                    result.throttle = memberships.outer_iter().map(|row| row.to_vec()).collect();
                }
                "brake" => {
                    result.brake = memberships.outer_iter().map(|row| row.to_vec()).collect();
                }
                "gear" => result.gear = memberships.outer_iter().map(|row| row.to_vec()).collect(),
                "steering_wheel_angle" => {
                    result.steering_wheel_angle =
                        memberships.outer_iter().map(|row| row.to_vec()).collect();
                }
                _ => (),
            }
        }

        result
    }
}
