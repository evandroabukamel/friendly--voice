//! Acoustic analysis with simple DSP: energy, pauses and pitch (autocorrelation).
use friendly_voice_application::ports::AcousticAnalyzer;
use friendly_voice_domain::audio::{AcousticFeatures, AudioClip};

#[derive(Debug, Clone, Copy)]
pub struct DspAcousticAnalyzer {
    pub frame_ms: f32,
    pub hop_ms: f32,
    pub min_pitch_hz: f32,
    pub max_pitch_hz: f32,
}

impl Default for DspAcousticAnalyzer {
    fn default() -> Self {
        Self {
            frame_ms: 40.0,
            hop_ms: 20.0,
            min_pitch_hz: 70.0,
            max_pitch_hz: 450.0,
        }
    }
}

fn percentile(sorted: &[f32], p: f32) -> f32 {
    if sorted.is_empty() {
        return 0.0;
    }
    let idx = ((sorted.len() - 1) as f32 * p).round() as usize;
    sorted[idx]
}

impl DspAcousticAnalyzer {
    /// Estimates the F0 of a frame through normalized autocorrelation.
    fn pitch(&self, frame: &[f32], sr: f32) -> Option<f32> {
        let min_lag = (sr / self.max_pitch_hz) as usize;
        let max_lag = ((sr / self.min_pitch_hz) as usize).min(frame.len() - 1);
        let energy: f32 = frame.iter().map(|x| x * x).sum();
        if energy <= f32::EPSILON || min_lag >= max_lag {
            return None;
        }
        let (mut best_lag, mut best) = (0, 0.0f32);
        for lag in min_lag..=max_lag {
            let r: f32 = frame[..frame.len() - lag]
                .iter()
                .zip(&frame[lag..])
                .map(|(a, b)| a * b)
                .sum();
            if r > best {
                best = r;
                best_lag = lag;
            }
        }
        (best_lag > 0 && best / energy > 0.3).then(|| sr / best_lag as f32)
    }
}

impl AcousticAnalyzer for DspAcousticAnalyzer {
    fn analyze(&self, clip: &AudioClip) -> AcousticFeatures {
        let sr = clip.sample_rate() as f32;
        let frame = ((self.frame_ms / 1000.0) * sr) as usize;
        let hop = ((self.hop_ms / 1000.0) * sr).max(1.0) as usize;
        let s = clip.samples();
        if s.len() < frame || frame == 0 {
            return AcousticFeatures::default();
        }
        let frames: Vec<&[f32]> = (0..=(s.len() - frame))
            .step_by(hop)
            .map(|i| &s[i..i + frame])
            .collect();
        let rms: Vec<f32> = frames
            .iter()
            .map(|f| (f.iter().map(|x| x * x).sum::<f32>() / f.len() as f32).sqrt())
            .collect();
        let mut sorted = rms.clone();
        sorted.sort_by(f32::total_cmp);
        let threshold = (percentile(&sorted, 0.95) * 0.1).max(1e-4);
        let speech: Vec<usize> = (0..frames.len()).filter(|&i| rms[i] >= threshold).collect();
        let pause_ratio = 1.0 - speech.len() as f32 / frames.len() as f32;

        let mut pitches: Vec<f32> = speech
            .iter()
            .filter_map(|&i| self.pitch(frames[i], sr))
            .collect();
        pitches.sort_by(f32::total_cmp);
        let mean_pitch_hz = if pitches.is_empty() {
            0.0
        } else {
            pitches.iter().sum::<f32>() / pitches.len() as f32
        };
        AcousticFeatures {
            mean_pitch_hz,
            pitch_spread_hz: percentile(&pitches, 0.9) - percentile(&pitches, 0.1),
            rms_energy: clip.rms(),
            pause_ratio,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn estimates_pitch_and_pauses() {
        let sr = 16_000;
        let mut s: Vec<f32> = (0..sr)
            .map(|i| 0.4 * (i as f32 * 200.0 * std::f32::consts::TAU / sr as f32).sin())
            .collect();
        s.extend(std::iter::repeat_n(0.0, sr as usize)); // 1s of silence
        let f = DspAcousticAnalyzer::default().analyze(&AudioClip::new(sr, s).unwrap());
        assert!((f.mean_pitch_hz - 200.0).abs() < 5.0, "{}", f.mean_pitch_hz);
        assert!((f.pause_ratio - 0.5).abs() < 0.05, "{}", f.pause_ratio);
    }
}
