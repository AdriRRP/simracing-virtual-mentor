use serde::{Deserialize, Serialize};

/// Represents a collection of telemetry variables.
#[derive(Serialize, Deserialize, PartialEq, Default, Clone, Debug)]
pub struct Variables {
    /// Speed measurements obtained from GPS (m/s).
    pub speed: Vec<f32>, // X

    /// Position of the throttle pedal (percentage)
    /// ranging from 0 (off throttle) to 1 (full throttle).
    pub throttle: Vec<f32>, // X

    /// Pressure applied on the brake pedal (percentage)
    /// ranging from 0 (brake released) to 1 (maximum pedal force).
    pub brake: Vec<f32>, // X

    /// Position of the clutch pedal (percentage)
    /// ranging from 0 (disengaged) to 1 (fully engaged).
    pub clutch: Vec<f32>, // X

    /// Position of the gear (-1 for reverse, 0 for neutral, and 1..n for current gear).
    pub gear: Vec<i8>,

    /// Revolutions per minute of the engine (rpm).
    pub rpm: Vec<f32>,

    /// Total distance traveled (m).
    pub distance: Vec<f32>,

    /// Percentage of the total distance traveled (%).
    pub distance_pct: Vec<f32>,

    /// Temperature of the track measured by the crew (°C).
    pub track_temp: Vec<f32>,

    /// Latitude coordinates (degrees).
    pub latitude: Vec<f64>, // X

    /// Longitude coordinates (degrees).
    pub longitude: Vec<f64>, // X

    /// Altitude above sea level (m).
    pub altitude: Vec<f32>,

    /// Angle of the steering wheel (radians).
    pub steering_wheel_angle: Vec<f32>, // X

    /// Remaining fuel level (liters).
    pub fuel_level: Vec<f32>,

    /// Current lap time measurements (s).
    pub lap_current_lap_time: Vec<f32>,
}
