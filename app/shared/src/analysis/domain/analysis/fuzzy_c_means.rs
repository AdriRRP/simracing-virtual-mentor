use ndarray::{Array, Array1, Array2, ArrayView, Axis, Ix1, Zip};
use ndarray_rand::{rand_distr::Uniform, RandomExt};
use rand::thread_rng;

#[derive(Clone, Debug)]
pub struct FittedModel {
    centroids: Array2<f64>,
    memberships: Array2<f64>,
    fpc: f64,
}

impl FittedModel {
    #[must_use]
    pub fn new(centroids: Array2<f64>, memberships: Array2<f64>) -> Self {
        let n_samples = memberships.shape()[0];
        let fpc = {
            #[allow(clippy::cast_precision_loss)]
            let fpc = memberships.mapv(|x| x.powi(2)).sum() / n_samples as f64;
            // Ensure FPC is within the expected range and not NaN
            if fpc.is_nan() || fpc <= 0.0 {
                eprintln!("Warning: FPC is NaN or 0.0, adjusting to f64::EPSILON");
                f64::EPSILON // Handle division by zero
            } else {
                fpc.clamp(f64::EPSILON, 1.0)
            }
        };

        Self {
            centroids,
            memberships,
            fpc,
        }
    }

    #[must_use]
    pub const fn centroids(&self) -> &Array2<f64> {
        &self.centroids
    }

    #[must_use]
    pub const fn memberships(&self) -> &Array2<f64> {
        &self.memberships
    }

    #[must_use]
    pub const fn fpc(&self) -> f64 {
        self.fpc
    }
}

/// A struct representing the Fuzzy C-Means algorithm.
pub struct FuzzyCMeans {
    c: usize,
    m: f64,
    max_iter: usize,
    error: f64,
}

impl FuzzyCMeans {
    /// Attempts to create a new instance of `FuzzyCMeans`.
    ///
    /// This constructor returns a `Result` to handle cases where the fuzziness
    /// parameter `m` is invalid. Specifically, `m` must be greater than 1;
    /// otherwise, the function returns an error.
    ///
    /// # Arguments
    ///
    /// * `c` - Number of clusters.
    /// * `m` - Fuzziness parameter. Must be greater than 1.
    /// * `max_iter` - Maximum number of iterations.
    /// * `error` - Convergence error.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Self)` if the provided `m` is valid (greater than 1).
    /// Otherwise, returns `Err(Error::FuzzinessFactorLowerOrEqualsThanOne)`.
    ///
    /// # Errors
    ///
    /// This function returns an `Error::FuzzinessFactorLowerOrEqualsThanOne` if `m`
    /// is less than or equal to 1, as this would make the fuzzy clustering algorithm
    /// mathematically invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use symracing_virtual_mentor_shared::analysis::domain::analysis::fuzzy_c_means::FuzzyCMeans;
    ///
    /// // This will succeed because m > 1
    /// let mut fcm = FuzzyCMeans::try_new(2, 2.0, 100, 0.01);
    ///
    /// // This will fail because m <= 1
    /// let result = FuzzyCMeans::try_new(2, 1.0, 100, 0.01);
    /// assert!(result.is_err());
    /// ```
    pub fn try_new(c: usize, m: f64, max_iter: usize, error: f64) -> Result<Self, Error> {
        if m <= 1. {
            return Err(Error::FuzzinessFactorLowerOrEqualsThanOne);
        }

        Ok(Self {
            c,
            m,
            max_iter,
            error,
        })
    }

