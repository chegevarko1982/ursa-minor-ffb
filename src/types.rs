use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc,
};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct FlightVars {
    pub sim_time_s: f64,
    pub airspeed_indicated: f64,
    pub on_ground: bool,
    pub bank_deg: f64,
    pub flaps_pct: f64,
    pub flaps_index: i32,
    pub gear_handle: f64,
    pub stalled: bool,
    pub ground_speed_kt: f64,
    pub paused: bool,
    pub spoilers_pct: f64, // Положение спойлеров в % (0.0 - 100.0)
    pub gear_comp_nose: f64,
    pub gear_comp_left: f64,
    pub gear_comp_right: f64,
}

impl Default for FlightVars {
    fn default() -> Self {
        Self {
            sim_time_s: 0.0,
            airspeed_indicated: 0.0,
            on_ground: false,
            bank_deg: 0.0,
            flaps_pct: 0.0,
            flaps_index: 0,
            gear_handle: 0.0,
            stalled: false,
            ground_speed_kt: 0.0,
            paused: false,
            spoilers_pct: 0.0,
            gear_comp_nose: 0.0,
            gear_comp_left: 0.0,
            gear_comp_right: 0.0,
        }
    }
}

// Остальной код остается без изменений
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RumbleConfig {
    pub overspeed_enabled: bool,
    pub overspeed_threshold_kn: f32,
    // ... (остальные поля RumbleConfig)
}