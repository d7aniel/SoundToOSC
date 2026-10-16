import React, { useState } from 'react';
import { ModuleConfig } from '../types';
import { useLanguage } from '../i18n';

interface ModuleCardProps {
  module: ModuleConfig;
  currentValues: number[];
  onUpdate: (updated: ModuleConfig) => void;
}

export const ModuleCard: React.FC<ModuleCardProps> = ({ module, currentValues, onUpdate }) => {
  const { t, getModuleName } = useLanguage();
  const [showShaper, setShowShaper] = useState(false);

  const toggleEnabled = () => {
    onUpdate({ ...module, enabled: !module.enabled });
  };

  const handleAddressChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    onUpdate({ ...module, address: e.target.value });
  };

  const handleGainChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    onUpdate({ ...module, gain: parseFloat(e.target.value) || 1.0 });
  };

  const handleAttackChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    onUpdate({ ...module, attack: parseFloat(e.target.value) || 0.5 });
  };

  const handleDecayChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    onUpdate({ ...module, decay: parseFloat(e.target.value) || 0.2 });
  };

  const handleMinChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    onUpdate({ ...module, min_val: parseFloat(e.target.value) || 0.0 });
  };

  const handleMaxChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    onUpdate({ ...module, max_val: parseFloat(e.target.value) || 1.0 });
  };

  const categoryColorClass = {
    '1D': 'badge-1d',
    '2D': 'badge-2d',
    '3D': 'badge-3d',
    '4D': 'badge-4d',
  }[module.category] || 'badge-1d';

  return (
    <div className={`module-card ${module.enabled ? 'card-enabled' : 'card-disabled'}`}>
      <div className="card-header">
        <div className="card-title-row">
          <label className="switch">
            <input type="checkbox" checked={module.enabled} onChange={toggleEnabled} />
            <span className="slider round"></span>
          </label>
          <span className="module-name">{getModuleName(module.id, module.name)}</span>
          <span className={`dim-badge ${categoryColorClass}`}>{module.category}</span>
        </div>

        <button
          className={`gear-btn ${showShaper ? 'gear-active' : ''}`}
          onClick={() => setShowShaper(!showShaper)}
          title={t.shaperToggleTitle}
        >
          ⚙
        </button>
      </div>

      <div className="card-address-row">
        <span className="osc-label">OSC</span>
        <input
          type="text"
          className="osc-address-input"
          value={module.address}
          onChange={handleAddressChange}
          placeholder="/address"
        />
      </div>

      {/* Live Value Meters */}
      <div className="card-meters">
        {Array.from({ length: module.dimension }).map((_, idx) => {
          const val = currentValues[idx] ?? 0;
          const isBipolar = module.id === 'stereo_pos' || (module.min_val < 0 && module.max_val > 0);

          let fillStyle: React.CSSProperties;
          let fillClass = 'meter-fill';
          let displayNum = val.toFixed(3);

          if (isBipolar) {
            const centerVal = (module.min_val + module.max_val) / 2;
            const halfRange = (module.max_val - module.min_val) / 2;
            const norm = halfRange > 0 ? (val - centerVal) / halfRange : 0;
            const clamped = Math.max(-1, Math.min(1, norm));

            fillStyle = {
              left: clamped >= 0 ? '50%' : `${50 + clamped * 50}%`,
              width: `${Math.abs(clamped) * 50}%`,
              opacity: module.enabled ? 1 : 0.4,
            };
            fillClass = `meter-fill ${clamped >= 0 ? 'meter-fill-bipolar-right' : 'meter-fill-bipolar-left'}`;
            displayNum = (val > 0.0005 ? '+' : '') + val.toFixed(3);
          } else {
            const range = module.max_val - module.min_val;
            const normPercent = range > 0
              ? Math.max(0, Math.min(100, ((val - module.min_val) / range) * 100))
              : Math.max(0, Math.min(100, val * 100));

            fillStyle = {
              left: 0,
              width: `${normPercent}%`,
              opacity: module.enabled ? 1 : 0.4,
            };
          }

          const label = module.id === 'stereo_pos' ? 'POS' : `v${idx}`;

          return (
            <div key={idx} className="meter-row">
              <span className="meter-label">{label}</span>
              {module.id === 'stereo_pos' && <span className="pan-indicator-tag">L</span>}
              <div className={`meter-track ${isBipolar ? 'meter-track-bipolar' : ''}`}>
                {isBipolar && <div className="meter-center-tick" />}
                <div className={fillClass} style={fillStyle} />
              </div>
              {module.id === 'stereo_pos' && <span className="pan-indicator-tag">R</span>}
              <span className="meter-num">{displayNum}</span>
            </div>
          );
        })}
      </div>

      {/* Shaping Controls Accordion */}
      {showShaper && (
        <div className="card-shaper-panel">
          <div className="shaper-row">
            <label>{t.gainLabel(module.gain.toFixed(2))}</label>
            <input
              type="range"
              min="0.1"
              max="4.0"
              step="0.05"
              value={module.gain}
              onChange={handleGainChange}
            />
          </div>

          <div className="shaper-row">
            <label>{t.attackLabel(module.attack.toFixed(2))}</label>
            <input
              type="range"
              min="0.05"
              max="1.0"
              step="0.05"
              value={module.attack}
              onChange={handleAttackChange}
            />
          </div>

          <div className="shaper-row">
            <label>{t.decayLabel(module.decay.toFixed(2))}</label>
            <input
              type="range"
              min="0.01"
              max="0.8"
              step="0.02"
              value={module.decay}
              onChange={handleDecayChange}
            />
          </div>

          <div className="range-row">
            <div className="range-input-group">
              <label>{t.minLabel}</label>
              <input
                type="number"
                step="0.1"
                value={module.min_val}
                onChange={handleMinChange}
              />
            </div>
            <div className="range-input-group">
              <label>{t.maxLabel}</label>
              <input
                type="number"
                step="0.1"
                value={module.max_val}
                onChange={handleMaxChange}
              />
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

