export type Language = 'es' | 'en';

export interface Translations {
  // Brand & Header
  brandSubtitle: string;
  startEngine: string;
  stopEngine: string;
  audioInput: string;
  targetHost: string;
  port: string;
  sendRate: string;
  presets: string;
  loadPresetPlaceholder: string;
  savePresetBtn: string;
  savePresetTitle: string;
  presetInputPlaceholder: string;
  confirmSave: string;
  cancel: string;
  language: string;

  // Visualizer
  visualizerListening: string;
  visualizerStopped: string;
  oscilloscope: string;
  fftSpectrum: string;
  bandSub: string;
  bandLow: string;
  bandMid: string;
  bandHigh: string;
  stereoPositionGauge: string;
  stereoCenter: string;

  // Module Grid
  filterAll: string;
  filter1D: string;
  filter2D: string;
  filter3D: string;
  filter4D: string;
  activeModulesCount: (active: number, total: number) => string;
  enableAll: string;
  disableAll: string;

  // Module Card
  shaperToggleTitle: string;
  gainLabel: (gain: string) => string;
  attackLabel: (attack: string) => string;
  decayLabel: (decay: string) => string;
  minLabel: string;
  maxLabel: string;

  // Status & App Messages
  statusInitializing: string;
  statusReady: string;
  statusEngineActive: (ip: string, port: number) => string;
  statusEngineStopped: string;
  statusStreaming: (ip: string, port: number, hz: number) => string;
  statusInitError: (err: string) => string;
  statusEngineError: (err: string) => string;
  statusSwitchedDevice: (device: string) => string;
  statusDeviceSwitchError: (err: string) => string;
  statusPresetSaved: (name: string) => string;
  statusPresetSaveError: (err: string) => string;
  statusPresetLoaded: (name: string) => string;
  statusPresetLoadError: (err: string) => string;

  // Footer
  footerTarget: string;
  footerFormat: string;
  footerCompatibility: string;
}

export const translations: Record<Language, Translations> = {
  es: {
    brandSubtitle: 'Transmisor OSC y DSP de Audio en Tiempo Real',
    startEngine: 'INICIAR MOTOR',
    stopEngine: 'DETENER MOTOR',
    audioInput: 'Entrada de Audio',
    targetHost: 'Host Destino',
    port: 'Puerto',
    sendRate: 'Tasa de Envío',
    presets: 'Preajustes',
    loadPresetPlaceholder: 'Cargar Preajuste...',
    savePresetBtn: 'Guardar',
    savePresetTitle: 'Guardar configuración actual como preajuste',
    presetInputPlaceholder: 'Nuevo nombre de preajuste (ej. DJ 4-Bandas, Tono Ambiental)...',
    confirmSave: 'Confirmar Guardado',
    cancel: 'Cancelar',
    language: 'Idioma',

    visualizerListening: 'Escuchando flujo de audio...',
    visualizerStopped: 'Motor detenido. Haz clic en INICIAR MOTOR arriba.',
    oscilloscope: 'OSCILOSCOPIO',
    fftSpectrum: 'ESPECTRO FFT (20Hz - 20kHz)',
    bandSub: 'SUB (20-80Hz)',
    bandLow: 'GRAVES (80-350Hz)',
    bandMid: 'MEDIOS (350-2.5kHz)',
    bandHigh: 'AGUDOS (2.5k-20kHz)',
    stereoPositionGauge: 'POSICIÓN ESTÉREO (I ↔ D)',
    stereoCenter: 'CENTRO',

    filterAll: 'Todos',
    filter1D: '1D Escalar',
    filter2D: '2D Vec2',
    filter3D: '3D Vec3',
    filter4D: '4D Vec4',
    activeModulesCount: (active: number, total: number) => `${active} de ${total} Activos`,
    enableAll: 'Activar Todos',
    disableAll: 'Desactivar Todos',

    shaperToggleTitle: 'Alternar Controles de Forma y Rango',
    gainLabel: (gain: string) => `Ganancia: ${gain}x`,
    attackLabel: (attack: string) => `Ataque: ${attack}`,
    decayLabel: (decay: string) => `Decaimiento: ${decay}`,
    minLabel: 'Mín',
    maxLabel: 'Máx',

    statusInitializing: 'Inicializando motor de audio...',
    statusReady: 'Listo',
    statusEngineActive: (ip: string, port: number) => `Motor activo: transmitiendo OSC a ${ip}:${port}`,
    statusEngineStopped: 'Motor detenido',
    statusStreaming: (ip: string, port: number, hz: number) => `Transmitiendo OSC a ${ip}:${port} a ${hz}Hz`,
    statusInitError: (err: string) => `Error al inicializar: ${err}`,
    statusEngineError: (err: string) => `Error del motor: ${err}`,
    statusSwitchedDevice: (device: string) => `Fuente de audio cambiada a: ${device}`,
    statusDeviceSwitchError: (err: string) => `Error al cambiar dispositivo: ${err}`,
    statusPresetSaved: (name: string) => `Preajuste guardado: "${name}"`,
    statusPresetSaveError: (err: string) => `Error al guardar preajuste: ${err}`,
    statusPresetLoaded: (name: string) => `Preajuste cargado: "${name}"`,
    statusPresetLoadError: (err: string) => `Error al cargar preajuste: ${err}`,

    footerTarget: 'Destino',
    footerFormat: 'Formato',
    footerCompatibility: 'Compatibilidad',
  },
  en: {
    brandSubtitle: 'Real-time Audio DSP & OSC Transmitter',
    startEngine: 'START ENGINE',
    stopEngine: 'STOP ENGINE',
    audioInput: 'Audio Input',
    targetHost: 'Target Host',
    port: 'Port',
    sendRate: 'Send Rate',
    presets: 'Presets',
    loadPresetPlaceholder: 'Load Preset...',
    savePresetBtn: 'Save',
    savePresetTitle: 'Save current setup as preset',
    presetInputPlaceholder: 'New preset name (e.g. DJ 4-Band, Ambient Pitch)...',
    confirmSave: 'Confirm Save',
    cancel: 'Cancel',
    language: 'Language',

    visualizerListening: 'Listening for audio stream...',
    visualizerStopped: 'Engine stopped. Click START ENGINE above.',
    oscilloscope: 'OSCILLOSCOPE',
    fftSpectrum: 'FFT SPECTRUM (20Hz - 20kHz)',
    bandSub: 'SUB (20-80Hz)',
    bandLow: 'LOW (80-350Hz)',
    bandMid: 'MID (350-2.5kHz)',
    bandHigh: 'HIGH (2.5k-20kHz)',
    stereoPositionGauge: 'STEREO POSITION (L ↔ R)',
    stereoCenter: 'CENTER',

    filterAll: 'All',
    filter1D: '1D Scalar',
    filter2D: '2D Vec2',
    filter3D: '3D Vec3',
    filter4D: '4D Vec4',
    activeModulesCount: (active: number, total: number) => `${active} of ${total} Active`,
    enableAll: 'Enable All',
    disableAll: 'Disable All',

    shaperToggleTitle: 'Toggle Shaping & Range Controls',
    gainLabel: (gain: string) => `Gain: ${gain}x`,
    attackLabel: (attack: string) => `Attack: ${attack}`,
    decayLabel: (decay: string) => `Decay: ${decay}`,
    minLabel: 'Min',
    maxLabel: 'Max',

    statusInitializing: 'Initializing audio engine...',
    statusReady: 'Ready',
    statusEngineActive: (ip: string, port: number) => `Engine active: streaming OSC to ${ip}:${port}`,
    statusEngineStopped: 'Engine stopped',
    statusStreaming: (ip: string, port: number, hz: number) => `Streaming OSC to ${ip}:${port} at ${hz}Hz`,
    statusInitError: (err: string) => `Error initializing: ${err}`,
    statusEngineError: (err: string) => `Engine error: ${err}`,
    statusSwitchedDevice: (device: string) => `Switched audio source to: ${device}`,
    statusDeviceSwitchError: (err: string) => `Device switch error: ${err}`,
    statusPresetSaved: (name: string) => `Preset saved: "${name}"`,
    statusPresetSaveError: (err: string) => `Preset save error: ${err}`,
    statusPresetLoaded: (name: string) => `Preset loaded: "${name}"`,
    statusPresetLoadError: (err: string) => `Preset load error: ${err}`,

    footerTarget: 'Target',
    footerFormat: 'Format',
    footerCompatibility: 'Compatibility',
  },
};

