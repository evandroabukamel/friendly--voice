use crate::DomainError;
use serde::{Deserialize, Serialize};

/// Mono audio normalized to [-1.0, 1.0].
#[derive(Debug, Clone, PartialEq)]
pub struct AudioClip {
    sample_rate: u32,
    samples: Vec<f32>,
}

impl AudioClip {
    pub fn new(sample_rate: u32, samples: Vec<f32>) -> Result<Self, DomainError> {
        if sample_rate == 0 {
            return Err(DomainError::invalid(
                "sample_rate",
                "must be greater than zero",
            ));
        }
        Ok(Self {
            sample_rate,
            samples,
        })
    }
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
    pub fn samples(&self) -> &[f32] {
        &self.samples
    }
    pub fn duration_secs(&self) -> f32 {
        self.samples.len() as f32 / self.sample_rate as f32
    }
    pub fn rms(&self) -> f32 {
        if self.samples.is_empty() {
            return 0.0;
        }
        (self.samples.iter().map(|s| s * s).sum::<f32>() / self.samples.len() as f32).sqrt()
    }
    pub fn clipping_ratio(&self) -> f32 {
        if self.samples.is_empty() {
            return 0.0;
        }
        self.samples.iter().filter(|s| s.abs() >= 0.999).count() as f32 / self.samples.len() as f32
    }
}

/// Acoustic measurements extracted from a sample (timbre/prosody).
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct AcousticFeatures {
    /// Mean fundamental frequency (Hz) of the voiced parts.
    pub mean_pitch_hz: f32,
    /// Pitch spread (p90 - p10): how "sing-song" the speech is.
    pub pitch_spread_hz: f32,
    pub rms_energy: f32,
    /// Fraction of time in silence/pauses.
    pub pause_ratio: f32,
}

/// Rules to accept a sample as training material.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SampleQualityPolicy {
    pub min_duration_secs: f32,
    pub max_duration_secs: f32,
    pub min_sample_rate: u32,
    pub max_clipping_ratio: f32,
    pub min_rms: f32,
}

impl Default for SampleQualityPolicy {
    fn default() -> Self {
        Self {
            min_duration_secs: 2.0,
            max_duration_secs: 180.0,
            min_sample_rate: 16_000,
            max_clipping_ratio: 0.01,
            min_rms: 0.005,
        }
    }
}

impl SampleQualityPolicy {
    pub fn validate(&self, clip: &AudioClip) -> Result<(), DomainError> {
        let d = clip.duration_secs();
        if d < self.min_duration_secs {
            return Err(DomainError::SampleRejected(format!(
                "audio too short ({d:.1}s); minimum {:.1}s",
                self.min_duration_secs
            )));
        }
        if d > self.max_duration_secs {
            return Err(DomainError::SampleRejected(format!(
                "audio too long ({d:.1}s); maximum {:.1}s — split it into parts",
                self.max_duration_secs
            )));
        }
        if clip.sample_rate() < self.min_sample_rate {
            return Err(DomainError::SampleRejected(format!(
                "sample rate {} Hz below the minimum {} Hz",
                clip.sample_rate(),
                self.min_sample_rate
            )));
        }
        if clip.clipping_ratio() > self.max_clipping_ratio {
            return Err(DomainError::SampleRejected(
                "clipped audio; record at a lower volume".into(),
            ));
        }
        if clip.rms() < self.min_rms {
            return Err(DomainError::SampleRejected("audio is nearly silent".into()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tone(secs: f32, rate: u32, amp: f32) -> AudioClip {
        let n = (secs * rate as f32) as usize;
        let s = (0..n)
            .map(|i| amp * (i as f32 * 220.0 * std::f32::consts::TAU / rate as f32).sin())
            .collect();
        AudioClip::new(rate, s).unwrap()
    }

    #[test]
    fn accepts_good_sample() {
        assert!(SampleQualityPolicy::default()
            .validate(&tone(3.0, 16_000, 0.3))
            .is_ok());
    }

    #[test]
    fn rejects_short_silent_clipped_and_low_rate() {
        let p = SampleQualityPolicy::default();
        assert!(p.validate(&tone(0.5, 16_000, 0.3)).is_err());
        assert!(p.validate(&tone(3.0, 16_000, 0.0)).is_err());
        assert!(p.validate(&tone(3.0, 8_000, 0.3)).is_err());
        let clipped = AudioClip::new(16_000, vec![1.0; 48_000]).unwrap();
        assert!(p.validate(&clipped).is_err());
    }
}
