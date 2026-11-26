use crate::DomainError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

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
        match self.problem(clip) {
            Some(problem) => Err(DomainError::SampleRejected(problem)),
            None => Ok(()),
        }
    }

    fn problem(&self, clip: &AudioClip) -> Option<SampleProblem> {
        let secs = clip.duration_secs();
        if secs < self.min_duration_secs {
            return Some(SampleProblem::TooShort {
                secs,
                min_secs: self.min_duration_secs,
            });
        }
        if secs > self.max_duration_secs {
            return Some(SampleProblem::TooLong {
                secs,
                max_secs: self.max_duration_secs,
            });
        }
        if clip.sample_rate() < self.min_sample_rate {
            return Some(SampleProblem::LowSampleRate {
                hz: clip.sample_rate(),
                min_hz: self.min_sample_rate,
            });
        }
        if clip.clipping_ratio() > self.max_clipping_ratio {
            return Some(SampleProblem::Clipped);
        }
        if clip.rms() < self.min_rms {
            return Some(SampleProblem::NearlySilent);
        }
        None
    }
}

/// Why a recording cannot be used as voice material: the file is not usable
/// audio, or the audio breaks the [`SampleQualityPolicy`]. Typed so each
/// presentation can explain it in the user's language; `Display` is the
/// English text.
#[derive(Debug, Clone, Copy, PartialEq, Error)]
pub enum SampleProblem {
    #[error("audio too short ({secs:.1}s); minimum {min_secs:.1}s")]
    TooShort { secs: f32, min_secs: f32 },
    #[error("audio too long ({secs:.1}s); maximum {max_secs:.1}s — split it into parts")]
    TooLong { secs: f32, max_secs: f32 },
    #[error("sample rate {hz} Hz below the minimum {min_hz} Hz")]
    LowSampleRate { hz: u32, min_hz: u32 },
    #[error("clipped audio; record at a lower volume")]
    Clipped,
    #[error("audio is nearly silent")]
    NearlySilent,
    /// The bytes match none of the formats the app reads.
    #[error("This audio format is not supported. Use a WAV, FLAC, MP3, M4A or OGG file.")]
    UnsupportedFormat,
    /// The container holds no audio track (e.g. only video or subtitles).
    #[error("This file has no audio. Use a WAV, FLAC, MP3, M4A or OGG file.")]
    NoAudioTrack,
    /// The container is supported but its codec is not (e.g. Opus, ALAC).
    #[error("This audio format is not supported. Convert the file to WAV.")]
    UnsupportedCodec,
    /// The container is supported but its content is damaged or cut short.
    #[error("This audio file could not be read; it may be damaged. Convert the file to WAV.")]
    Unreadable,
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

    fn problem(p: &SampleQualityPolicy, clip: &AudioClip) -> SampleProblem {
        match p.validate(clip) {
            Err(DomainError::SampleRejected(problem)) => problem,
            other => panic!("expected a rejected sample, got {other:?}"),
        }
    }

    #[test]
    fn rejects_short_long_silent_clipped_and_low_rate() {
        let p = SampleQualityPolicy::default();
        assert_eq!(
            problem(&p, &tone(0.5, 16_000, 0.3)),
            SampleProblem::TooShort {
                secs: 0.5,
                min_secs: 2.0
            }
        );
        assert_eq!(
            problem(&p, &tone(181.0, 16_000, 0.3)),
            SampleProblem::TooLong {
                secs: 181.0,
                max_secs: 180.0
            }
        );
        assert_eq!(
            problem(&p, &tone(3.0, 8_000, 0.3)),
            SampleProblem::LowSampleRate {
                hz: 8_000,
                min_hz: 16_000
            }
        );
        let clipped = AudioClip::new(16_000, vec![1.0; 48_000]).unwrap();
        assert_eq!(problem(&p, &clipped), SampleProblem::Clipped);
        assert_eq!(
            problem(&p, &tone(3.0, 16_000, 0.0)),
            SampleProblem::NearlySilent
        );
    }

    #[test]
    fn every_problem_explains_itself_in_english() {
        let cases = [
            (
                SampleProblem::TooShort {
                    secs: 0.5,
                    min_secs: 2.0,
                },
                "audio too short (0.5s); minimum 2.0s",
            ),
            (
                SampleProblem::TooLong {
                    secs: 181.0,
                    max_secs: 180.0,
                },
                "audio too long (181.0s); maximum 180.0s — split it into parts",
            ),
            (
                SampleProblem::LowSampleRate {
                    hz: 8_000,
                    min_hz: 16_000,
                },
                "sample rate 8000 Hz below the minimum 16000 Hz",
            ),
            (
                SampleProblem::Clipped,
                "clipped audio; record at a lower volume",
            ),
            (SampleProblem::NearlySilent, "audio is nearly silent"),
            (
                SampleProblem::UnsupportedFormat,
                "This audio format is not supported. Use a WAV, FLAC, MP3, M4A or OGG file.",
            ),
            (
                SampleProblem::NoAudioTrack,
                "This file has no audio. Use a WAV, FLAC, MP3, M4A or OGG file.",
            ),
            (
                SampleProblem::UnsupportedCodec,
                "This audio format is not supported. Convert the file to WAV.",
            ),
            (
                SampleProblem::Unreadable,
                "This audio file could not be read; it may be damaged. Convert the file to WAV.",
            ),
        ];
        for (problem, text) in cases {
            assert_eq!(problem.to_string(), text);
        }
        assert_eq!(
            DomainError::SampleRejected(SampleProblem::NearlySilent).to_string(),
            "audio sample rejected: audio is nearly silent"
        );
    }
}
