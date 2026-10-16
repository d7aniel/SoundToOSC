export interface ModuleConfig {
  id: string;
  name: string;
  category: '1D' | '2D' | '3D' | '4D';
  dimension: number;
  enabled: boolean;
  address: string;
  gain: number;
  attack: number;
  decay: number;
  min_val: number;
  max_val: number;
}

export interface EngineSettings {
  target_ip: string;
  target_port: number;
  send_rate_hz: number;
  input_device_name: string | null;
}

export interface AudioDeviceInfo {
  id: string;
  name: string;
  is_default: boolean;
  is_loopback: boolean;
}

export interface TelemetryPayload {
  waveform: number[];
  spectrum: number[];
  values: Record<string, number[]>;
}

export interface Preset {
  name: string;
  settings: EngineSettings;
  modules: ModuleConfig[];
}
