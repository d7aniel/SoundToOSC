use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use ringbuf::traits::Consumer;
use ringbuf::HeapCons;
use tauri::{AppHandle, Emitter};
use tokio::sync::Mutex;

use crate::audio::AudioInputManager;
use crate::dsp::{AnalysisResult, AudioAnalysisEngine, ValueShaper};
use crate::models::{EngineSettings, ModuleConfig, TelemetryPayload};
use crate::osc::OscSender;
use crate::presets::PresetManager;

pub struct AppStateInner {
    pub settings: Mutex<EngineSettings>,
    pub modules: Mutex<Vec<ModuleConfig>>,
    pub audio_manager: Mutex<AudioInputManager>,
    pub osc_sender: OscSender,
    pub preset_manager: Mutex<PresetManager>,
    pub is_running: Arc<AtomicBool>,
}

#[derive(Clone)]
pub struct AppState {
    pub inner: Arc<AppStateInner>,
}

impl AppState {
    pub fn new(app: &AppHandle) -> Result<Self, String> {
        let osc_sender = OscSender::new()?;
        let preset_manager = PresetManager::new(app);
        let (settings, modules) = preset_manager.load_session();

        let inner = AppStateInner {
            settings: Mutex::new(settings),
            modules: Mutex::new(modules),
            audio_manager: Mutex::new(AudioInputManager::new()),
            osc_sender,
            preset_manager: Mutex::new(preset_manager),
            is_running: Arc::new(AtomicBool::new(false)),
        };

        Ok(Self {
            inner: Arc::new(inner),
        })
    }
}

pub fn start_engine_loop(
    state: Arc<AppStateInner>,
    app_handle: AppHandle,
    mut consumer: HeapCons<f32>,
    sample_rate: f32,
    channels: u16,
) {
    state.is_running.store(true, Ordering::SeqCst);
    let is_running = state.is_running.clone();

    std::thread::spawn(move || {
        let mut dsp_engine = AudioAnalysisEngine::new();
        let mut shaper = ValueShaper::new();

        // 2048 sample window for DSP analysis
        let window_size = 2048;
        let mut mono_window = vec![0.0f32; window_size];
        let mut left_window = vec![0.0f32; window_size];
        let mut right_window = vec![0.0f32; window_size];

        let mut read_scratch = Vec::with_capacity(4096);

        println!("[Engine] Starting DSP & OSC loop (channels: {}, sr: {})", channels, sample_rate);

        while is_running.load(Ordering::Relaxed) {
            // Read target send rate from settings
            let (target_ip, target_port, send_rate_hz) = {
                if let Ok(settings) = state.settings.try_lock() {
                    (settings.target_ip.clone(), settings.target_port, settings.send_rate_hz.clamp(10, 120))
                } else {
                    ("127.0.0.1".to_string(), 9000, 60)
                }
            };

            // Read available audio samples from ring buffer
            read_scratch.clear();
            while let Some(sample) = consumer.try_pop() {
                read_scratch.push(sample);
                if read_scratch.len() >= 8192 {
                    break;
                }
            }

            // If we have new audio samples, push them into sliding window
            if !read_scratch.is_empty() {
                if channels >= 2 {
                    let mut i = 0;
                    while i + 1 < read_scratch.len() {
                        let l = read_scratch[i];
                        let r = read_scratch[i + 1];
                        let m = (l + r) * 0.5;

                        mono_window.remove(0);
                        mono_window.push(m);

                        left_window.remove(0);
                        left_window.push(l);

                        right_window.remove(0);
                        right_window.push(r);

                        i += channels as usize;
                    }
                } else {
                    for &s in &read_scratch {
                        mono_window.remove(0);
                        mono_window.push(s);
                    }
                }
            }

            // Run DSP analysis
            let result = dsp_engine.analyze(
                &mono_window,
                if channels >= 2 { Some(&left_window) } else { None },
                if channels >= 2 { Some(&right_window) } else { None },
                sample_rate,
            );

            // Process active modules & OSC transmission
            let mut telemetry_values: HashMap<String, Vec<f32>> = HashMap::new();

            if let Ok(modules) = state.modules.try_lock() {
                for m in modules.iter() {
                    let raw_values = extract_module_raw_values(&m.id, &result);
                    let shaped = shaper.shape(
                        &m.id,
                        &raw_values,
                        m.gain,
                        m.attack,
                        m.decay,
                        m.min_val,
                        m.max_val,
                    );

                    if m.enabled {
                        // Send OSC message with 1 to 4 float components.
                        // Bound to ephemeral client port, targeting Andragon on target_ip:target_port!
                        let _ = state.osc_sender.send_message(
                            &target_ip,
                            target_port,
                            &m.address,
                            &shaped,
                        );
                    }

                    telemetry_values.insert(m.id.clone(), shaped);
                }
            }

            // Emit live telemetry for UI visualizer
            let payload = TelemetryPayload {
                waveform: result.waveform,
                spectrum: result.spectrum_bins,
                values: telemetry_values,
            };
            let _ = app_handle.emit("audio-telemetry", payload);

            // Throttle loop to configured send rate (e.g. 60Hz -> ~16.6ms)
            let frame_duration = Duration::from_micros((1_000_000 / send_rate_hz as u64).max(5000));
            std::thread::sleep(frame_duration);
        }

        println!("[Engine] Stopped DSP & OSC loop");
    });
}

fn extract_module_raw_values(id: &str, res: &AnalysisResult) -> Vec<f32> {
    match id {
        "rms" => vec![res.rms],
        "peak" => vec![res.peak],
        "pitch" => vec![res.pitch],
        "pitch_class" => vec![res.pitch_class],
        "centroid" => vec![res.centroid],
        "flatness" => vec![res.flatness],
        "flux" => vec![res.flux],
        "zcr" => vec![res.zcr],
        "onset" => vec![res.onset],
        "stereo_pos" => vec![res.stereo_pos],
        "stereo" => vec![res.stereo[0], res.stereo[1]],
        "pitch_conf" => vec![res.pitch, res.pitch_conf],
        "loud_bright" => vec![res.rms, res.centroid],
        "eq3" => vec![res.eq3[0], res.eq3[1], res.eq3[2]],
        "rgb" => vec![res.rgb[0], res.rgb[1], res.rgb[2]],
        "eq4" => vec![res.eq4[0], res.eq4[1], res.eq4[2], res.eq4[3]],
        "composite4" => vec![res.rms, res.pitch, res.centroid, res.onset],
        _ => vec![res.rms],
    }
}
