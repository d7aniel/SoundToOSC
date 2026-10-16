use realfft::{RealFftPlanner, RealToComplex};
use std::sync::Arc;

pub const FFT_SIZE: usize = 2048;

pub struct AudioAnalysisEngine {
    fft_plan: Arc<dyn RealToComplex<f32>>,
    prev_spectrum: Vec<f32>,
    onset_pulse: f32,
    window: Vec<f32>,
}

pub struct AnalysisResult {
    pub rms: f32,
    pub peak: f32,
    pub zcr: f32,
    pub pitch: f32,       // Normalized 0.0 - 1.0 (MIDI note 21..108 mapped)
    pub pitch_hz: f32,    // Raw Hz
    pub pitch_conf: f32,  // 0.0 - 1.0
    pub pitch_class: f32, // 0.0 - 1.0 (12 semitones)
    pub centroid: f32,    // Normalized 0.0 - 1.0
    pub flatness: f32,    // 0.0 - 1.0
    pub flux: f32,        // Normalized 0.0 - 1.0
    pub onset: f32,       // 0.0 - 1.0 pulse
    pub stereo: [f32; 2], // [Left RMS, Right RMS]
    pub stereo_pos: f32,  // -1.0 (Full Left) to +1.0 (Full Right), 0.0 Center
    pub eq3: [f32; 3],    // [Bass, Mid, Treble]
    pub rgb: [f32; 3],    // [R, G, B]
    pub eq4: [f32; 4],    // [Sub, Low, Mid, High]
    pub waveform: Vec<f32>, // 128 points for UI
    pub spectrum_bins: Vec<f32>, // 64 log bins for UI
}

impl AudioAnalysisEngine {
    pub fn new() -> Self {
        let mut planner = RealFftPlanner::<f32>::new();
        let fft_plan = planner.plan_fft_forward(FFT_SIZE);

        // Precompute Hann window
        let mut window = vec![0.0f32; FFT_SIZE];
        let two_pi = std::f32::consts::PI * 2.0;
        for i in 0..FFT_SIZE {
            window[i] = 0.5 * (1.0 - (two_pi * i as f32 / (FFT_SIZE as f32 - 1.0)).cos());
        }

        let spectrum_len = FFT_SIZE / 2 + 1;

        Self {
            fft_plan,
            prev_spectrum: vec![0.0; spectrum_len],
            onset_pulse: 0.0,
            window,
        }
    }

