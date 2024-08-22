use crate::analysis::domain::analysis::fcm_grid::{Config, FcmGrid};
use crate::lap::domain::lap::variables::Variables;

use ndarray::Array2;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct ClustersMemberships {
    pub speed: Vec<Vec<f64>>,
    pub throttle: Vec<Vec<f64>>,
    pub brake: Vec<Vec<f64>>,
    pub gear: Vec<Vec<f64>>,
    pub steering_wheel_angle: Vec<Vec<f64>>,
}

impl ClustersMemberships {
    /// Transforms and fits the Fuzzy C-Means (FCM) model to various variables from `diff_variables`.
    ///
    /// This function performs the following steps:
    /// 1. Initializes a `FcmGrid` with predefined parameter ranges for grid search.
    /// 2. Converts each variable from `diff_variables` into a 2D `Array2<f64>`.
    /// 3. Performs grid search to find the best FCM model for each variable.
    /// 4. Extracts membership values from the best model and stores them in the result struct.
    ///
    /// # Arguments
    ///
    /// * `diff_variables` - A reference to a `Variables` struct containing different variables to be fitted.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - The `Self` struct populated with membership values for each variable if successful.
    /// * `Err(String)` - An error message if any operation fails, including errors from model creation,
    ///   fitting, or data transformation.
    ///
    /// # Errors
    ///
    /// * `Failed to create ndarray` - Occurs if the `Array2::from_shape_vec` call fails due to shape or data issues.
    /// * `Error during FCM fitting` - Occurs if there is an error while creating or fitting the FCM model,
    ///   which could be due to invalid model parameters or fitting issues.
    pub fn try_transform_and_fit(diff_variables: &Variables, fcm_grid_config: &Config) -> Result<Self, String> {
        let fcm_grid = FcmGrid::new(fcm_grid_config.clone());

        let variables: Vec<(&str, Vec<f64>)> = vec![
            (
                "speed",
                diff_variables.speed.iter().map(|&g| f64::from(g)).collect(),
            ),
            (
                "throttle",
                diff_variables
                    .throttle
                    .iter()
                    .map(|&g| f64::from(g))
                    .collect(),
            ),
            (
                "brake",
                diff_variables.brake.iter().map(|&g| f64::from(g)).collect(),
            ),
            (
                "gear",
                diff_variables.gear.iter().map(|&g| f64::from(g)).collect(),
            ),
            (
                "steering_wheel_angle",
                diff_variables
                    .steering_wheel_angle
                    .iter()
                    .map(|&g| f64::from(g))
                    .collect(),
            ),
        ];

        let mut result = Self::default();

        for (name, data) in variables {
            let data = Array2::from_shape_vec((data.len(), 1), data).map_err(|e| format!("{e}"))?;
            let best_model = fcm_grid
                .get_best_fitted_model(&data)
                .map_err(|e| format!("{e}"))?;
            let memberships = best_model.memberships();

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

        Ok(result)
    }
}
