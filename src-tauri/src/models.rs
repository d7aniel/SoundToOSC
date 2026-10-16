use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleConfig {
    pub id: String,
    pub name: String,
    pub category: String, // "1D", "2D", "3D", "4D"
    pub dimension: usize,
    pub enabled: bool,
    pub address: String,
    pub gain: f32,
    pub attack: f32,  // 0.0 to 1.0 (smoothing filter)
    pub decay: f32,   // 0.0 to 1.0 (smoothing filter)
    pub min_val: f32, // output range min
    pub max_val: f32, // output range max
}

impl ModuleConfig {
    pub fn new(
        id: &str,
        name: &str,
        category: &str,
        dimension: usize,
        enabled: bool,
        address: &str,
    ) -> Self {
        Self::new_with_range(id, name, category, dimension, enabled, address, 0.0, 1.0)
    }

    pub fn new_with_range(
        id: &str,
        name: &str,
        category: &str,
        dimension: usize,
        enabled: bool,
        address: &str,
        min_val: f32,
        max_val: f32,
    ) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            category: category.to_string(),
            dimension,
            enabled,
            address: address.to_string(),
            gain: 1.0,
            attack: 0.7,
            decay: 0.15,
            min_val,
            max_val,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineSettings {
    pub target_ip: String,
    pub target_port: u16,
    pub send_rate_hz: u32,
    pub input_device_name: Option<String>,
}

impl Default for EngineSettings {
    fn default() -> Self {
        Self {
            target_ip: "127.0.0.1".to_string(),
            target_port: 9000,
            send_rate_hz: 60,
            input_device_name: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDeviceInfo {
    pub id: String,
    pub name: String,
    pub is_default: bool,
    pub is_loopback: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryPayload {
    pub waveform: Vec<f32>,                 // ~128 sample points for oscilloscope
    pub spectrum: Vec<f32>,                 // ~64 frequency bins (log spaced)
    pub values: HashMap<String, Vec<f32>>,  // module_id -> [f32; 1..4]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preset {
    pub name: String,
    pub settings: EngineSettings,
    pub modules: Vec<ModuleConfig>,
}

pub fn get_default_modules() -> Vec<ModuleConfig> {
    vec![
        // 1D Modules
        ModuleConfig::new("rms", "RMS Loudness", "1D", 1, true, "/audio/rms"),
        ModuleConfig::new("peak", "Peak Amplitude", "1D", 1, false, "/audio/peak"),
        ModuleConfig::new("pitch", "Pitch Tracker", "1D", 1, false, "/audio/pitch"),
        ModuleConfig::new("pitch_class", "Pitch Class / Octave", "1D", 1, false, "/audio/note"),
        ModuleConfig::new("centroid", "Spectral Centroid (Brightness)", "1D", 1, false, "/audio/centroid"),
        ModuleConfig::new("flatness", "Spectral Flatness (Noisiness)", "1D", 1, false, "/audio/flatness"),
        ModuleConfig::new("flux", "Spectral Flux (Transients)", "1D", 1, false, "/audio/flux"),
        ModuleConfig::new("zcr", "Zero Crossing Rate", "1D", 1, false, "/audio/zcr"),
        ModuleConfig::new("onset", "Beat / Onset Pulse", "1D", 1, true, "/audio/onset"),
        ModuleConfig::new_with_range("stereo_pos", "Stereo Position", "1D", 1, false, "/audio/position", -1.0, 1.0),
        // 2D Modules
        ModuleConfig::new("stereo", "Stereo Amplitude [L, R]", "2D", 2, false, "/audio/stereo"),
        ModuleConfig::new("pitch_conf", "Pitch & Confidence [Pitch, Conf]", "2D", 2, false, "/audio/pitch_conf"),
        ModuleConfig::new("loud_bright", "Loudness & Brightness [RMS, Bright]", "2D", 2, false, "/audio/loud_bright"),
        // 3D Modules
        ModuleConfig::new("eq3", "3-Band EQ [Bass, Mid, Treble]", "3D", 3, true, "/audio/eq3"),
        ModuleConfig::new("rgb", "Audio-to-RGB Color [R, G, B]", "3D", 3, false, "/audio/rgb"),
        // 4D Modules
        ModuleConfig::new("eq4", "4-Band Spectrum [Sub, Low, Mid, High]", "4D", 4, false, "/audio/eq4"),
        ModuleConfig::new("composite4", "Composite Vector [RMS, Pitch, Centroid, Onset]", "4D", 4, false, "/audio/vec4"),
    ]
}
