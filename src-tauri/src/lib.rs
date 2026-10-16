pub mod audio;
pub mod commands;
pub mod dsp;
pub mod engine;
pub mod models;
pub mod osc;
pub mod presets;

use tauri::Manager;
use engine::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let state = AppState::new(app.handle()).map_err(|e| Box::<dyn std::error::Error>::from(e))?;
            app.manage(state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_audio_devices,
            commands::get_settings,
            commands::update_settings,
            commands::get_modules,
            commands::update_module,
            commands::start_engine,
            commands::stop_engine,
            commands::is_engine_running,
            commands::list_presets,
            commands::save_preset,
            commands::load_preset,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
