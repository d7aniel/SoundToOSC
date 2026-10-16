use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::StreamConfig;
use ringbuf::traits::Producer;
use ringbuf::HeapProd;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use crate::models::AudioDeviceInfo;

// Safe wrapper for cpal::Stream on Windows where COM pointers prevent auto-Send
pub struct SendStream(pub cpal::Stream);
unsafe impl Send for SendStream {}
unsafe impl Sync for SendStream {}

pub struct AudioInputManager {
    stream: Option<SendStream>,
    is_running: Arc<AtomicBool>,
    current_device_name: Option<String>,
    sample_rate: f32,
    channels: u16,
}

impl AudioInputManager {
    pub fn new() -> Self {
        Self {
            stream: None,
            is_running: Arc::new(AtomicBool::new(false)),
            current_device_name: None,
            sample_rate: 44100.0,
            channels: 1,
        }
    }

    pub fn list_devices() -> Vec<AudioDeviceInfo> {
        let host = cpal::default_host();
        let mut list = Vec::new();

        let default_name = host.default_input_device().and_then(|d| d.name().ok());

        if let Ok(devices) = host.input_devices() {
            for d in devices {
                if let Ok(name) = d.name() {
                    let is_default = default_name.as_ref().map_or(false, |def| def == &name);
                    list.push(AudioDeviceInfo {
                        id: name.clone(),
                        name: name.clone(),
                        is_default,
                        is_loopback: false,
                    });
                }
            }
        }

        // On Windows with WASAPI, output devices can be opened for loopback capture
        #[cfg(target_os = "windows")]
        {
            if let Ok(outputs) = host.output_devices() {
                for d in outputs {
                    if let Ok(name) = d.name() {
                        list.push(AudioDeviceInfo {
                            id: format!("loopback:{}", name),
                            name: format!("(Loopback) {}", name),
                            is_default: false,
                            is_loopback: true,
                        });
                    }
                }
            }
        }

        list
    }

    pub fn start_stream(
        &mut self,
        device_name: Option<&str>,
        mut producer: HeapProd<f32>,
    ) -> Result<(f32, u16), String> {
        self.stop_stream();

        let host = cpal::default_host();

        let (device, is_loopback) = if let Some(name) = device_name {
            if name.starts_with("loopback:") {
                let actual_name = &name["loopback:".len()..];
                let dev = host
                    .output_devices()
                    .map_err(|e| format!("Failed to list output devices: {}", e))?
                    .find(|d| d.name().map_or(false, |n| n == actual_name))
                    .ok_or_else(|| format!("Loopback device '{}' not found", actual_name))?;
                (dev, true)
            } else {
                let dev = host
                    .input_devices()
                    .map_err(|e| format!("Failed to list input devices: {}", e))?
                    .find(|d| d.name().map_or(false, |n| n == name))
                    .ok_or_else(|| format!("Input device '{}' not found", name))?;
                (dev, false)
            }
        } else {
            let dev = host
                .default_input_device()
                .ok_or_else(|| "No default input audio device found".to_string())?;
            (dev, false)
        };

        let device_display_name = device.name().unwrap_or_else(|_| "Unknown Device".to_string());
        println!("[Audio] Opening device: {} (loopback: {})", device_display_name, is_loopback);

        let default_config = if is_loopback {
            device
                .default_output_config()
                .map_err(|e| format!("Failed to get loopback config: {}", e))?
        } else {
            device
                .default_input_config()
                .map_err(|e| format!("Failed to get input config: {}", e))?
        };

        let sample_rate = default_config.sample_rate().0 as f32;
        let channels = default_config.channels();
        let sample_format = default_config.sample_format();
        let config: StreamConfig = default_config.into();

        let err_fn = |err| eprintln!("[Audio Stream Error]: {}", err);

        let stream = match sample_format {
            cpal::SampleFormat::F32 => device.build_input_stream(
                &config,
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    for &s in data {
                        let _ = producer.try_push(s);
                    }
                },
                err_fn,
                None,
            ),
            cpal::SampleFormat::I16 => device.build_input_stream(
                &config,
                move |data: &[i16], _: &cpal::InputCallbackInfo| {
                    for &s in data {
                        let float_val = s as f32 / 32768.0;
                        let _ = producer.try_push(float_val);
                    }
                },
                err_fn,
                None,
            ),
            cpal::SampleFormat::U16 => device.build_input_stream(
                &config,
                move |data: &[u16], _: &cpal::InputCallbackInfo| {
                    for &s in data {
                        let float_val = (s as f32 - 32768.0) / 32768.0;
                        let _ = producer.try_push(float_val);
                    }
                },
                err_fn,
                None,
            ),
            _ => return Err(format!("Unsupported audio sample format: {:?}", sample_format)),
        }
        .map_err(|e| format!("Failed to build audio stream: {}", e))?;

        stream
            .play()
            .map_err(|e| format!("Failed to start audio playback stream: {}", e))?;

        self.stream = Some(SendStream(stream));
        self.is_running.store(true, Ordering::SeqCst);
        self.current_device_name = Some(device_display_name);
        self.sample_rate = sample_rate;
        self.channels = channels;

        Ok((sample_rate, channels))
    }

    pub fn stop_stream(&mut self) {
        if let Some(stream) = self.stream.take() {
            drop(stream);
        }
        self.is_running.store(false, Ordering::SeqCst);
    }
}
