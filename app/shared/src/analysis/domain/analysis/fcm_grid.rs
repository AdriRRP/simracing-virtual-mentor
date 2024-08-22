use crate::analysis::domain::analysis::fuzzy_c_means::{FittedModel, FuzzyCMeans};
use ndarray::Array2;

pub struct FcmGrid {
    c: (usize, usize, usize),
    m: (f64, f64, f64),
    max_iter: (usize, usize, usize),
    error: (f64, f64, f64),
}

impl FcmGrid {
    /// Creates a new instance of `FcmGrid` with the specified parameter ranges.
    ///
    /// # Arguments
    ///
    /// * `c` - Tuple containing the minimum, maximum, and increment values for the number of clusters.
    /// * `m` - Tuple containing the minimum, maximum, and increment values for the fuzziness parameter.
    /// * `max_iter` - Tuple containing the minimum, maximum, and increment values for the maximum number of iterations.
    /// * `error` - Tuple containing the minimum, maximum, and increment values for the convergence error.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::analysis::domain::analysis::fuzzy_c_means::FcmGrid;
    ///
    /// let grid = FcmGrid::new((2, 5, 1), (1.5, 2.5, 0.5), (50, 200, 50), (0.01, 0.1, 0.01));
    /// ```
    pub const fn new(
        c: (usize, usize, usize),
        m: (f64, f64, f64),
        max_iter: (usize, usize, usize),
        error: (f64, f64, f64),
    ) -> Self {
        Self {
            c,
            m,
            max_iter,
            error,
        }
    }

    /// Performs a grid search to find the best Fuzzy C-Means (FCM) model based on the provided parameter ranges.
    ///
    /// This method iterates over all combinations of parameters defined in the `FcmGrid` and selects the model
    /// with the best evaluation score (i.e., the lowest value of the objective function).
    ///
    /// # Arguments
    ///
    /// * `data` - The input data to be used for fitting the FCM model.
    ///
    /// # Returns
    ///
    /// * `Ok(FittedModel)` - The best fitted model found during the grid search.
    /// * `Err(Error)` - An error if no valid model is found or if there is a failure in model creation or fitting.
    ///
    /// # Examples
    ///
    /// ```
    /// use ndarray::Array2;
    /// use crate::analysis::domain::analysis::fuzzy_c_means::{FcmGrid, FittedModel, Error};
    ///
    /// let data = Array2::<f64>::zeros((100, 10)); // Example data
    /// let grid = FcmGrid::new((2, 5, 1), (1.5, 2.5, 0.5), (50, 200, 50), (0.01, 0.1, 0.01));
    /// match grid.get_best_fitted_model(&data) {
    ///     Ok(model) => println!("Best model found: {:?}", model),
    ///     Err(e) => eprintln!("Error: {:?}", e),
    /// }
    /// ```
    pub fn get_best_fitted_model(&self, data: &Array2<f64>) -> Result<FittedModel, Error> {
        let mut best_model: Option<FittedModel> = None;

        let (mut c, c_max, c_inc) = self.c;
        let (mut m, m_max, m_inc) = self.m;
        let (mut max_iter, max_iter_max, max_iter_inc) = self.max_iter;
        let (mut error, error_max, error_inc) = self.error;

        // Iterate over all combinations of parameters
        while c <= c_max {
            while m <= m_max {
                while max_iter <= max_iter_max {
                    while error <= error_max {
                        // Attempt to create and fit the Fuzzy C-Means model
                        let model_result = FuzzyCMeans::try_new(c, m, max_iter, error)
                            .map_err(|e| Error::CreatingModel(e.to_string()))?
                            .try_fit(data)
                            .map_err(|e| Error::FittingModel(e.to_string()))?;

                        if best_model.is_none()
                            || best_model
                                .clone()
                                .is_some_and(|m| m.fpc() < model_result.fpc())
                        {
                            best_model = Some(model_result);
                        }

                        error += error_inc;
                    }
                    max_iter += max_iter_inc;
                }
                m += m_inc;
            }
            c += c_inc;
        }

        // Return the best model found or an error if none was valid
        best_model.ok_or(Error::NoValidModelFound)
    }
}

#[derive(PartialEq, Eq, Debug, thiserror::Error)]
pub enum Error {
    #[error("Error creating model: {0}")]
    CreatingModel(String),
    #[error("Error fitting model: {0}")]
    FittingModel(String),
    #[error("No valid model found")]
    NoValidModelFound,
}
