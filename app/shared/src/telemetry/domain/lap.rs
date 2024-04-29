#[derive(Clone, PartialEq)]
pub struct Lap {
    pub number: u16,
    pub distances: Vec<f32>,
    pub velocity: Vec<f32>,
}