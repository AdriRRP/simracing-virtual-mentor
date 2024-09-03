use crate::analysis::domain::analysis::fuzzy_c_means::FittedModel;
use crate::analysis::domain::analysis::{tag::Tag, tags::Tags};

use ndarray::{Array1, Array2};
use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub struct TagAssigner {
    pub differences: Array2<f64>,
    pub centroids: Array2<f64>,
    pub memberships: Array2<f64>,
    pub threshold: f64,
}

impl TagAssigner {
    #[must_use]
    pub fn new(differences: Array2<f64>, fitted_model: &FittedModel, threshold: f64) -> Self {
        Self {
            differences,
            centroids: fitted_model.centroids().clone(),
            memberships: fitted_model.memberships().clone(),
            threshold,
        }
    }

    #[must_use]
    pub fn assign(&self) -> Tags {
        let sorted_centroids_with_index = self.sort_centroids_with_index();

        let mut sorted_centroid_tags_with_index =
            self.define_cluster_tags_with_index(&sorted_centroids_with_index);

        sorted_centroid_tags_with_index
            .sort_by(|(idx_1, _), (idx_2, _)| idx_1.partial_cmp(idx_2).unwrap_or(Ordering::Equal));

        let centroid_tags: Vec<Tag> = sorted_centroid_tags_with_index
            .into_iter()
            .map(|(_, t)| t)
            .collect();

        self.assign_final_tags(&centroid_tags)
    }

    // Return sorted centroids with its original index
    pub(crate) fn sort_centroids_with_index(&self) -> Vec<(usize, Array1<f64>)> {
        let mut indexed_centroids: Vec<(usize, Array1<f64>)> = self
            .centroids
            .outer_iter() // Iterate over rows of Array2
            .enumerate() // Add row index
            .map(|(i, row)| (i, row.to_owned())) // Turns every row into Array1 and adds its index
            .collect();

        indexed_centroids
            .sort_by(|(_, a), (_, b)| a[0].partial_cmp(&b[0]).unwrap_or(Ordering::Equal));

        indexed_centroids
    }

    fn norm_l2(a: &Array1<f64>) -> f64 {
        a.iter().map(|x| x.powi(2)).sum::<f64>().sqrt()
    }

    #[allow(clippy::cast_possible_truncation)]
    pub(crate) fn define_cluster_tags_with_index(
        &self,
        sorted_indexed_centroids: &[(usize, Array1<f64>)],
    ) -> Vec<(usize, Tag)> {
        let num_of_centroids = sorted_indexed_centroids.len();
        let tags: Vec<(usize, Tag)> = Vec::new();

        // Each individual element is greater than 0
        if self.differences.iter().all(|&diff| diff > 0.0) {
            sorted_indexed_centroids.iter().enumerate().fold(
                tags,
                |mut acc, (idx, (original_idx, _))| {
                    acc.push((*original_idx, Tag::increase_with_level(idx as u8)));
                    acc
                },
            )
        } else if self.differences.iter().all(|&diff| diff < 0.0) {
            sorted_indexed_centroids.iter().enumerate().fold(
                tags,
                |mut acc, (idx, (original_idx, _))| {
                    let level = num_of_centroids - 1 - idx;
                    acc.push((*original_idx, Tag::reduce_with_level(level as u8)));
                    acc
                },
            )
        } else {
            // Distances from origin to each centroid
            let from_origin_distances: Vec<f64> = sorted_indexed_centroids
                .iter()
                .map(|(_, centroid)| Self::norm_l2(centroid))
                .collect();

            // Optional index to position with stay tag
            let stay_idx = from_origin_distances
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
                .map(|(idx, _)| idx);

            if let Some(stay_idx) = stay_idx {
                // Add new index to be able to compare with stay_idx
                sorted_indexed_centroids.iter().enumerate().fold(
                    tags,
                    |mut acc, (idx, (original_idx, _))| {
                        match idx.cmp(&stay_idx) {
                            Ordering::Less => {
                                let level = stay_idx - 1 - idx;
                                acc.push((*original_idx, Tag::reduce_with_level(level as u8)));
                            }
                            Ordering::Greater => {
                                let level = idx - (stay_idx + 1);
                                acc.push((*original_idx, Tag::increase_with_level(level as u8)));
                            }
                            Ordering::Equal => {
                                acc.push((*original_idx, Tag::stay()));
                            }
                        }
                        acc
                    },
                )
            } else {
                tags // Empty tags in no keep_index found
            }
        }
    }

