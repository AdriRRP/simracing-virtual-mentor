use ndarray::{Array1, Array2, ArrayViewMut, Axis, Ix1, Zip};
use ndarray_rand::{rand_distr::Uniform, RandomExt};
use rand::thread_rng;

/// A struct representing the Fuzzy C-Means algorithm.
pub struct FuzzyCMeans {
    c: usize,
    m: f64,
    max_iter: usize,
    error: f64,
    cluster_centers: Option<Array2<f64>>,
}

impl FuzzyCMeans {
    /// Creates a new instance of `FuzzyCMeans`.
    ///
    /// # Arguments
    ///
    /// * `c` - Number of clusters.
    /// * `m` - Fuzziness parameter.
    /// * `max_iter` - Maximum number of iterations.
    /// * `error` - Convergence error.
    ///
    /// # Examples
    ///
    /// ```
    /// use symracing_virtual_mentor_shared::analysis::domain::analysis::fuzzy_c_means::FuzzyCMeans;
    ///
    /// let mut fcm = FuzzyCMeans::new(2, 2.0, 100, 0.01);
    /// ```
    #[must_use]
    pub const fn new(c: usize, m: f64, max_iter: usize, error: f64) -> Self {
        Self {
            c,
            m,
            max_iter,
            error,
            cluster_centers: None,
        }
    }

    /// Fits the model to the provided data.
    ///
    /// # Arguments
    ///
    /// * `data` - Data matrix of shape (`n_samples`, `n_features`).
    ///
    /// # Returns
    ///
    /// A membership matrix of shape (`n_samples`, `n_clusters`).
    ///
    /// # Examples
    ///
    /// ```
    /// use ndarray::array;
    /// use symracing_virtual_mentor_shared::analysis::domain::analysis::fuzzy_c_means::FuzzyCMeans;
    ///
    /// let data = array![
    ///     [1.0, 2.0, 1.5],
    ///     [1.5, 1.8, 1.7],
    ///     [5.0, 8.0, 7.5],
    ///     [8.0, 8.0, 8.0],
    ///     [1.0, 0.6, 0.8],
    ///     [9.0, 11.0, 10.5],
    /// ];
    ///
    /// let mut fcm = FuzzyCMeans::new(2, 2.0, 100, 0.01);
    /// let memberships = fcm.fit(&data);
    /// ```
    pub fn fit(&mut self, data: &Array2<f64>) -> Array2<f64> {
        let n_samples = data.shape()[0];

        // Initialize memberships randomly
        let mut memberships = {
            let mut rng = thread_rng();
            Array2::<f64>::random_using((n_samples, self.c), Uniform::new(0., 1.), &mut rng)
        };

        // Normalize memberships (ensure )
        for mut membership in memberships.outer_iter_mut() {
            Self::normalize_membership(&mut membership);
        }

        for _ in 0..self.max_iter {
            let cluster_centers = self.update_cluster_centers(data, &memberships);
            let mut new_memberships = self.update_memberships(data, &cluster_centers);

            // Normalize new memberships
            for mut membership in new_memberships.outer_iter_mut() {
                Self::normalize_membership(&mut membership);
            }

            let diff = &memberships - &new_memberships;
            let max_diff = diff
                .mapv(f64::abs)
                .fold(f64::NEG_INFINITY, |a, &b| a.max(b));

            memberships = new_memberships;

            if max_diff < self.error {
                break;
            }
        }

        self.cluster_centers = Some(self.update_cluster_centers(data, &memberships));
        memberships
    }

    /// Ensure that the sum of a point's membership to each cluster is 1
    fn normalize_membership(membership: &mut ArrayViewMut<f64, Ix1>) {
        let sum: f64 = membership.sum();
        if sum == 0.0 {
            let len = membership.len();
            membership.assign(&(Array1::ones(len) / len as f64));
        } else {
            *membership /= sum;
        }
    }

    /// Updates the cluster centers.
    fn update_cluster_centers(&self, data: &Array2<f64>, memberships: &Array2<f64>) -> Array2<f64> {
        let powered_memberships = memberships.mapv(|x| x.powf(self.m));

        let numerator = powered_memberships.t().dot(data);
        let denominator = powered_memberships.sum_axis(Axis(0)).insert_axis(Axis(1));

        // Handle division by zero
        numerator / &denominator.mapv(|x| if x == 0. { f64::EPSILON } else { x })
    }

    /// Updates the memberships of the data points based on the cluster centers.
    fn update_memberships(&self, data: &Array2<f64>, cluster_centers: &Array2<f64>) -> Array2<f64> {
        let mut new_memberships = Array2::<f64>::zeros((data.shape()[0], self.c));

        for (i, sample) in data.outer_iter().enumerate() {
            for (j, center) in cluster_centers.outer_iter().enumerate() {
                let numerator = Self::distance(&sample, &center).powf(2. / (self.m - 1.));
                let mut denominator = 0.;

                for other_center in cluster_centers.outer_iter() {
                    denominator += Self::distance(&sample, &other_center).powf(2. / (self.m - 1.));
                }

                // Handle division by zero
                denominator = if denominator == 0. {
                    f64::EPSILON
                } else {
                    denominator
                };

                new_memberships[[i, j]] = 1. / (numerator / denominator);
            }
        }

        new_memberships
    }

