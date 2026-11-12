//! Voice samples, cloned voice profile and speech script.
use crate::accent::AccentProfile;
use crate::audio::AcousticFeatures;
use crate::ids::{SampleId, SpeakerId};
use crate::phonetics::{Phoneme, PhoneticUtterance};
use crate::tone::VoiceTone;
use crate::DomainError;
use serde::{Deserialize, Serialize};

/// Audio sample uploaded by the user, already analyzed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceSample {
    pub id: SampleId,
    pub speaker_id: SpeakerId,
    /// Locator of the original audio in storage.
    pub audio_locator: String,
    pub duration_secs: f32,
    /// What was said in the recording.
    pub transcript: String,
    /// Citation pronunciation of the transcript.
    pub canonical: PhoneticUtterance,
    /// Phonemes actually heard (phone recognizer or annotation).
    pub observed: Option<Vec<Phoneme>>,
    pub features: AcousticFeatures,
}

/// Prosodic traits (melody and rhythm) of the person.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct ProsodyProfile {
    pub mean_pitch_hz: f32,
    pub pitch_spread_hz: f32,
    pub syllables_per_second: f32,
    pub pause_ratio: f32,
}

impl ProsodyProfile {
    /// Aggregates the samples weighting by duration.
    pub fn aggregate(samples: &[VoiceSample]) -> Self {
        let total: f32 = samples.iter().map(|s| s.duration_secs).sum();
        if total <= 0.0 {
            return Self::default();
        }
        let wavg = |f: &dyn Fn(&VoiceSample) -> f32| {
            samples.iter().map(|s| f(s) * s.duration_secs).sum::<f32>() / total
        };
        let speech_secs: f32 = samples
            .iter()
            .map(|s| s.duration_secs * (1.0 - s.features.pause_ratio))
            .sum();
        let syllables: usize = samples.iter().map(|s| s.canonical.syllable_count()).sum();
        Self {
            mean_pitch_hz: wavg(&|s| s.features.mean_pitch_hz),
            pitch_spread_hz: wavg(&|s| s.features.pitch_spread_hz),
            syllables_per_second: if speech_secs > 0.0 {
                syllables as f32 / speech_secs
            } else {
                0.0
            },
            pause_ratio: wavg(&|s| s.features.pause_ratio),
        }
    }
}

/// Minimum requirements to generate a voice.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EnrollmentPolicy {
    pub min_samples: usize,
    pub min_total_secs: f32,
}

impl Default for EnrollmentPolicy {
    fn default() -> Self {
        Self {
            min_samples: 1,
            min_total_secs: 10.0,
        }
    }
}

impl EnrollmentPolicy {
    pub fn check(&self, samples: &[VoiceSample]) -> Result<(), DomainError> {
        let total: f32 = samples.iter().map(|s| s.duration_secs).sum();
        if samples.len() < self.min_samples || total < self.min_total_secs {
            return Err(DomainError::InsufficientMaterial(format!(
                "{} sample(s), {total:.1}s in total; at least {} sample(s) and {:.0}s are required",
                samples.len(),
                self.min_samples,
                self.min_total_secs
            )));
        }
        Ok(())
    }
}

/// Voice generated for a person: timbre (in the synthesis engine) + accent + prosody.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceProfile {
    pub speaker_id: SpeakerId,
    /// Voice identifier in the cloning engine.
    pub engine_voice_id: String,
    pub accent: AccentProfile,
    pub prosody: ProsodyProfile,
    pub sample_count: usize,
    pub total_audio_secs: f32,
}

/// Script handed to the synthesizer: text + accented pronunciation +
/// prosody (already coloured by the chosen tone).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpeechScript {
    pub text: String,
    pub pronunciation: PhoneticUtterance,
    pub prosody: ProsodyProfile,
    #[serde(default)]
    pub tone: VoiceTone,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::phonetics::{parse_ipa, PhoneticWord};

    fn sample(secs: f32, pitch: f32, ipa: &str) -> VoiceSample {
        VoiceSample {
            id: SampleId::new("x"),
            speaker_id: SpeakerId::new("s"),
            audio_locator: String::new(),
            duration_secs: secs,
            transcript: String::new(),
            canonical: PhoneticUtterance::new(vec![PhoneticWord::new("", parse_ipa(ipa), None)]),
            observed: None,
            features: AcousticFeatures {
                mean_pitch_hz: pitch,
                pitch_spread_hz: 20.0,
                rms_energy: 0.1,
                pause_ratio: 0.5,
            },
        }
    }

    #[test]
    fn aggregates_prosody_weighted_by_duration() {
        let p = ProsodyProfile::aggregate(&[
            sample(2.0, 100.0, "papa"),
            sample(6.0, 200.0, "papapapa"),
        ]);
        assert!((p.mean_pitch_hz - 175.0).abs() < 1e-3);
        // 6 syllables in 4s of speech
        assert!((p.syllables_per_second - 1.5).abs() < 1e-3);
    }

    #[test]
    fn enrollment_requires_material() {
        let policy = EnrollmentPolicy::default();
        assert!(policy.check(&[sample(4.0, 1.0, "a")]).is_err());
        assert!(policy
            .check(&[sample(4.0, 1.0, "a"), sample(7.0, 1.0, "a")])
            .is_ok());
    }
}