    /// Fits the Fuzzy C-Means model to the provided data by generating an initial random
    /// membership matrix.
    ///
    /// This function initializes the clustering process by generating a random membership
    /// matrix and then calling `try_fit_with_memberships` to iteratively update the cluster
    /// centers (centroids) and the membership matrix until the algorithm converges or the
    /// maximum number of iterations is reached.
    ///
    /// # Arguments
    ///
    /// * `data` - A 2D array containing the data points to be clustered. Each row represents
    ///            a data point, and each column represents a feature.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a `FittedModel` if the fitting process is successful,
    /// or an `Error` if an unexpected issue occurs during the fitting process. Since the
    /// membership matrix is generated to be compatible with the data, errors related to
    /// incompatible dimensions should not occur.
    ///
    /// # Errors
    ///
    /// While this function should not encounter dimension-related errors (due to the
    /// guaranteed compatibility of the membership matrix), it may propagate other errors
    /// from `try_fit_with_memberships`, although such cases are unlikely given correct
    /// implementation of the model.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ndarray::array;
    /// use symracing_virtual_mentor_shared::analysis::domain::analysis::fuzzy_c_means::FuzzyCMeans;
    ///
    /// let data = array![[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]];
    /// let fcm = FuzzyCMeans::try_new(2, 1.7, 100, 1e-4).unwrap();
    ///
    /// match fcm.try_fit(&data) {
    ///     Ok(fitted_model) => println!("Model fitted successfully!"),
    ///     Err(e) => println!("Error fitting model: {:?}", e),
    /// }
    /// ```
    pub fn try_fit(&self, data: &Array2<f64>) -> Result<FittedModel, Error> {
        let num_samples = data.shape()[0];
        let mut initial_memberships = self.generate_random_memberships(num_samples);
        self.try_fit_with_memberships(data, &mut initial_memberships)
    }

    /// Fits the Fuzzy C-Means model to the provided data using the initial membership matrix.
    ///
    /// This function iteratively updates the cluster centers (centroids) and the membership
    /// matrix until the algorithm converges or the maximum number of iterations is reached.
    ///
    /// # Arguments
    ///
    /// * `data` - A 2D array containing the data points to be clustered. Each row represents
    ///            a data point, and each column represents a feature.
    /// * `memberships` - A mutable reference to a 2D array representing the membership matrix,
    ///                   where each entry denotes the degree of membership of a data point
    ///                   to a particular cluster. The number of rows should match the number
    ///                   of data points in `data`.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a `FittedModel` if the fitting process is successful, or
    /// an `Error` if the membership matrix and data dimensions are incompatible.
    ///
    /// # Errors
    ///
    /// This function will return an `Error::IncompatibleMatrixMultiplication` if the number
    /// of rows in the membership matrix does not match the number of rows in the data matrix.
    /// This error occurs when the shapes of the membership matrix and the data matrix are
    /// not compatible for the clustering process.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ndarray::array;
    /// use symracing_virtual_mentor_shared::analysis::domain::analysis::fuzzy_c_means::FuzzyCMeans;
    ///
    /// let data = array![[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]];
    /// let mut memberships = array![[0.5, 0.5], [0.3, 0.7], [0.2, 0.8]];
    /// let fcm = FuzzyCMeans::try_new(2, 1.7, 100, 1e-4).unwrap();
    ///
    /// match fcm.try_fit_with_memberships(&data, &mut memberships) {
    ///     Ok(fitted_model) => println!("Model fitted successfully!"),
    ///     Err(e) => println!("Error fitting model: {:?}", e),
    /// }
    /// ```
    pub fn try_fit_with_memberships(
        &self,
        data: &Array2<f64>,
        memberships: &mut Array2<f64>,
    ) -> Result<FittedModel, Error> {
        let ((memberships_rows, memberships_cols), (data_rows, data_cols)) =
            (memberships.dim(), data.dim());
        if memberships_rows != data_rows {
            return Err(Error::IncompatibleMatrixMultiplication(
                format!("membership shape {memberships_rows} x {memberships_cols}"),
                format!("data shape {data_rows} x {data_cols}"),
            ));
        }

        let mut centroids: Array2<f64> = Array::<f64, _>::zeros((self.c, data.shape()[0]));

        for _ in 0..self.max_iter {
            centroids = self.update_cluster_centers(data, memberships);
            let new_memberships = self.update_memberships(data, &centroids);

            let diff = &*memberships - &new_memberships;
            let max_diff = diff
                .mapv(f64::abs)
                .fold(f64::NEG_INFINITY, |a, &b| a.max(b));

            *memberships = new_memberships;

            if max_diff < self.error {
                break;
            }
        }

        Ok(FittedModel::new(centroids, memberships.to_owned()))
    }