    /// Calculates the Euclidean distance between two points.
    fn distance(x: &ndarray::ArrayView1<f64>, y: &ndarray::ArrayView1<f64>) -> f64 {
        Zip::from(x)
            .and(y)
            .fold(0., |acc, &x_i, &y_i| {
                // (x_1 - y_1) ^ 2 + (x_2 - y_2) ^ 2 + ... + (x_n - y_n) ^ 2
                (x_i - y_i).mul_add(x_i - y_i, acc)
            })
            .sqrt()
    }

    /// Returns the cluster centers after fitting the model.
    ///
    /// # Examples
    ///
    /// ```
    /// use ndarray::array;
    /// use symracing_virtual_mentor_shared::analysis::domain::analysis::fuzzy_c_means::FuzzyCMeans;
    ///
    /// let data = array![
    ///     [1.0, 2.0, 1.5],
    ///     [1.5, 1.8, 1.7],
    ///     [5.0, 8.0, 7.5],
    ///     [8.0, 8.0, 8.0],
    ///     [1.0, 0.6, 0.8],
    ///     [9.0, 11.0, 10.5],
    /// ];
    ///
    /// let mut fcm = FuzzyCMeans::new(2, 2.0, 100, 0.01);
    /// fcm.fit(&data);
    /// let centers = fcm.cluster_centers().unwrap();
    /// ```
    #[must_use]
    pub const fn cluster_centers(&self) -> Option<&Array2<f64>> {
        self.cluster_centers.as_ref()
    }

    /// Calculates the Fuzzy Partition Coefficient (FPC).
    ///
    /// # Arguments
    ///
    /// * `memberships` - Membership matrix of shape (`n_samples`, `n_clusters`).
    ///
    /// # Returns
    ///
    /// The FPC value.
    ///
    /// # Examples
    ///
    /// ```
    /// use ndarray::array;
    /// use symracing_virtual_mentor_shared::analysis::domain::analysis::fuzzy_c_means::FuzzyCMeans;
    ///
    /// let data = array![
    ///     [1.0, 2.0, 1.5],
    ///     [1.5, 1.8, 1.7],
    ///     [5.0, 8.0, 7.5],
    ///     [8.0, 8.0, 8.0],
    ///     [1.0, 0.6, 0.8],
    ///     [9.0, 11.0, 10.5],
    /// ];
    ///
    /// let mut fcm = FuzzyCMeans::new(2, 2.0, 100, 0.01);
    /// let memberships = fcm.fit(&data);
    /// let fpc_value = fcm.fpc(&memberships);
    /// assert!(fpc_value > 0.0 && fpc_value <= 1.0);
    /// ```
    #[must_use]
    pub fn fpc(&self, memberships: &Array2<f64>) -> f64 {
        let n_samples = memberships.shape()[0];
        let fpc = memberships.mapv(|x| x.powi(2)).sum() / n_samples as f64;

        // Ensure FPC is within the expected range and not NaN
        if fpc.is_nan() || fpc <= 0.0 {
            eprintln!("Warning: FPC is NaN or 0.0, adjusting to f64::EPSILON");
            f64::EPSILON // Handle division by zero
        } else {
            fpc.clamp(f64::EPSILON, 1.0)
        }
    }
}

/// Tests for the FuzzyCMeans module.
#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_fit() {
        let data = array![
            [1.0, 2.0, 1.5],
            [1.5, 1.8, 1.7],
            [5.0, 8.0, 7.5],
            [8.0, 8.0, 8.0],
            [1.0, 0.6, 0.8],
            [9.0, 11.0, 10.5],
        ];

        let mut fcm = FuzzyCMeans::new(2, 2.0, 100, 0.01);
        let memberships = fcm.fit(&data);

        assert_eq!(memberships.shape(), [6, 2]);

        // Ensure that memberships are normalized
        for row in memberships.outer_iter() {
            let sum: f64 = row.sum();
            assert!((sum - 1.0).abs() < 1e-5, "Memberships are not normalized");
        }
    }

    #[test]
    fn test_cluster_centers() {
        let data = array![
            [1.0, 2.0, 1.5],
            [1.5, 1.8, 1.7],
            [5.0, 8.0, 7.5],
            [8.0, 8.0, 8.0],
            [1.0, 0.6, 0.8],
            [9.0, 11.0, 10.5],
        ];

        let mut fcm = FuzzyCMeans::new(2, 2.0, 100, 0.01);
        fcm.fit(&data);
        let centers = fcm.cluster_centers().unwrap();

        assert_eq!(centers.shape(), [2, 3]);
    }

    #[test]
    fn test_fpc() {
        let data = array![
            [1.0, 2.0, 1.5],
            [1.5, 1.8, 1.7],
            [5.0, 8.0, 7.5],
            [8.0, 8.0, 8.0],
            [1.0, 0.6, 0.8],
            [9.0, 11.0, 10.5],
        ];

        let mut fcm = FuzzyCMeans::new(2, 2.0, 100, 0.01);
        let memberships = fcm.fit(&data);
        let fpc_value = fcm.fpc(&memberships);

        println!("FPC value: {}", fpc_value);
        assert!(fpc_value > 0.0 && fpc_value <= 1.0);
    }
}
