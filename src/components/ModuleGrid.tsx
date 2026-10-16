import React, { useState } from 'react';
import { ModuleConfig } from '../types';
import { ModuleCard } from './ModuleCard';
import { useLanguage } from '../i18n';

interface ModuleGridProps {
  modules: ModuleConfig[];
  currentValues: Record<string, number[]>;
  onUpdateModule: (module: ModuleConfig) => void;
  onBatchToggle: (enable: boolean) => void;
}

export const ModuleGrid: React.FC<ModuleGridProps> = ({
  modules,
  currentValues,
  onUpdateModule,
  onBatchToggle,
}) => {
  const { t } = useLanguage();
  const [filter, setFilter] = useState<'ALL' | '1D' | '2D' | '3D' | '4D'>('ALL');

  const filteredModules = modules.filter((m) => {
    if (filter === 'ALL') return true;
    return m.category === filter;
  });

  const countFor = (cat: string) => modules.filter((m) => m.category === cat).length;
  const activeCount = modules.filter((m) => m.enabled).length;

  return (
    <div className="module-grid-section">
      <div className="grid-toolbar">
        <div className="filter-tabs">
          <button
            className={`tab-btn ${filter === 'ALL' ? 'tab-active' : ''}`}
            onClick={() => setFilter('ALL')}
          >
            {t.filterAll} ({modules.length})
          </button>
          <button
            className={`tab-btn ${filter === '1D' ? 'tab-active' : ''}`}
            onClick={() => setFilter('1D')}
          >
            {t.filter1D} ({countFor('1D')})
          </button>
          <button
            className={`tab-btn ${filter === '2D' ? 'tab-active' : ''}`}
            onClick={() => setFilter('2D')}
          >
            {t.filter2D} ({countFor('2D')})
          </button>
          <button
            className={`tab-btn ${filter === '3D' ? 'tab-active' : ''}`}
            onClick={() => setFilter('3D')}
          >
            {t.filter3D} ({countFor('3D')})
          </button>
          <button
            className={`tab-btn ${filter === '4D' ? 'tab-active' : ''}`}
            onClick={() => setFilter('4D')}
          >
            {t.filter4D} ({countFor('4D')})
          </button>
        </div>

        <div className="batch-actions">
          <span className="active-count-label">{t.activeModulesCount(activeCount, modules.length)}</span>
          <button className="batch-btn" onClick={() => onBatchToggle(true)}>{t.enableAll}</button>
          <button className="batch-btn" onClick={() => onBatchToggle(false)}>{t.disableAll}</button>
        </div>
      </div>


      <div className="cards-grid">
        {filteredModules.map((module) => (
          <ModuleCard
            key={module.id}
            module={module}
            currentValues={currentValues[module.id] || []}
            onUpdate={onUpdateModule}
          />
        ))}
      </div>
    </div>
  );
};