export const moduleNames: Record<Language, Record<string, string>> = {
  es: {
    rms: 'Volumen RMS',
    peak: 'Amplitud Pico',
    pitch: 'Detector de Tono',
    pitch_class: 'Clase de Tono / Octava',
    centroid: 'Centroide Espectral (Brillo)',
    flatness: 'Planitud Espectral (Ruido)',
    flux: 'Flujo Espectral (Transitorios)',
    zcr: 'Tasa de Cruce por Cero',
    onset: 'Golpe / Pulso de Inicio',
    stereo_pos: 'Posición Estéreo [-1, 1]',
    stereo: 'Amplitud Estéreo [I, D]',
    pitch_conf: 'Tono y Confianza [Tono, Conf]',
    loud_bright: 'Volumen y Brillo [RMS, Brillo]',
    eq3: 'EQ 3 Bandas [Graves, Medios, Agudos]',
    rgb: 'Audio a Color RGB [R, G, B]',
    eq4: 'Espectro 4 Bandas [Sub, Graves, Medios, Agudos]',
    composite4: 'Vector Compuesto [RMS, Tono, Centroide, Inicio]',
  },
  en: {
    rms: 'RMS Loudness',
    peak: 'Peak Amplitude',
    pitch: 'Pitch Tracker',
    pitch_class: 'Pitch Class / Octave',
    centroid: 'Spectral Centroid (Brightness)',
    flatness: 'Spectral Flatness (Noisiness)',
    flux: 'Spectral Flux (Transients)',
    zcr: 'Zero Crossing Rate',
    onset: 'Beat / Onset Pulse',
    stereo_pos: 'Stereo Position [-1, 1]',
    stereo: 'Stereo Amplitude [L, R]',
    pitch_conf: 'Pitch & Confidence [Pitch, Conf]',
    loud_bright: 'Loudness & Brightness [RMS, Bright]',
    eq3: '3-Band EQ [Bass, Mid, Treble]',
    rgb: 'Audio-to-RGB Color [R, G, B]',
    eq4: '4-Band Spectrum [Sub, Low, Mid, High]',
    composite4: 'Composite Vector [RMS, Pitch, Centroid, Onset]',
  },
};
