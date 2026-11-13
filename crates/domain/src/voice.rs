//! Voice samples, cloned voice profile and speech script.
use crate::audio::AcousticFeatures;
use crate::ids::{SampleId, SpeakerId};
use crate::language::Language;
use crate::tone::VoiceTone;
use crate::DomainError;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Audio sample uploaded by the user, already analyzed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceSample {
    pub id: SampleId,
    pub speaker_id: SpeakerId,
    /// Locator of the original audio in storage.
    pub audio_locator: String,
    pub duration_secs: f32,
    /// What was said in the recording, when known (it is not needed to clone
    /// the voice; it only helps to estimate the speaking rate).
    #[serde(default)]
    pub transcript: Option<String>,
    pub features: AcousticFeatures,
}

/// Rough syllable count of a text: groups of vowel letters per word. Good
/// enough for the speaking rate of languages written in the Latin alphabet;
/// other scripts count zero.
pub fn estimate_syllables(text: &str) -> usize {
    const VOWELS: &str = "aeiouyáàâãäéèêëíìîïóòôõöúùûüý";
    text.split(|c: char| !c.is_alphabetic())
        .map(|word| {
            let mut count = 0;
            let mut in_vowel = false;
            for c in word.chars().flat_map(char::to_lowercase) {
                let vowel = VOWELS.contains(c);
                if vowel && !in_vowel {
                    count += 1;
                }
                in_vowel = vowel;
            }
            count
        })
        .sum()
}

/// Prosodic traits (melody and rhythm) of the person.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct ProsodyProfile {
    pub mean_pitch_hz: f32,
    pub pitch_spread_hz: f32,
    /// 0 when no sample has a transcript.
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
        let (syllables, speech_secs) = samples
            .iter()
            .filter_map(|s| {
                let syllables = estimate_syllables(s.transcript.as_deref()?);
                (syllables > 0)
                    .then_some((syllables, s.duration_secs * (1.0 - s.features.pause_ratio)))
            })
            .fold((0usize, 0f32), |(n, secs), (s, d)| (n + s, secs + d));
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

/// Voice register detected from the recordings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VoiceGender {
    Female,
    Male,
}

impl VoiceGender {
    /// Boundary between typical adult male (≈ 85–155 Hz) and female
    /// (≈ 165–255 Hz) speaking pitch.
    pub const BOUNDARY_HZ: f32 = 160.0;

    /// Detected from the mean speaking pitch; `None` when there is no pitch.
    pub fn from_pitch(mean_pitch_hz: f32) -> Option<Self> {
        if !mean_pitch_hz.is_finite() || mean_pitch_hz <= 0.0 {
            None
        } else if mean_pitch_hz >= Self::BOUNDARY_HZ {
            Some(Self::Female)
        } else {
            Some(Self::Male)
        }
    }

    pub fn slug(self) -> &'static str {
        match self {
            Self::Female => "female",
            Self::Male => "male",
        }
    }
}

impl fmt::Display for VoiceGender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.slug())
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

/// Voice generated for a person: the clone lives in the synthesis engine;
/// here are its language, detected gender and prosody.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceProfile {
    pub speaker_id: SpeakerId,
    /// Voice identifier in the cloning engine.
    pub engine_voice_id: String,
    #[serde(default)]
    pub language: Language,
    #[serde(default)]
    pub gender: Option<VoiceGender>,
    pub prosody: ProsodyProfile,
    pub sample_count: usize,
    pub total_audio_secs: f32,
}

/// Script handed to the synthesizer: text + language + prosody (already
/// coloured by the chosen tone).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpeechScript {
    pub text: String,
    #[serde(default)]
    pub language: Language,
    pub prosody: ProsodyProfile,
    #[serde(default)]
    pub tone: VoiceTone,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample(secs: f32, pitch: f32, transcript: Option<&str>) -> VoiceSample {
        VoiceSample {
            id: SampleId::new("x"),
            speaker_id: SpeakerId::new("s"),
            audio_locator: String::new(),
            duration_secs: secs,
            transcript: transcript.map(str::to_string),
            features: AcousticFeatures {
                mean_pitch_hz: pitch,
                pitch_spread_hz: 20.0,
                rms_energy: 0.1,
                pause_ratio: 0.5,
            },
        }
    }

    #[test]
    fn estimates_syllables_from_vowel_groups() {
        assert_eq!(estimate_syllables("Bom dia, meu amor!"), 5);
        assert_eq!(estimate_syllables("coração"), 3);
        assert_eq!(estimate_syllables("你好"), 0);
    }

    #[test]
    fn aggregates_prosody_weighted_by_duration() {
        let p = ProsodyProfile::aggregate(&[
            sample(2.0, 100.0, Some("papa")),
            sample(6.0, 200.0, Some("papapapa")),
        ]);
        assert!((p.mean_pitch_hz - 175.0).abs() < 1e-3);
        // 6 syllables in 4s of speech
        assert!((p.syllables_per_second - 1.5).abs() < 1e-3);
    }

    #[test]
    fn rate_is_unknown_without_transcripts() {
        let p = ProsodyProfile::aggregate(&[sample(6.0, 200.0, None)]);
        assert_eq!(p.syllables_per_second, 0.0);
        assert!((p.mean_pitch_hz - 200.0).abs() < 1e-3);
    }

    #[test]
    fn detects_gender_from_pitch() {
        assert_eq!(VoiceGender::from_pitch(245.0), Some(VoiceGender::Female));
        assert_eq!(VoiceGender::from_pitch(118.0), Some(VoiceGender::Male));
        assert_eq!(VoiceGender::from_pitch(0.0), None);
        assert_eq!(VoiceGender::from_pitch(f32::NAN), None);
    }

    #[test]
    fn enrollment_requires_material() {
        let policy = EnrollmentPolicy::default();
        assert!(policy.check(&[sample(4.0, 1.0, None)]).is_err());
        assert!(policy
            .check(&[sample(4.0, 1.0, None), sample(7.0, 1.0, None)])
            .is_ok());
    }

    #[test]
    fn reads_samples_and_profiles_saved_by_older_versions() {
        let old_sample = r#"{"id":"smp-1","speaker_id":"spk-1","audio_locator":"a.wav",
            "duration_secs":5.0,"transcript":"oi","canonical":{"words":[]},"observed":null,
            "features":{"mean_pitch_hz":200.0,"pitch_spread_hz":20.0,"rms_energy":0.1,"pause_ratio":0.2}}"#;
        let s: VoiceSample = serde_json::from_str(old_sample).unwrap();
        assert_eq!(s.transcript.as_deref(), Some("oi"));
        let old_profile = r#"{"speaker_id":"spk-1","engine_voice_id":"v","accent":{},
            "prosody":{"mean_pitch_hz":200.0,"pitch_spread_hz":20.0,"syllables_per_second":5.0,"pause_ratio":0.2},
            "sample_count":1,"total_audio_secs":5.0}"#;
        let p: VoiceProfile = serde_json::from_str(old_profile).unwrap();
        assert_eq!(p.language, Language::default());
        assert_eq!(p.gender, None);
    }
}
