use std::collections::HashMap;

#[derive(Default)]
pub struct ValueShaper {
    // module_id -> smoothed vector of floats
    state: HashMap<String, Vec<f32>>,
}

impl ValueShaper {
    pub fn new() -> Self {
        Self {
            state: HashMap::new(),
        }
    }

    pub fn shape(
        &mut self,
        module_id: &str,
        raw_values: &[f32],
        gain: f32,
        attack: f32,
        decay: f32,
        min_val: f32,
        max_val: f32,
    ) -> Vec<f32> {
        let prev = self.state.entry(module_id.to_string()).or_insert_with(|| vec![0.0; raw_values.len()]);
        if prev.len() != raw_values.len() {
            *prev = vec![0.0; raw_values.len()];
        }

        let attack_coeff = attack.clamp(0.01, 1.0);
        let decay_coeff = decay.clamp(0.01, 1.0);

        let mut output = Vec::with_capacity(raw_values.len());

        for (i, &raw) in raw_values.iter().enumerate() {
            let prev_val = prev[i];

            let (smoothed, remapped) = if module_id == "stereo_pos" {
                // Bipolar [-1.0, 1.0] signal centered at 0.0
                let gained = (raw * gain).clamp(-1.0, 1.0);
                let coeff = if gained.abs() > prev_val.abs() {
                    attack_coeff
                } else {
                    decay_coeff
                };
                let s = prev_val + (gained - prev_val) * coeff;
                // Remap normalized -1.0..1.0 to min_val..max_val
                let r = min_val + (s + 1.0) * 0.5 * (max_val - min_val);
                (s, r)
            } else {
                let gained = (raw * gain).max(0.0);
                let s = if gained > prev_val {
                    prev_val + (gained - prev_val) * attack_coeff
                } else {
                    prev_val + (gained - prev_val) * decay_coeff
                };
                // Remap 0.0..1.0 to min_val..max_val
                let r = min_val + s * (max_val - min_val);
                (s, r)
            };

            prev[i] = smoothed;
            output.push(if remapped.is_finite() { remapped } else { 0.0 });
        }

        output
    }
}
