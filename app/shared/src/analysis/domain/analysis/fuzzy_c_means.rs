use ndarray::{Array1, Array2};
use rand::prelude::*;

pub struct FuzzyCMeans {
    centroids: Array2<f64>,
    memberships: Array2<f64>,
    m: f64, // factor de fuzziness
}

impl FuzzyCMeans {
    #[must_use]
    pub fn new(num_clusters: usize, num_features: usize, fuzziness: f64) -> Self {
        Self {
            centroids: Array2::zeros((num_clusters, num_features)),
            memberships: Array2::zeros((num_clusters, num_features)),
            m: fuzziness,
        }
    }

    pub fn initialize_centroids(&mut self, data: &Array1<f64>, num_clusters: usize) {
        let mut rng = thread_rng();
        let n = data.len();
        for i in 0..num_clusters {
            let idx = rng.gen_range(0..n);
            self.centroids[[i, 0]] = data[idx];
        }
    }

    fn update_memberships(&mut self, data: &Array1<f64>) {
        let n = data.len();
        let c = self.centroids.shape()[0];
        for i in 0..n {
            for j in 0..c {
                let mut sum = 0.0;
                for k in 0..c {
                    let denominator = (data[i] - self.centroids[[k, 0]]).powi(2);
                    if denominator == 0.0 {
                        continue;
                    }
                    let ratio = ((data[i] - self.centroids[[j, 0]]).powi(2) / denominator)
                        .powf(1.0 / (self.m - 1.0));
                    sum += ratio;
                }
                if sum == 0.0 {
                    sum = 1.0; // Evitar división por cero
                }
                self.memberships[[j, i]] = 1.0 / sum;
            }
        }
    }

    fn update_centroids(&mut self, data: &Array1<f64>) {
        let n = data.len();
        let c = self.centroids.shape()[0];
        for j in 0..c {
            let mut numerator = 0.0;
            let mut denominator = 0.0;
            for i in 0..n {
                let u_ij_m = self.memberships[[j, i]].powf(self.m);
                numerator += u_ij_m * data[i];
                denominator += u_ij_m;
            }
            if denominator == 0.0 {
                denominator = 1.0; // Evitar división por cero
            }
            self.centroids[[j, 0]] = numerator / denominator;
        }
    }

    pub fn fit(&mut self, data: &Array1<f64>, max_iter: usize, tolerance: f64) {
        let n = data.len();
        let c = self.centroids.shape()[0];
        self.memberships = Array2::zeros((c, n));

        for _ in 0..max_iter {
            let old_centroids = self.centroids.clone();
            self.update_memberships(data);
            self.update_centroids(data);
            let max_change = (&self.centroids - &old_centroids)
                .mapv(f64::abs)
                .fold(0.0, |a: f64, &b| a.max(b));
            if max_change < tolerance {
                break;
            }
        }
    }

    #[must_use]
    pub fn predict(&self) -> Vec<Vec<f64>> {
        let n = self.memberships.shape()[1];
        let c = self.centroids.shape()[0];
        let mut memberships = Vec::with_capacity(n);

        for i in 0..n {
            let mut point_memberships = Vec::with_capacity(c);
            for j in 0..c {
                point_memberships.push(self.memberships[[j, i]]);
            }
            memberships.push(point_memberships);
        }

        memberships
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_fuzzy_cmeans_basic() {
        let data = array![1.0, 2.0, 1.5, 8.0, 9.0, 8.5];
        let mut fcm = FuzzyCMeans::new(2, 1, 2.0);
        fcm.initialize_centroids(&data, 2);
        fcm.fit(&data, 100, 1e-5);

        let memberships = fcm.predict();
        assert_eq!(memberships.len(), 6);

        for m in memberships {
            assert_eq!(m.len(), 2);
            assert!(m.iter().all(|&x| x >= 0.0 && x <= 1.0));
        }

        let centroids = fcm.centroids;
        assert_eq!(centroids.shape()[0], 2);
        assert_eq!(centroids.shape()[1], 1);
    }

    #[test]
    fn test_fuzzy_cmeans_three_clusters() {
        let data = array![1.0, 2.0, 1.5, 8.0, 9.0, 8.5, 20.0];
        let mut fcm = FuzzyCMeans::new(3, 1, 2.0);
        fcm.initialize_centroids(&data, 3);
        fcm.fit(&data, 100, 1e-5);

        let memberships = fcm.predict();
        assert_eq!(memberships.len(), 7);

        for m in memberships {
            assert_eq!(m.len(), 3);
            assert!(m.iter().all(|&x| x >= 0.0 && x <= 1.0));
        }

        let centroids = fcm.centroids;
        assert_eq!(centroids.shape()[0], 3);
        assert_eq!(centroids.shape()[1], 1);
    }

    #[test]
    fn test_fuzzy_cmeans_high_fuzziness() {
        let data = array![1.0, 2.0, 1.5, 8.0, 9.0, 8.5];
        let mut fcm = FuzzyCMeans::new(2, 1, 5.0);
        fcm.initialize_centroids(&data, 2);
        fcm.fit(&data, 100, 1e-5);

        let memberships = fcm.predict();
        assert_eq!(memberships.len(), 6);

        for m in memberships {
            assert_eq!(m.len(), 2);
            assert!(m.iter().all(|&x| x >= 0.0 && x <= 1.0));
        }

        let centroids = fcm.centroids;
        assert_eq!(centroids.shape()[0], 2);
        assert_eq!(centroids.shape()[1], 1);
    }

    #[test]
    fn test_fuzzy_cmeans_low_tolerance() {
        let data = array![1.0, 2.0, 1.5, 8.0, 9.0, 8.5];
        let mut fcm = FuzzyCMeans::new(2, 1, 2.0);
        fcm.initialize_centroids(&data, 2);
        fcm.fit(&data, 100, 1e-10);

        let memberships = fcm.predict();
        assert_eq!(memberships.len(), 6);

        for m in memberships {
            assert_eq!(m.len(), 2);
            assert!(m.iter().all(|&x| x >= 0.0 && x <= 1.0));
        }

        let centroids = fcm.centroids;
        assert_eq!(centroids.shape()[0], 2);
        assert_eq!(centroids.shape()[1], 1);
    }

    #[test]
    fn test_fuzzy_cmeans_large_data_set() {
        let data = array![1.0, 2.0, 1.5, 8.0, 9.0, 8.5, 15.0, 16.0, 20.0, 22.0];
        let mut fcm = FuzzyCMeans::new(3, 1, 2.0);
        fcm.initialize_centroids(&data, 3);
        fcm.fit(&data, 100, 1e-5);

        let memberships = fcm.predict();
        assert_eq!(memberships.len(), 10);

        for m in memberships {
            assert_eq!(m.len(), 3);
            assert!(m.iter().all(|&x| x >= 0.0 && x <= 1.0));
        }

        let centroids = fcm.centroids;
        assert_eq!(centroids.shape()[0], 3);
        assert_eq!(centroids.shape()[1], 1);
    }
}
