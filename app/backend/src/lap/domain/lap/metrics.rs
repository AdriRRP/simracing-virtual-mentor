use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Metrics {
    pub speed: Vec<f32>,
    pub throttle: Vec<f32>,
    pub brake: Vec<f32>,
    pub clutch: Vec<f32>,
    pub gear: Vec<u8>,
    pub rpm: Vec<f32>,
    pub distance: Vec<f32>,
    pub distance_pct: Vec<f32>,
    pub track_temp: Vec<f32>,
    pub latitude: Vec<f64>,
    pub longitude: Vec<f64>,
    pub altitude: Vec<f32>,
    pub steering_wheel_angle: Vec<f32>,
    pub fuel_level: Vec<f32>,
    pub lap_current_lap_time: Vec<f32>,
}