    fn generate_random_memberships(&self, num_samples: usize) -> Array2<f64> {
        // Initialize memberships randomly
        let mut memberships = {
            let mut rng = thread_rng();
            Array2::<f64>::random_using((num_samples, self.c), Uniform::new(0., 1.), &mut rng)
        };

        // Normalize memberships
        for mut membership in memberships.outer_iter_mut() {
            let sum: f64 = membership.sum();
            if sum == 0.0 {
                let len = membership.len();
                #[allow(clippy::cast_precision_loss)]
                membership.assign(&(Array1::ones(len) / len as f64));
            } else {
                membership /= sum;
            }
        }

        memberships
    }

    fn update_cluster_centers(&self, data: &Array2<f64>, memberships: &Array2<f64>) -> Array2<f64> {
        let powered_memberships = memberships.mapv(|x| x.powf(self.m));

        let numerator = powered_memberships.t().dot(data);
        let denominator = powered_memberships.sum_axis(Axis(0)).insert_axis(Axis(1));

        // Handle division by zero
        numerator / &denominator.mapv(|x| if x == 0. { f64::EPSILON } else { x })
    }

    fn update_memberships(&self, data: &Array2<f64>, centroids: &Array2<f64>) -> Array2<f64> {
        let num_samples = data.shape()[0];
        let mut new_memberships = Array2::<f64>::zeros((num_samples, self.c));

        for (i, sample) in data.axis_iter(Axis(0)).enumerate() {
            for (j, centroid) in centroids.axis_iter(Axis(0)).enumerate() {
                let numerator = Self::distance(&sample, &centroid); // d_ij
                let u_ij: f64 = 1.
                    / (centroids
                        .axis_iter(Axis(0))
                        .map(|k| {
                            let denominator = Self::distance(&sample, &k); // d_ik
                            let denominator = if denominator == 0. {
                                f64::EPSILON
                            } else {
                                denominator
                            };
                            (numerator / denominator).powf(2. / (self.m - 1.))
                        })
                        .sum::<f64>());
                new_memberships[[i, j]] = u_ij;
            }
        }

        new_memberships
    }

    #[must_use]
    pub fn distance(x: &ArrayView<f64, Ix1>, y: &ArrayView<f64, Ix1>) -> f64 {
        Zip::from(x)
            .and(y)
            .fold(0., |acc, &x_i, &y_i| {
                // (x_1 - y_1) ^ 2 + (x_2 - y_2) ^ 2 + ... + (x_n - y_n) ^ 2
                (x_i - y_i).mul_add(x_i - y_i, acc)
            })
            .sqrt()
    }
}

#[derive(PartialEq, Eq, Debug, thiserror::Error)]
pub enum Error {
    #[error("Fuzziness factor must be greater than 1")]
    FuzzinessFactorLowerOrEqualsThanOne,
    #[error("{0} cannot be multiplied by {1}")]
    IncompatibleMatrixMultiplication(String, String),
}