    fn assign_final_tags(&self, centroid_tags: &[Tag]) -> Tags {
        let mut final_tags: Vec<Tag> = Vec::new();

        for i in 0..self.differences.nrows() {
            let mut indexed_difference_memberships: Vec<(usize, f64)> = self
                .memberships
                .row(i)
                .to_vec()
                .into_iter()
                .enumerate()
                .collect();

            indexed_difference_memberships
                .sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal));

            // TODO: Better way to unwrap as Tag::stay() when option is none
            let mut final_tag = indexed_difference_memberships
                .pop()
                .map_or(Tag::stay(), |(i, _)| centroid_tags[i]);

            if let Some((i, membership)) = indexed_difference_memberships.pop() {
                if membership >= self.threshold {
                    let secondary_tag = centroid_tags[i];
                    final_tag.add_tendency(&secondary_tag);
                }
            }
            final_tags.push(final_tag);
        }

        Tags::from(final_tags)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::domain::analysis::tag::Base;
    use crate::analysis::domain::analysis::tag_assigner::Tag;
    use ndarray::array;

    #[test]
    fn test_assign_custom_centroids() {
        // Define centroids manually
        let centroids = array![
            [-30.0], // Should assign reduce(1)
            [-10.0], // Should assign reduce(0)
            [0.0],   // Should assign stay
            [10.0],  // Should assign increase(0)
            [20.0],  // Should assign increase(1)
        ];

        // Differences distributed between -40 and 40
        let differences = array![
            [-40.0],
            [-35.0],
            [-25.0],
            [-15.0],
            [-5.0],
            [0.0],
            [5.0],
            [15.0],
            [25.0],
            [35.0]
        ];

        // Force memberships to ensure each point is assigned to the nearest centroid
        let memberships = array![
            [1.0, 0.0, 0.0, 0.0, 0.0], // Closest to -30.0
            [1.0, 0.0, 0.0, 0.0, 0.0], // Closest to -30.0
            [0.0, 1.0, 0.0, 0.0, 0.0], // Closest to -10.0
            [0.0, 1.0, 0.0, 0.0, 0.0], // Closest to -10.0
            [0.0, 0.0, 1.0, 0.0, 0.0], // Closest to 0.0
            [0.0, 0.0, 1.0, 0.0, 0.0], // Exactly at 0.0
            [0.0, 0.0, 0.0, 1.0, 0.0], // Closest to 10.0
            [0.0, 0.0, 0.0, 1.0, 0.0], // Closest to 10.0
            [0.0, 0.0, 0.0, 0.0, 1.0], // Closest to 20.0
            [0.0, 0.0, 0.0, 0.0, 1.0], // Closest to 20.0
        ];

        // Create a FittedModel manually with the centroids and memberships
        let fitted_model = FittedModel::new(centroids.clone(), memberships.clone());

        // Create the TagAssigner with the differences and the fitted model
        let tag_assigner = TagAssigner::new(differences.clone(), &fitted_model, 0.5);

        // Assign the final tags to the difference points
        let assigned_tags = tag_assigner.assign();

        // Define the expected tags for each difference
        let expected_tags = vec![
            Tag::reduce_with_level(1),
            Tag::reduce_with_level(1), // -40, -35 -> reduce(1)
            Tag::reduce(),
            Tag::reduce(), // -25, -15 -> reduce(0)
            Tag::stay(),
            Tag::stay(), // -5, 0 -> stay
            Tag::increase(),
            Tag::increase(), // 5, 15 -> increase(0)
            Tag::increase_with_level(1),
            Tag::increase_with_level(1), // 25, 35 -> increase(1)
        ];

        // Ensure the assigned tags match the expected tags
        assert_eq!(assigned_tags, Tags::from(expected_tags));
    }

    #[test]
    fn test_assign_custom_centroids_with_reordered_centroids() {
        // Define centroids manually, but in a different order
        let centroids = array![
            [20.0],  // Should assign increase(1)
            [-10.0], // Should assign reduce(0)
            [10.0],  // Should assign increase(0)
            [0.0],   // Should assign stay
            [-30.0], // Should assign reduce(1)
        ];

        // Differences distributed between -40 and 40
        let differences = array![
            [-40.0],
            [-35.0],
            [-25.0],
            [-15.0],
            [-5.0],
            [0.0],
            [5.0],
            [15.0],
            [25.0],
            [35.0]
        ];

        // Force memberships to ensure each point is assigned to the nearest centroid
        let memberships = array![
            [0.0, 0.0, 0.0, 0.0, 1.0], // Closest to -30.0
            [0.0, 0.0, 0.0, 0.0, 1.0], // Closest to -30.0
            [0.0, 1.0, 0.0, 0.0, 0.0], // Closest to -10.0
            [0.0, 1.0, 0.0, 0.0, 0.0], // Closest to -10.0
            [0.0, 0.0, 0.0, 1.0, 0.0], // Closest to 0.0
            [0.0, 0.0, 0.0, 1.0, 0.0], // Exactly at 0.0
            [0.0, 0.0, 1.0, 0.0, 0.0], // Closest to 10.0
            [0.0, 0.0, 1.0, 0.0, 0.0], // Closest to 10.0
            [1.0, 0.0, 0.0, 0.0, 0.0], // Closest to 20.0
            [1.0, 0.0, 0.0, 0.0, 0.0], // Closest to 20.0
        ];

        // Create a FittedModel manually with the centroids and memberships
        let fitted_model = FittedModel::new(centroids.clone(), memberships.clone());

        // Create the TagAssigner with the differences and the fitted model
        let tag_assigner = TagAssigner::new(differences.clone(), &fitted_model, 0.5);

        // Assign the final tags to the difference points
        let assigned_tags = tag_assigner.assign();

        // Define the expected tags for each difference
        let expected_tags = vec![
            Tag::reduce_with_level(1),
            Tag::reduce_with_level(1), // -40, -35 -> reduce(1)
            Tag::reduce(),
            Tag::reduce(), // -25, -15 -> reduce(0)
            Tag::stay(),
            Tag::stay(), // -5, 0 -> stay
            Tag::increase(),
            Tag::increase(), // 5, 15 -> increase(0)
            Tag::increase_with_level(1),
            Tag::increase_with_level(1), // 25, 35 -> increase(1)
        ];

        // Ensure the assigned tags match the expected tags
        assert_eq!(assigned_tags, Tags::from(expected_tags));
    }

    #[test]
    fn test_assign_all_positive_differences() {
        // Define centroids manually
        let centroids = array![
            [5.0],  // increase(0)
            [10.0], // increase(1)
            [15.0], // increase(2)
        ];

        // All differences are positive
        let differences = array![[6.0], [8.0], [11.0], [14.0], [17.0],];

        // Force memberships
        let memberships = array![
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ];

        // Create a FittedModel manually with the centroids and memberships
        let fitted_model = FittedModel::new(centroids.clone(), memberships.clone());

        // Create the TagAssigner with the differences and the fitted model
        let tag_assigner = TagAssigner::new(differences.clone(), &fitted_model, 0.5);

        // Assign the final tags to the difference points
        let assigned_tags = tag_assigner.assign();

        // Define the expected tags for each difference
        let expected_tags = vec![
            Tag::increase(),
            Tag::increase(),
            Tag::increase_with_level(1),
            Tag::increase_with_level(1),
            Tag::increase_with_level(2),
        ];

        // Ensure the assigned tags match the expected tags
        assert_eq!(assigned_tags, Tags::from(expected_tags));
    }

    #[test]
    fn test_assign_all_negative_differences() {
        // Define centroids manually
        let centroids = array![
            [-15.0], // reduce(2)
            [-10.0], // reduce(1)
            [-5.0],  // reduce(0)
        ];

        // All differences are negative
        let differences = array![[-16.0], [-12.0], [-9.0], [-6.0], [-4.0],];

        // Force memberships
        let memberships = array![
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ];

        // Create a FittedModel manually with the centroids and memberships
        let fitted_model = FittedModel::new(centroids.clone(), memberships.clone());

        // Create the TagAssigner with the differences and the fitted model
        let tag_assigner = TagAssigner::new(differences.clone(), &fitted_model, 0.5);

        // Assign the final tags to the difference points
        let assigned_tags = tag_assigner.assign();

        // Define the expected tags for each difference
        let expected_tags = vec![
            Tag::reduce_with_level(2),
            Tag::reduce_with_level(2),
            Tag::reduce_with_level(1),
            Tag::reduce_with_level(1),
            Tag::reduce(),
        ];

        // Ensure the assigned tags match the expected tags
        assert_eq!(assigned_tags, Tags::from(expected_tags));
    }

    #[test]
    fn test_assign_two_dimensional_differences() {
        // Define centroids manually for 2D data
        let centroids = array![
            [-30.0, -30.0], // Should assign reduce(1)
            [-10.0, -10.0], // Should assign reduce(0)
            [0.0, 0.0],     // Should assign stay
            [10.0, 10.0],   // Should assign increase(0)
            [20.0, 20.0],   // Should assign increase(1)
        ];

        // 2D differences distributed between (-40, -40) and (40, 40)
        let differences = array![
            [-40.0, -40.0],
            [-35.0, -35.0],
            [-25.0, -25.0],
            [-15.0, -15.0],
            [-5.0, -5.0],
            [0.0, 0.0],
            [5.0, 5.0],
            [15.0, 15.0],
            [25.0, 25.0],
            [35.0, 35.0]
        ];

        // Force memberships to ensure each point is assigned to the nearest centroid
        let memberships = array![
            [1.0, 0.0, 0.0, 0.0, 0.0], // Closest to [-30.0, -30.0]
            [1.0, 0.0, 0.0, 0.0, 0.0], // Closest to [-30.0, -30.0]
            [0.0, 1.0, 0.0, 0.0, 0.0], // Closest to [-10.0, -10.0]
            [0.0, 1.0, 0.0, 0.0, 0.0], // Closest to [-10.0, -10.0]
            [0.0, 0.0, 1.0, 0.0, 0.0], // Closest to [0.0, 0.0]
            [0.0, 0.0, 1.0, 0.0, 0.0], // Exactly at [0.0, 0.0]
            [0.0, 0.0, 0.0, 1.0, 0.0], // Closest to [10.0, 10.0]
            [0.0, 0.0, 0.0, 1.0, 0.0], // Closest to [10.0, 10.0]
            [0.0, 0.0, 0.0, 0.0, 1.0], // Closest to [20.0, 20.0]
            [0.0, 0.0, 0.0, 0.0, 1.0], // Closest to [20.0, 20.0]
        ];

        // Create a FittedModel manually with the centroids and memberships
        let fitted_model = FittedModel::new(centroids.clone(), memberships.clone());

        // Create the TagAssigner with the differences and the fitted model
        let tag_assigner = TagAssigner::new(differences.clone(), &fitted_model, 0.5);

        // Assign the final tags to the difference points
        let assigned_tags = tag_assigner.assign();

        // Define the expected tags for each difference
        let expected_tags = vec![
            Tag::reduce_with_level(1),
            Tag::reduce_with_level(1), // [-40, -40], [-35, -35] -> reduce(1)
            Tag::reduce(),
            Tag::reduce(), // [-25, -25], [-15, -15] -> reduce(0)
            Tag::stay(),
            Tag::stay(), // [-5, -5], [0, 0] -> stay
            Tag::increase(),
            Tag::increase(), // [5, 5], [15, 15] -> increase(0)
            Tag::increase_with_level(1),
            Tag::increase_with_level(1), // [25, 25], [35, 35] -> increase(1)
        ];

        // Ensure the assigned tags match the expected tags
        assert_eq!(assigned_tags, Tags::from(expected_tags));
    }

    #[test]
    fn test_assign_with_tendency_tags() {
        // Define centroids manually
        let centroids = array![
            [-30.0], // Should assign reduce(1)
            [-10.0], // Should assign reduce(0)
            [0.0],   // Should assign stay
            [10.0],  // Should assign increase(0)
            [20.0],  // Should assign increase(1)
        ];

        // Differences with some values closer to two centroids
        let differences = array![
            [-25.0], // Closest to reduce(0) with tendency to reduce(1)
            [-5.0],  // Closest to stay with tendency to reduce(0)
            [15.0],  // Closest to increase(1) with tendency to increase(0)
        ];

        // Force memberships to reflect tendency
        let memberships = array![
            [0.2, 0.8, 0.0, 0.0, 0.0], // Strong to reduce(0) but some to reduce(1)
            [0.0, 0.3, 0.7, 0.0, 0.0], // Strong to stay but some to reduce(0)
            [0.0, 0.0, 0.0, 0.4, 0.6], // Strong to increase(1) but some to increase(0)
        ];

        // Create a FittedModel manually with the centroids and memberships
        let fitted_model = FittedModel::new(centroids.clone(), memberships.clone());

        // Create the TagAssigner with the differences and the fitted model
        let tag_assigner = TagAssigner::new(differences.clone(), &fitted_model, 0.2);

        // Assign the final tags to the difference points
        let assigned_tags = tag_assigner.assign();

        // Define the expected tags for each difference with tendency
        let expected_tags = vec![
            Tag::Tendency(Base::Reduce(0), Base::Reduce(1)), // reduce(0) with tendency to reduce(1)
            Tag::Tendency(Base::Stay, Base::Reduce(0)),      // stay with tendency to reduce(0)
            Tag::Tendency(Base::Increase(1), Base::Increase(0)), // increase(1) with tendency to increase(0)
        ];

        // Ensure the assigned tags match the expected tags
        assert_eq!(assigned_tags, Tags::from(expected_tags));
    }
}
