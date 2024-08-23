use crate::analysis::domain::analysis::fuzzy_c_means::{FittedModel, FuzzyCMeans};
use ndarray::Array2;

/// Configuration for the FCM (Fuzzy C-Means) grid search.
#[derive(Clone)]
pub struct Config {
    /// Number of clusters (`c`), with optional max value and increment.
    pub c: (usize, Option<usize>, Option<usize>),
    /// Fuzziness parameter (`m`), with optional max value and increment.
    pub m: (f64, Option<f64>, Option<f64>),
    /// Maximum number of iterations (`max_iter`), with optional max value and increment.
    pub max_iter: (usize, Option<usize>, Option<usize>),
    /// Convergence error (`error`), with optional max value and increment.
    pub error: (f64, Option<f64>, Option<f64>),
}

impl Config {
    /// Creates a new configuration for the FCM grid search.
    ///
    /// # Arguments
    ///
    /// * `c` - Number of clusters (start, max, increment).
    /// * `m` - Fuzziness parameter (start, max, increment).
    /// * `max_iter` - Maximum number of iterations (start, max, increment).
    /// * `error` - Convergence error (start, max, increment).
    ///
    /// # Returns
    ///
    /// A new instance of `Config`.
    #[must_use]
    pub const fn new(
        c: (usize, Option<usize>, Option<usize>),
        m: (f64, Option<f64>, Option<f64>),
        max_iter: (usize, Option<usize>, Option<usize>),
        error: (f64, Option<f64>, Option<f64>),
    ) -> Self {
        Self {
            c,
            m,
            max_iter,
            error,
        }
    }
}

/// FCM (Fuzzy C-Means) grid search for finding the best model configuration.
pub struct FcmGrid {
    config: Config,
}

impl FcmGrid {
    /// Creates a new FCM grid search with the provided configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration for the grid search.
    ///
    /// # Returns
    ///
    /// A new instance of `FcmGrid`.
    #[must_use]
    pub const fn new(config: Config) -> Self {
        Self { config }
    }

    /// Performs the grid search and returns the best fitted model.
    ///
    /// # Arguments
    ///
    /// * `data` - Input data as a 2D array.
    ///
    /// # Returns
    ///
    /// A `Result` containing the best fitted model, or an error if no valid model was found.
    ///
    /// # Errors
    ///
    /// * `Error::CreatingModel` - If there is an error creating the model.
    /// * `Error::FittingModel` - If there is an error fitting the model.
    /// * `Error::NoValidModelFound` - If no valid model is found during the grid search.
    pub fn get_best_fitted_model(&self, data: &Array2<f64>) -> Result<FittedModel, Error> {
        let mut best_model: Option<FittedModel> = None;

        for c in Self::range(self.config.c) {
            for m in Self::range_f64(self.config.m) {
                for max_iter in Self::range(self.config.max_iter) {
                    for error in Self::range_f64(self.config.error) {
                        let model_result = FuzzyCMeans::try_new(c, m, max_iter, error)
                            .map_err(|e| Error::CreatingModel(e.to_string()))?
                            .try_fit(data)
                            .map_err(|e| Error::FittingModel(e.to_string()))?;

                        if best_model
                            .as_ref()
                            .map_or(true, |m| m.fpc() < model_result.fpc())
                        {
                            best_model = Some(model_result);
                        }
                    }
                }
            }
        }

        best_model.ok_or(Error::NoValidModelFound)
    }

    fn range(param: (usize, Option<usize>, Option<usize>)) -> Box<dyn Iterator<Item = usize>> {
        let (start, max, inc) = param;
        if let (Some(max), Some(inc)) = (max, inc) {
            Box::new((start..=max).step_by(inc))
        } else {
            Box::new(std::iter::once(start))
        }
    }

    fn range_f64(param: (f64, Option<f64>, Option<f64>)) -> Box<dyn Iterator<Item = f64>> {
        let (start, max, inc) = param;
        if let (Some(max), Some(inc)) = (max, inc) {
            Box::new(std::iter::successors(Some(start), move |&prev| {
                let next = prev + inc;
                if next <= max {
                    Some(next)
                } else {
                    None
                }
            }))
        } else {
            Box::new(std::iter::once(start))
        }
    }
}

/// Error type for the FCM grid search process.
#[derive(PartialEq, Eq, Debug, thiserror::Error)]
pub enum Error {
    #[error("Error creating model: {0}")]
    CreatingModel(String),
    #[error("Error fitting model: {0}")]
    FittingModel(String),
    #[error("No valid model found")]
    NoValidModelFound,
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_fcm_grid_no_increments() {
        let config = Config::new(
            (2, None, None),
            (2.0, None, None),
            (100, None, None),
            (0.01, None, None),
        );
        let fcm_grid = FcmGrid::new(config);
        let data = array![[1.0], [2.0], [3.0], [4.0], [5.0]];

        let result = fcm_grid.get_best_fitted_model(&data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fcm_grid_with_increments() {
        let config = Config::new(
            (2, Some(3), Some(1)),
            (2.0, Some(2.5), Some(0.1)),
            (100, None, None),
            (0.01, None, None),
        );
        let fcm_grid = FcmGrid::new(config);
        let data = array![[1.0], [2.0], [3.0], [4.0], [5.0]];

        let result = fcm_grid.get_best_fitted_model(&data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fcm_grid_no_valid_model() {
        let config = Config::new(
            (0, None, None),
            (0.0, None, None),
            (0, None, None),
            (0.0, None, None),
        );
        let fcm_grid = FcmGrid::new(config);
        let data = array![[1.0], [2.0], [3.0], [4.0], [5.0]];

        let result = fcm_grid.get_best_fitted_model(&data);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            Error::CreatingModel("Fuzziness factor must be greater than 1".to_string())
        );
    }
}
