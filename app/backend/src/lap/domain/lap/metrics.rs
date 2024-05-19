use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
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
}

impl Default for Metrics {
    fn default() -> Self {
        Self {
            speed: Vec::new(),
            throttle: Vec::new(),
            brake: Vec::new(),
            clutch: Vec::new(),
            gear: Vec::new(),
            rpm: Vec::new(),
            distance: Vec::new(),
            distance_pct: Vec::new(),
            track_temp: Vec::new(),
            latitude: Vec::new(),
            longitude: Vec::new(),
            altitude: Vec::new(),
            steering_wheel_angle: Vec::new(),
            fuel_level: Vec::new(),
        }
    }
}
