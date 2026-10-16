# SoundToOSC 🎵⚡

**SoundToOSC** is a cross-platform desktop application built with **Rust** and **Tauri v2** with a **React + TypeScript** webview UI. It performs uncompressed, real-time native audio analysis and broadcasts the extracted features as **OSC (Open Sound Control)** packets to port `9000` for consumption by the **Andragon** shader visual editor.

---

## 🚀 Key Highlights

* **Direct Native Microphone & System Loopback**: Audio capture bypasses webview compression/AGC completely using Rust (`cpal`), reading pure raw PCM $f32$ audio from physical microphones, audio interfaces, and Windows WASAPI loopback.
* **Non-Blocking OSC Client**: The UDP socket binds to ephemeral port `0.0.0.0:0` and transmits directly to `127.0.0.1:9000`. It **never binds to port 9000**, leaving port 9000 completely free for Andragon to listen on.
* **Andragon OSC Protocol Compatible**: Formats all payloads into numeric arrays `[f32; 1..4]` strictly matching [`ConnectionsManager.ts`](file:///c:/Users/aleja/Documents/Emmelab/andragon%20refactor/andragon/src/sys/connections/ConnectionsManager.ts) and [`UniformOscManager.ts`](file:///c:/Users/aleja/Documents/Emmelab/andragon%20refactor/andragon/src/engine/render/uniforms/UniformOscManager.ts) for `FloatUniform` and `VectorUniform` (`vec2`, `vec3`, `vec4`).
* **Independent Power Toggles**: Any module can be turned on or off dynamically with zero interference to other modules.
* **Per-Module Shaping**: Adjustable Gain, Attack/Decay smoothing filters, and Min/Max output range remapping.
* **Configurable Transmission Rate**: High-speed slider (10 Hz – 120 Hz, default 60 Hz) to deliver smooth 60 FPS shader visuals without flooding network sockets.
* **Session Auto-Persistence & Named Presets**: Automatically remembers all toggles, addresses, gain, and smoothing settings on restart, with support for saving named JSON configuration profiles.

---

## 📊 Included Analysis Modules

### 1D Modules (Scalar / FloatUniform)
* **RMS Loudness** (`/audio/rms`): Root-mean-square perceived volume (0.0 – 1.0).
* **Peak Amplitude** (`/audio/peak`): Instantaneous peak level for clip and flash detection.
* **Pitch Tracker** (`/audio/pitch`): YIN autocorrelation monophonic pitch detection normalized to 0.0 – 1.0.
* **Pitch Class / Note** (`/audio/note`): Semitone chroma (0 – 11) for chromatic note reactivity.
* **Spectral Centroid** (`/audio/centroid`): Spectral center of mass indicating perceived brightness/sharpness.
* **Spectral Flatness** (`/audio/flatness`): Wiener entropy (0.0 pure tone to 1.0 white noise).
* **Spectral Flux** (`/audio/flux`): Rate of spectral energy change for transient detection.
* **Zero Crossing Rate (ZCR)** (`/audio/zcr`): Count of waveform sign transitions (high frequency noise, snares, speech).
* **Beat / Onset Pulse** (`/audio/onset`): Fast attack impulse triggered on musical transients with exponential decay.
* **Stereo Position** (`/audio/position`): Spatial pan position from -1.0 (full left) to +1.0 (full right) based on the relative difference between L and R channels.

### 2D Modules (Vec2 / VectorUniform)
* **Stereo Amplitude** (`/audio/stereo`): `[left_rms, right_rms]` for stereo balance and spatial panning.
* **Pitch & Confidence** (`/audio/pitch_conf`): `[pitch_norm, confidence_0_to_1]`.
* **Loudness & Brightness** (`/audio/loud_bright`): `[rms, spectral_centroid]`.

### 3D Modules (Vec3 / VectorUniform)
* **3-Band EQ Energy** (`/audio/eq3`): `[bass, mid, treble]` (20-250Hz, 250-2500Hz, 2.5k-20kHz).
* **Audio-to-RGB Color** (`/audio/rgb`): `[r, g, b]` color mapping derived from energy, centroid, and flatness.

### 4D Modules (Vec4 / VectorUniform)
* **4-Band Spectrum** (`/audio/eq4`): `[sub, low, mid, high]` (20-80Hz, 80-350Hz, 350-2.5kHz, 2.5k-20kHz).
* **Composite Audio Vector** (`/audio/vec4`): `[rms, pitch, centroid, onset]`.

---

## 🛠️ Development & Running

### Prerequisites
* Rust (stable, 2021 edition)
* Node.js (v20+) & npm

### Commands
```bash
# Run local desktop development (hot-reloads both frontend and backend)
npm run tauri dev

# Check Rust backend compilation
cargo check --manifest-path src-tauri/Cargo.toml

# Build production bundle
npm run tauri build
```