    pub fn analyze(
        &mut self,
        mono_samples: &[f32],
        left_samples: Option<&[f32]>,
        right_samples: Option<&[f32]>,
        sample_rate: f32,
    ) -> AnalysisResult {
        let n = mono_samples.len();
        if n == 0 {
            return self.empty_result();
        }

        // 1. RMS and Peak
        let mut sum_sq = 0.0f32;
        let mut peak = 0.0f32;
        for &s in mono_samples {
            let abs = s.abs();
            if abs > peak {
                peak = abs;
            }
            sum_sq += s * s;
        }
        let rms = (sum_sq / n as f32).sqrt().min(1.0);

        // 2. Zero Crossing Rate (ZCR)
        let mut zcr_count = 0;
        for i in 1..n {
            if (mono_samples[i] >= 0.0 && mono_samples[i - 1] < 0.0)
                || (mono_samples[i] < 0.0 && mono_samples[i - 1] >= 0.0)
            {
                zcr_count += 1;
            }
        }
        let zcr = (zcr_count as f32 / (n as f32 - 1.0)).min(1.0);

        // 3. Stereo RMS & Position (-1.0 to 1.0)
        let (stereo, stereo_pos) = match (left_samples, right_samples) {
            (Some(l), Some(r)) => {
                let l_rms = (l.iter().map(|&x| x * x).sum::<f32>() / l.len().max(1) as f32).sqrt().min(1.0);
                let r_rms = (r.iter().map(|&x| x * x).sum::<f32>() / r.len().max(1) as f32).sqrt().min(1.0);
                let sum = l_rms + r_rms;
                let diff = r_rms - l_rms;
                let pos = if sum > 1e-4 {
                    (diff / sum).clamp(-1.0, 1.0)
                } else {
                    0.0
                };
                ([l_rms, r_rms], pos)
            }
            _ => ([rms, rms], 0.0),
        };

        // 4. Pitch Detection (Autocorrelation / YIN)
        let (pitch_hz, pitch_conf) = self.detect_pitch(mono_samples, sample_rate);
        let (pitch_norm, pitch_class) = if pitch_hz > 20.0 {
            // MIDI note = 69 + 12 * log2(f / 440)
            let midi = 69.0 + 12.0 * (pitch_hz / 440.0).log2();
            let norm = ((midi - 21.0) / 87.0).clamp(0.0, 1.0); // A0 (21) to C8 (108)
            let class = (midi.rem_euclid(12.0)) / 12.0;
            (norm, class)
        } else {
            (0.0, 0.0)
        };

        // 5. FFT Analysis
        let spectrum_len = FFT_SIZE / 2 + 1;
        let mut in_buf = vec![0.0f32; FFT_SIZE];
        let copy_len = n.min(FFT_SIZE);
        let offset = if n > FFT_SIZE { n - FFT_SIZE } else { 0 };

        for i in 0..copy_len {
            in_buf[i] = mono_samples[offset + i] * self.window[i];
        }

        let mut out_spectrum = self.fft_plan.make_output_vec();
        let _ = self.fft_plan.process(&mut in_buf, &mut out_spectrum);

        // Magnitude spectrum normalized
        let mut mags = Vec::with_capacity(spectrum_len);
        let norm_factor = 2.0 / (FFT_SIZE as f32);
        for c in &out_spectrum {
            mags.push(c.norm() * norm_factor);
        }

        // 6. Spectral Centroid
        let nyquist = sample_rate / 2.0;
        let bin_hz = sample_rate / FFT_SIZE as f32;
        let mut num = 0.0f32;
        let mut den = 0.0f32;
        for (k, &m) in mags.iter().enumerate() {
            let freq = k as f32 * bin_hz;
            num += freq * m;
            den += m;
        }
        let centroid = if den > 1e-6 {
            (num / den / (nyquist * 0.75)).clamp(0.0, 1.0)
        } else {
            0.0
        };

        // 7. Spectral Flatness (Wiener entropy)
        let mut log_sum = 0.0f32;
        let mut arith_sum = 0.0f32;
        let mut count = 0;
        let eps = 1e-8f32;
        // consider bins above DC up to 10kHz
        let max_bin = ((10000.0 / bin_hz) as usize).min(mags.len());
        for &m in &mags[1..max_bin] {
            let p = m * m + eps;
            log_sum += p.ln();
            arith_sum += p;
            count += 1;
        }
        let flatness = if count > 0 && arith_sum > 1e-6 {
            let geom_mean = (log_sum / count as f32).exp();
            let arith_mean = arith_sum / count as f32;
            (geom_mean / arith_mean).clamp(0.0, 1.0)
        } else {
            0.0
        };

        // 8. Spectral Flux & Onset
        let mut flux_sum = 0.0f32;
        for i in 0..spectrum_len {
            let diff = mags[i] - self.prev_spectrum[i];
            if diff > 0.0 {
                flux_sum += diff;
            }
        }
        self.prev_spectrum.copy_from_slice(&mags);
        let flux = (flux_sum * 5.0).clamp(0.0, 1.0);

        // Onset detector with dynamic trigger and exponential decay
        if flux > 0.35 && flux > self.onset_pulse {
            self.onset_pulse = 1.0;
        } else {
            self.onset_pulse = (self.onset_pulse * 0.82).max(0.0);
        }
        let onset = self.onset_pulse;

        // 9. Band energies (EQ3 and EQ4)
        let get_band_energy = |min_hz: f32, max_hz: f32| -> f32 {
            let start = ((min_hz / bin_hz) as usize).clamp(0, spectrum_len - 1);
            let end = ((max_hz / bin_hz) as usize).clamp(start + 1, spectrum_len);
            let sum: f32 = mags[start..end].iter().sum();
            (sum * 2.5).min(1.0)
        };

        // EQ3: Bass (20-250), Mid (250-2500), Treble (2500-18000)
        let bass = get_band_energy(20.0, 250.0);
        let mid = get_band_energy(250.0, 2500.0);
        let treble = get_band_energy(2500.0, 18000.0);
        let eq3 = [bass, mid, treble];

        // EQ4: Sub (20-80), Low (80-350), Mid (350-2500), High (2500-18000)
        let sub = get_band_energy(20.0, 80.0);
        let low = get_band_energy(80.0, 350.0);
        let eq4 = [sub, low, mid, treble];

        // RGB Color Mapping
        let r = (bass * 0.7 + flatness * 0.3).clamp(0.0, 1.0);
        let g = (mid * 0.6 + centroid * 0.4).clamp(0.0, 1.0);
        let b = (treble * 0.6 + pitch_norm * 0.4).clamp(0.0, 1.0);
        let rgb = [r, g, b];

        // 10. Waveform and Spectrum for UI Display
        let waveform = self.downsample_waveform(mono_samples, 128);
        let spectrum_bins = self.log_spectrum_bins(&mags, bin_hz, 64);

        AnalysisResult {
            rms,
            peak,
            zcr,
            pitch: pitch_norm,
            pitch_hz,
            pitch_conf,
            pitch_class,
            centroid,
            flatness,
            flux,
            onset,
            stereo,
            stereo_pos,
            eq3,
            rgb,
            eq4,
            waveform,
            spectrum_bins,
        }
    }

