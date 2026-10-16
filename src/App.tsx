import React, { useEffect, useState, useCallback, useMemo } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen, UnlistenFn } from '@tauri-apps/api/event';
import { AudioDeviceInfo, EngineSettings, ModuleConfig, TelemetryPayload, Preset } from './types';
import { Header } from './components/Header';
import { MasterVisualizer } from './components/MasterVisualizer';
import { ModuleGrid } from './components/ModuleGrid';
import { useLanguage } from './i18n';
import './App.css';

type StatusState =
  | { type: 'initializing' }
  | { type: 'ready' }
  | { type: 'engine_active'; ip: string; port: number }
  | { type: 'engine_stopped' }
  | { type: 'streaming'; ip: string; port: number; hz: number }
  | { type: 'switched_device'; device: string }
  | { type: 'preset_saved'; name: string }
  | { type: 'preset_loaded'; name: string }
  | { type: 'init_error'; error: string }
  | { type: 'engine_error'; error: string }
  | { type: 'device_error'; error: string }
  | { type: 'preset_save_error'; error: string }
  | { type: 'preset_load_error'; error: string };

export const App: React.FC = () => {
  const { t } = useLanguage();
  const [isRunning, setIsRunning] = useState<boolean>(false);
  const [devices, setDevices] = useState<AudioDeviceInfo[]>([]);
  const [selectedDevice, setSelectedDevice] = useState<string | null>(null);
  const [settings, setSettings] = useState<EngineSettings>({
    target_ip: '127.0.0.1',
    target_port: 9000,
    send_rate_hz: 60,
    input_device_name: null,
  });
  const [modules, setModules] = useState<ModuleConfig[]>([]);
  const [presets, setPresets] = useState<string[]>([]);
  const [telemetry, setTelemetry] = useState<TelemetryPayload | null>(null);
  const [status, setStatus] = useState<StatusState>({ type: 'initializing' });

  const statusMessage = useMemo(() => {
    switch (status.type) {
      case 'initializing':
        return t.statusInitializing;
      case 'ready':
        return t.statusReady;
      case 'engine_active':
        return t.statusEngineActive(status.ip, status.port);
      case 'engine_stopped':
        return t.statusEngineStopped;
      case 'streaming':
        return t.statusStreaming(status.ip, status.port, status.hz);
      case 'switched_device':
        return t.statusSwitchedDevice(status.device);
      case 'preset_saved':
        return t.statusPresetSaved(status.name);
      case 'preset_loaded':
        return t.statusPresetLoaded(status.name);
      case 'init_error':
        return t.statusInitError(status.error);
      case 'engine_error':
        return t.statusEngineError(status.error);
      case 'device_error':
        return t.statusDeviceSwitchError(status.error);
      case 'preset_save_error':
        return t.statusPresetSaveError(status.error);
      case 'preset_load_error':
        return t.statusPresetLoadError(status.error);
    }
  }, [status, t]);

  // Initialize data on mount
  useEffect(() => {
    const init = async () => {
      try {
        const [devs, sets, mods, presList, active] = await Promise.all([
          invoke<AudioDeviceInfo[]>('get_audio_devices'),
          invoke<EngineSettings>('get_settings'),
          invoke<ModuleConfig[]>('get_modules'),
          invoke<string[]>('list_presets'),
          invoke<boolean>('is_engine_running'),
        ]);

        setDevices(devs);
        setSettings(sets);
        setModules(mods);
        setPresets(presList);
        setIsRunning(active);

        // Pick default device or stored device
        if (sets.input_device_name) {
          setSelectedDevice(sets.input_device_name);
        } else {
          const defaultDev = devs.find((d) => d.is_default);
          if (defaultDev) setSelectedDevice(defaultDev.id);
        }

        setStatus(active ? { type: 'engine_active', ip: sets.target_ip, port: sets.target_port } : { type: 'ready' });
      } catch (err) {
        console.error('Initialization error:', err);
        setStatus({ type: 'init_error', error: String(err) });
      }
    };

    init();

    // Listen to real-time audio telemetry from Rust
    let unlisten: UnlistenFn | undefined;
    listen<TelemetryPayload>('audio-telemetry', (event) => {
      setTelemetry(event.payload);
    }).then((fn) => {
      unlisten = fn;
    });

    return () => {
      if (unlisten) unlisten();
    };
  }, []);

  const handleToggleEngine = useCallback(async () => {
    try {
      if (isRunning) {
        await invoke('stop_engine');
        setIsRunning(false);
        setStatus({ type: 'engine_stopped' });
      } else {
        await invoke('start_engine', { deviceName: selectedDevice });
        setIsRunning(true);
        setStatus({
          type: 'streaming',
          ip: settings.target_ip,
          port: settings.target_port,
          hz: settings.send_rate_hz,
        });
      }
    } catch (err) {
      console.error('Failed to toggle engine:', err);
      setStatus({ type: 'engine_error', error: String(err) });
    }
  }, [isRunning, selectedDevice, settings]);

  const handleSelectDevice = useCallback(async (deviceId: string) => {
    setSelectedDevice(deviceId);
    if (isRunning) {
      try {
        await invoke('start_engine', { deviceName: deviceId });
        setStatus({ type: 'switched_device', device: deviceId });
      } catch (err) {
        console.error('Failed to switch device:', err);
        setStatus({ type: 'device_error', error: String(err) });
      }
    }
  }, [isRunning]);

  const handleUpdateSettings = useCallback(async (newSettings: EngineSettings) => {
    setSettings(newSettings);
    try {
      await invoke('update_settings', { settings: newSettings });
    } catch (err) {
      console.error('Failed to update settings:', err);
    }
  }, []);

  const handleUpdateModule = useCallback(async (updated: ModuleConfig) => {
    setModules((prev) => prev.map((m) => (m.id === updated.id ? updated : m)));
    try {
      await invoke('update_module', { module: updated });
    } catch (err) {
      console.error('Failed to update module:', err);
    }
  }, []);

  const handleBatchToggle = useCallback(async (enable: boolean) => {
    setModules((prev) => {
      const next = prev.map((m) => ({ ...m, enabled: enable }));
      // Persist each in background
      next.forEach((m) => invoke('update_module', { module: m }));
      return next;
    });
  }, []);

  const handleSavePreset = useCallback(async (name: string) => {
    try {
      await invoke('save_preset', { name });
      const updatedList = await invoke<string[]>('list_presets');
      setPresets(updatedList);
      setStatus({ type: 'preset_saved', name });
    } catch (err) {
      console.error('Failed to save preset:', err);
      setStatus({ type: 'preset_save_error', error: String(err) });
    }
  }, []);

  const handleLoadPreset = useCallback(async (name: string) => {
    try {
      const preset = await invoke<Preset>('load_preset', { name });
      setSettings(preset.settings);
      setModules(preset.modules);
      if (preset.settings.input_device_name) {
        setSelectedDevice(preset.settings.input_device_name);
      }
      setStatus({ type: 'preset_loaded', name });
    } catch (err) {
      console.error('Failed to load preset:', err);
      setStatus({ type: 'preset_load_error', error: String(err) });
    }
  }, []);

  return (
    <div className="app-container">
      <Header
        isRunning={isRunning}
        onToggleEngine={handleToggleEngine}
        devices={devices}
        selectedDevice={selectedDevice}
        onSelectDevice={handleSelectDevice}
        settings={settings}
        onUpdateSettings={handleUpdateSettings}
        presets={presets}
        onSavePreset={handleSavePreset}
        onLoadPreset={handleLoadPreset}
      />

      <main className="app-main">
        <MasterVisualizer telemetry={telemetry} isRunning={isRunning} />

        <ModuleGrid
          modules={modules}
          currentValues={telemetry?.values || {}}
          onUpdateModule={handleUpdateModule}
          onBatchToggle={handleBatchToggle}
        />
      </main>

      <footer className="app-footer">
        <div className="status-indicator">
          <span className={`status-dot ${isRunning ? 'dot-active' : 'dot-idle'}`} />
          <span className="status-text">{statusMessage}</span>
        </div>
        <div className="footer-meta">
          <span>{t.footerTarget}: {settings.target_ip}:{settings.target_port}</span>
          <span className="divider">•</span>
          <span>{t.footerFormat}: [f32; 1..4]</span>
          <span className="divider">•</span>
          <span>{t.footerCompatibility}: Andragon OSC</span>
        </div>
      </footer>
    </div>
  );
};


export default App;
