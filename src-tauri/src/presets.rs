use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use crate::models::{get_default_modules, EngineSettings, ModuleConfig, Preset};

const SESSION_FILE_NAME: &str = "session.json";
const PRESETS_DIR_NAME: &str = "presets";

pub struct PresetManager {
    config_dir: PathBuf,
}

impl PresetManager {
    pub fn new(app: &AppHandle) -> Self {
        let config_dir = app
            .path()
            .app_config_dir()
            .unwrap_or_else(|_| PathBuf::from("."));

        let _ = fs::create_dir_all(&config_dir);
        let _ = fs::create_dir_all(config_dir.join(PRESETS_DIR_NAME));

        Self { config_dir }
    }

    pub fn load_session(&self) -> (EngineSettings, Vec<ModuleConfig>) {
        let path = self.config_dir.join(SESSION_FILE_NAME);
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(preset) = serde_json::from_str::<Preset>(&content) {
                    println!("[PresetManager] Loaded existing session from {:?}", path);
                    let mut modules = preset.modules;
                    for default_mod in get_default_modules() {
                        if !modules.iter().any(|m| m.id == default_mod.id) {
                            modules.push(default_mod);
                        }
                    }
                    return (preset.settings, modules);
                }
            }
        }

        println!("[PresetManager] Starting with default configuration");
        (EngineSettings::default(), get_default_modules())
    }

    pub fn save_session(&self, settings: &EngineSettings, modules: &[ModuleConfig]) -> Result<(), String> {
        let path = self.config_dir.join(SESSION_FILE_NAME);
        let preset = Preset {
            name: "Current Session".to_string(),
            settings: settings.clone(),
            modules: modules.to_vec(),
        };

        let json = serde_json::to_string_pretty(&preset)
            .map_err(|e| format!("Failed to serialize session: {}", e))?;
        fs::write(&path, json).map_err(|e| format!("Failed to write session file: {}", e))?;

        Ok(())
    }

    pub fn save_named_preset(&self, name: &str, settings: &EngineSettings, modules: &[ModuleConfig]) -> Result<(), String> {
        let sanitized = name.replace(|c: char| !c.is_alphanumeric() && c != '_' && c != '-', "_");
        let path = self.config_dir.join(PRESETS_DIR_NAME).join(format!("{}.json", sanitized));

        let preset = Preset {
            name: name.to_string(),
            settings: settings.clone(),
            modules: modules.to_vec(),
        };

        let json = serde_json::to_string_pretty(&preset)
            .map_err(|e| format!("Failed to serialize preset: {}", e))?;
        fs::write(&path, json).map_err(|e| format!("Failed to write preset file: {}", e))?;

        Ok(())
    }

    pub fn list_named_presets(&self) -> Vec<String> {
        let dir = self.config_dir.join(PRESETS_DIR_NAME);
        let mut list = Vec::new();

        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                if let Some(ext) = entry.path().extension() {
                    if ext == "json" {
                        if let Some(stem) = entry.path().file_stem() {
                            list.push(stem.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }

        list
    }

    pub fn load_named_preset(&self, name: &str) -> Result<Preset, String> {
        let path = self.config_dir.join(PRESETS_DIR_NAME).join(format!("{}.json", name));
        let content = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read preset '{}': {}", name, e))?;
        let preset = serde_json::from_str::<Preset>(&content)
            .map_err(|e| format!("Failed to parse preset '{}': {}", name, e))?;

        Ok(preset)
    }
}
