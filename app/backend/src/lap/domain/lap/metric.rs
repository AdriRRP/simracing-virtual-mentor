#[derive(Clone)]
pub struct Metric {
    pub speed: f32,
    pub throttle: f32,
    pub brake: f32,
    pub clutch: f32,
    pub gear: u8,
    pub rpm: f32,
    pub distance: f32,
    pub distance_pct: f32,
    pub track_temp: f32,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f32,
    pub steering_wheel_angle: f32,
    pub fuel_level: f32,
}