/// Tests for the FuzzyCMeans module.
#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    /// Define the tolerance for floating-point comparisons.
    const EPSILON: f64 = 1e-6;

    /// Helper function to compare two arrays of f64 with a given tolerance.
    fn assert_array_eq(expected: &Array2<f64>, result: &Array2<f64>, epsilon: f64) {
        let (expected_rows, expected_cols) = expected.dim();
        let (result_rows, result_cols) = result.dim();

        assert_eq!(
            (expected_rows, expected_cols),
            (result_rows, result_cols),
            "Array dimensions do not match: expected ({}, {}), got ({}, {})",
            expected_rows,
            expected_cols,
            result_rows,
            result_cols
        );

        for r in 0..expected_rows {
            for c in 0..expected_cols {
                assert!(
                    (expected[[r, c]] - result[[r, c]]).abs() < epsilon,
                    "Element at ({}, {}) differs by more than epsilon {}: expected {}, got {}",
                    r,
                    c,
                    epsilon,
                    expected[[r, c]],
                    result[[r, c]]
                );
            }
        }
    }

    /// Tests FuzzyCMeans with one-dimensional data and verifies centroids, memberships, and FPC.
    #[test]
    fn first_iter_on_one_dim_data() {
        let c = 2;
        let m = 1.7;
        let max_iter = 1;
        let error = 0.01;

        let fcm = FuzzyCMeans::try_new(c, m, max_iter, error)
            .unwrap_or_else(|e| panic!("Test Failed: {e}"));

        let data = array![[1.], [2.], [4.], [7.]];
        let mut initial_memberships = array![[0.8, 0.2], [0.7, 0.3], [0.2, 0.8], [0.1, 0.9],];

        let result = fcm
            .try_fit_with_memberships(&data, &mut initial_memberships)
            .unwrap_or_else(|e| panic!("Test Failed: {e}"));

        let expected_centroids = array![[1.6539221306009797], [5.198884159924932]];
        assert_array_eq(&expected_centroids, result.centroids(), EPSILON);

        let expected_memberships = array![
            [0.9950975335864382, 0.004902466413561808],
            [0.9982632153654964, 0.0017367846345036414],
            [0.12806762946010766, 0.8719323705398923],
            [0.042760125204558706, 0.9572398747954414]
        ];
        assert_array_eq(&expected_memberships, result.memberships(), EPSILON);

        let expected_fpc = 0.920394895454056;
        assert!(
            (result.fpc() - expected_fpc).abs() < EPSILON,
            "FPC value {} differs from expected {} beyond epsilon {}",
            result.fpc(),
            expected_fpc,
            EPSILON
        );
    }

    /// Tests FuzzyCMeans with two-dimensional data and verifies centroids, memberships, and FPC.
    #[test]
    fn first_iter_on_two_dim_data() {
        let c = 2;
        let m = 1.7;
        let max_iter = 1;
        let error = 0.01;

        let fcm = FuzzyCMeans::try_new(c, m, max_iter, error)
            .unwrap_or_else(|e| panic!("Test Failed: {e}"));

        let data = array![[1., 3.], [2., 5.], [4., 8.], [7., 9.]];
        let mut initial_memberships = array![[0.8, 0.2], [0.7, 0.3], [0.2, 0.8], [0.1, 0.9],];

        let result = fcm
            .try_fit_with_memberships(&data, &mut initial_memberships)
            .unwrap_or_else(|e| panic!("Test Failed: {e}"));

        let expected_centroids = array![
            [1.6539221306009797, 4.167447085460862],
            [5.198884159924932, 8.072577522696557]
        ];
        assert_array_eq(&expected_centroids, result.centroids(), EPSILON);

        let expected_memberships = array![
            [0.9895736243443243, 0.010426375655675672],
            [0.9895640701351411, 0.01043592986485885],
            [0.022536642797307747, 0.9774633572026923],
            [0.025939954747919117, 0.9740600452520809]
        ];
        assert_array_eq(&expected_memberships, result.memberships(), EPSILON);

        let expected_fpc = 0.9660297481982968;
        assert!(
            (result.fpc() - expected_fpc).abs() < EPSILON,
            "FPC value {} differs from expected {} beyond epsilon {}",
            result.fpc(),
            expected_fpc,
            EPSILON
        );
    }

    /// Tests FuzzyCMeans behavior with incompatible membership shape.
    #[test]
    fn incompatible_shape() {
        let c = 2;
        let m = 1.7;
        let max_iter = 1;
        let error = 0.01;

        let fcm = FuzzyCMeans::try_new(c, m, max_iter, error)
            .unwrap_or_else(|e| panic!("Test Failed: {e}"));

        let data = array![[1.], [2.], [4.], [7.]];
        let mut initial_memberships = array![[0.8, 0.2, 0.7, 0.3], [0.2, 0.8, 0.1, 0.9],];

        let expected_error = Error::IncompatibleMatrixMultiplication(
            "membership shape 2 x 4".to_string(),
            "data shape 4 x 1".to_string(),
        );

        let result = fcm.try_fit_with_memberships(&data, &mut initial_memberships);

        // Verify that the error matches the expected error
        assert!(result.is_err_and(|e| e == expected_error));
    }
}
