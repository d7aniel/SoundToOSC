use std::sync::atomic::Ordering;
use ringbuf::traits::Split;
use ringbuf::HeapRb;
use tauri::{AppHandle, State};

use crate::audio::AudioInputManager;
use crate::engine::{start_engine_loop, AppState};
use crate::models::{AudioDeviceInfo, EngineSettings, ModuleConfig, Preset};

#[tauri::command]
pub async fn get_audio_devices() -> Result<Vec<AudioDeviceInfo>, String> {
    Ok(AudioInputManager::list_devices())
}

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<EngineSettings, String> {
    let settings = state.inner.settings.lock().await;
    Ok(settings.clone())
}

#[tauri::command]
pub async fn update_settings(
    settings: EngineSettings,
    state: State<'_, AppState>,
) -> Result<(), String> {
    {
        let mut cur = state.inner.settings.lock().await;
        *cur = settings.clone();
    }
    // Auto-save session
    let modules = state.inner.modules.lock().await;
    let preset_mgr = state.inner.preset_manager.lock().await;
    let _ = preset_mgr.save_session(&settings, &modules);

    Ok(())
}

#[tauri::command]
pub async fn get_modules(state: State<'_, AppState>) -> Result<Vec<ModuleConfig>, String> {
    let modules = state.inner.modules.lock().await;
    Ok(modules.clone())
}

#[tauri::command]
pub async fn update_module(
    module: ModuleConfig,
    state: State<'_, AppState>,
) -> Result<(), String> {
    {
        let mut modules = state.inner.modules.lock().await;
        if let Some(existing) = modules.iter_mut().find(|m| m.id == module.id) {
            *existing = module;
        }
    }

    // Auto-save session
    let settings = state.inner.settings.lock().await;
    let modules = state.inner.modules.lock().await;
    let preset_mgr = state.inner.preset_manager.lock().await;
    let _ = preset_mgr.save_session(&settings, &modules);

    Ok(())
}

#[tauri::command]
pub async fn start_engine(
    device_name: Option<String>,
    app_handle: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    // Stop any existing stream
    {
        let mut audio_mgr = state.inner.audio_manager.lock().await;
        audio_mgr.stop_stream();
        state.inner.is_running.store(false, Ordering::SeqCst);
    }

    // Small delay to let previous loop gracefully terminate
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Allocate ring buffer (capacity: 48000 * 4 = ~4 seconds of audio)
    let rb = HeapRb::<f32>::new(48000 * 4);
    let (prod, cons) = rb.split();

    let (sample_rate, channels) = {
        let mut audio_mgr = state.inner.audio_manager.lock().await;
        audio_mgr.start_stream(device_name.as_deref(), prod)?
    };

    // Update settings with device name
    {
        let mut settings = state.inner.settings.lock().await;
        settings.input_device_name = device_name;
    }

    // Start background DSP & OSC thread
    start_engine_loop(state.inner.clone(), app_handle, cons, sample_rate, channels);

    Ok(())
}

#[tauri::command]
pub async fn stop_engine(state: State<'_, AppState>) -> Result<(), String> {
    let mut audio_mgr = state.inner.audio_manager.lock().await;
    audio_mgr.stop_stream();
    state.inner.is_running.store(false, Ordering::SeqCst);
    Ok(())
}

#[tauri::command]
pub async fn is_engine_running(state: State<'_, AppState>) -> Result<bool, String> {
    Ok(state.inner.is_running.load(Ordering::SeqCst))
}

#[tauri::command]
pub async fn list_presets(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let preset_mgr = state.inner.preset_manager.lock().await;
    Ok(preset_mgr.list_named_presets())
}

#[tauri::command]
pub async fn save_preset(
    name: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let settings = state.inner.settings.lock().await;
    let modules = state.inner.modules.lock().await;
    let preset_mgr = state.inner.preset_manager.lock().await;
    preset_mgr.save_named_preset(&name, &settings, &modules)
}

#[tauri::command]
pub async fn load_preset(
    name: String,
    state: State<'_, AppState>,
) -> Result<Preset, String> {
    let preset = {
        let preset_mgr = state.inner.preset_manager.lock().await;
        preset_mgr.load_named_preset(&name)?
    };

    {
        let mut settings = state.inner.settings.lock().await;
        *settings = preset.settings.clone();
    }
    {
        let mut modules = state.inner.modules.lock().await;
        *modules = preset.modules.clone();
    }

    // Auto-save session with newly loaded preset
    let preset_mgr = state.inner.preset_manager.lock().await;
    let _ = preset_mgr.save_session(&preset.settings, &preset.modules);

    Ok(preset)
}
