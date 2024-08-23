use crate::lap::domain::lap::Lap;

#[must_use]
pub fn generate_union(lap1: &Lap, lap2: &Lap) -> Vec<f32> {
    let mut distances: Vec<f32> = lap1
        .variables
        .distance
        .iter()
        .chain(lap2.variables.distance.iter())
        .copied()
        .collect();

    // Sort, considering NaN as the smallest or largest (here: smallest)
    distances.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Less));

    distances.dedup();
    distances
}
