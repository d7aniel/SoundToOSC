import React, { useState } from 'react';
import { AudioDeviceInfo, EngineSettings } from '../types';
import { useLanguage } from '../i18n';

interface HeaderProps {
  isRunning: boolean;
  onToggleEngine: () => void;
  devices: AudioDeviceInfo[];
  selectedDevice: string | null;
  onSelectDevice: (deviceId: string) => void;
  settings: EngineSettings;
  onUpdateSettings: (settings: EngineSettings) => void;
  presets: string[];
  onSavePreset: (name: string) => void;
  onLoadPreset: (name: string) => void;
}

export const Header: React.FC<HeaderProps> = ({
  isRunning,
  onToggleEngine,
  devices,
  selectedDevice,
  onSelectDevice,
  settings,
  onUpdateSettings,
  presets,
  onSavePreset,
  onLoadPreset,
}) => {
  const { t, language, setLanguage } = useLanguage();
  const [newPresetName, setNewPresetName] = useState('');
  const [showPresetInput, setShowPresetInput] = useState(false);

  const handleSavePreset = (e: React.FormEvent) => {
    e.preventDefault();
    if (newPresetName.trim()) {
      onSavePreset(newPresetName.trim());
      setNewPresetName('');
      setShowPresetInput(false);
    }
  };

  return (
    <header className="app-header">
      <div className="header-top">
        <div className="brand-section">
          <div className="brand-icon">
            <span className="pulse-dot" data-active={isRunning}></span>
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
              <path d="M12 2v20M17 5v14M7 9v6M22 10v4M2 11v2" />
            </svg>
          </div>
          <div>
            <h1 className="brand-title">SoundToOSC</h1>
            <span className="brand-subtitle">{t.brandSubtitle}</span>
          </div>
        </div>

        <div className="engine-control-section">
          <div className="lang-toggle-group" role="group" aria-label={t.language}>
            <button
              type="button"
              className={`lang-btn ${language === 'es' ? 'lang-btn-active' : ''}`}
              onClick={() => setLanguage('es')}
              title="Cambiar a Español"
            >
              ES
            </button>
            <button
              type="button"
              className={`lang-btn ${language === 'en' ? 'lang-btn-active' : ''}`}
              onClick={() => setLanguage('en')}
              title="Switch to English"
            >
              EN
            </button>
          </div>

          <button
            id="engine-toggle-btn"
            className={`engine-btn ${isRunning ? 'engine-active' : 'engine-stopped'}`}
            onClick={onToggleEngine}
          >
            {isRunning ? (
              <>
                <span className="btn-indicator active"></span>
                <span>{t.stopEngine}</span>
              </>
            ) : (
              <>
                <span className="btn-indicator inactive"></span>
                <span>{t.startEngine}</span>
              </>
            )}
          </button>
        </div>
      </div>

      <div className="controls-bar">
        {/* Audio Device Selection */}
        <div className="control-group">
          <label htmlFor="device-select">{t.audioInput}</label>
          <select
            id="device-select"
            className="select-input"
            value={selectedDevice || ''}
            onChange={(e) => onSelectDevice(e.target.value)}
          >
            {devices.map((d) => (
              <option key={d.id} value={d.id}>
                {d.is_default ? `★ ${d.name}` : d.name}
              </option>
            ))}
          </select>
        </div>

        {/* Target Host */}
        <div className="control-group">
          <label htmlFor="target-ip">{t.targetHost}</label>
          <input
            id="target-ip"
            type="text"
            className="text-input"
            value={settings.target_ip}
            onChange={(e) => onUpdateSettings({ ...settings, target_ip: e.target.value })}
            placeholder="127.0.0.1"
          />
        </div>

        {/* Target Port */}
        <div className="control-group small-group">
          <label htmlFor="target-port">{t.port}</label>
          <input
            id="target-port"
            type="number"
            className="text-input"
            value={settings.target_port}
            onChange={(e) => onUpdateSettings({ ...settings, target_port: parseInt(e.target.value, 10) || 9000 })}
            placeholder="9000"
          />
        </div>

        {/* Send Rate Slider */}
        <div className="control-group rate-group">
          <div className="label-with-val">
            <label htmlFor="send-rate">{t.sendRate}</label>
            <span className="rate-val">{settings.send_rate_hz} Hz</span>
          </div>
          <input
            id="send-rate"
            type="range"
            min="10"
            max="120"
            step="5"
            value={settings.send_rate_hz}
            onChange={(e) => onUpdateSettings({ ...settings, send_rate_hz: parseInt(e.target.value, 10) })}
          />
        </div>

        {/* Presets */}
        <div className="control-group presets-group">
          <label htmlFor="preset-select">{t.presets}</label>
          <div className="preset-actions">
            <select
              id="preset-select"
              className="select-input"
              defaultValue=""
              onChange={(e) => {
                if (e.target.value) onLoadPreset(e.target.value);
              }}
            >
              <option value="" disabled>{t.loadPresetPlaceholder}</option>
              {presets.map((p) => (
                <option key={p} value={p}>{p}</option>
              ))}
            </select>
            <button
              className="preset-btn"
              title={t.savePresetTitle}
              onClick={() => setShowPresetInput(!showPresetInput)}
            >
              {t.savePresetBtn}
            </button>
          </div>
        </div>
      </div>

      {showPresetInput && (
        <form className="preset-save-dialog" onSubmit={handleSavePreset}>
          <input
            type="text"
            className="text-input"
            placeholder={t.presetInputPlaceholder}
            value={newPresetName}
            onChange={(e) => setNewPresetName(e.target.value)}
            autoFocus
          />
          <button type="submit" className="save-confirm-btn">{t.confirmSave}</button>
          <button type="button" className="cancel-btn" onClick={() => setShowPresetInput(false)}>{t.cancel}</button>
        </form>
      )}
    </header>
  );
};