    fn detect_pitch(&self, samples: &[f32], sample_rate: f32) -> (f32, f32) {
        // YIN-based pitch estimator
        let min_freq = 50.0;
        let max_freq = 1200.0;
        let min_period = (sample_rate / max_freq) as usize;
        let max_period = (sample_rate / min_freq) as usize;

        let len = samples.len() / 2;
        if len <= max_period {
            return (0.0, 0.0);
        }

        // Difference function
        let mut diff = vec![0.0f32; max_period + 1];
        for tau in min_period..=max_period {
            let mut sum = 0.0f32;
            for j in 0..len {
                let delta = samples[j] - samples[j + tau];
                sum += delta * delta;
            }
            diff[tau] = sum;
        }

        // Cumulative mean normalized difference
        let mut cmnd = vec![0.0f32; max_period + 1];
        cmnd[0] = 1.0;
        let mut running_sum = 0.0f32;
        for tau in 1..=max_period {
            running_sum += diff[tau];
            cmnd[tau] = if running_sum > 1e-6 {
                (diff[tau] * tau as f32) / running_sum
            } else {
                1.0
            };
        }

        // Find first minimum below threshold
        let threshold = 0.20;
        let mut best_tau = 0;
        for tau in min_period..max_period {
            if cmnd[tau] < threshold {
                let mut local_min = tau;
                while local_min + 1 <= max_period && cmnd[local_min + 1] < cmnd[local_min] {
                    local_min += 1;
                }
                best_tau = local_min;
                break;
            }
        }

        if best_tau == 0 {
            // Fallback to absolute minimum if within range
            let mut min_val = 1.0f32;
            for tau in min_period..=max_period {
                if cmnd[tau] < min_val {
                    min_val = cmnd[tau];
                    best_tau = tau;
                }
            }
            if min_val > 0.45 {
                return (0.0, 0.0);
            }
        }

        if best_tau == 0 {
            return (0.0, 0.0);
        }

        // Parabolic interpolation for fine tuning
        let s0 = cmnd[best_tau.saturating_sub(1)];
        let s1 = cmnd[best_tau];
        let s2 = cmnd[(best_tau + 1).min(max_period)];
        let delta = if (s2 - 2.0 * s1 + s0).abs() > 1e-6 {
            (s0 - s2) / (2.0 * (s0 - 2.0 * s1 + s2))
        } else {
            0.0
        };

        let fine_tau = best_tau as f32 + delta;
        let freq = sample_rate / fine_tau;
        let conf = (1.0 - cmnd[best_tau]).clamp(0.0, 1.0);

        (freq, conf)
    }

    fn downsample_waveform(&self, samples: &[f32], target_len: usize) -> Vec<f32> {
        let mut out = Vec::with_capacity(target_len);
        let step = samples.len() as f32 / target_len as f32;
        for i in 0..target_len {
            let idx = (i as f32 * step) as usize;
            out.push(samples.get(idx).copied().unwrap_or(0.0));
        }
        out
    }

    fn log_spectrum_bins(&self, mags: &[f32], bin_hz: f32, target_bins: usize) -> Vec<f32> {
        let min_freq = 20.0f32;
        let max_freq = 20000.0f32;
        let mut bins = Vec::with_capacity(target_bins);

        for i in 0..target_bins {
            let f_start = min_freq * (max_freq / min_freq).powf(i as f32 / target_bins as f32);
            let f_end = min_freq * (max_freq / min_freq).powf((i + 1) as f32 / target_bins as f32);

            let idx_start = ((f_start / bin_hz) as usize).clamp(0, mags.len() - 1);
            let idx_end = ((f_end / bin_hz) as usize).clamp(idx_start + 1, mags.len());

            let mut max_val = 0.0f32;
            for &m in &mags[idx_start..idx_end] {
                if m > max_val {
                    max_val = m;
                }
            }
            bins.push((max_val * 4.0).min(1.0));
        }

        bins
    }

    fn empty_result(&self) -> AnalysisResult {
        AnalysisResult {
            rms: 0.0,
            peak: 0.0,
            zcr: 0.0,
            pitch: 0.0,
            pitch_hz: 0.0,
            pitch_conf: 0.0,
            pitch_class: 0.0,
            centroid: 0.0,
            flatness: 0.0,
            flux: 0.0,
            onset: 0.0,
            stereo: [0.0, 0.0],
            stereo_pos: 0.0,
            eq3: [0.0, 0.0, 0.0],
            rgb: [0.0, 0.0, 0.0],
            eq4: [0.0, 0.0, 0.0, 0.0],
            waveform: vec![0.0; 128],
            spectrum_bins: vec![0.0; 64],
        }
    }
}
