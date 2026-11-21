//! Voice material (audio samples) and voice generation.
use super::failure::AppFailure;
use super::session::{non_empty, Session};
use super::speakers::Gender;
use friendly_voice_application::use_cases::AddVoiceSampleInput;
use friendly_voice_domain::ids::SpeakerId;

#[derive(Debug, Clone, PartialEq)]
pub struct SampleSummary {
    pub id: String,
    pub duration_secs: f32,
    pub mean_pitch_hz: f32,
    pub transcript: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VoiceSummary {
    pub language: String,
    pub gender: Option<Gender>,
    pub sample_count: u32,
    pub total_audio_secs: f32,
}

impl Session {
    /// Adds a WAV or FLAC recording of the person (format detected from the
    /// bytes). The transcript is optional.
    pub fn add_sample(
        &self,
        speaker_id: String,
        audio: Vec<u8>,
        transcript: Option<String>,
    ) -> Result<SampleSummary, AppFailure> {
        let sample = self
            .container
            .add_voice_sample()
            .execute(AddVoiceSampleInput {
                speaker_id: SpeakerId::new(speaker_id),
                audio,
                transcript: non_empty(transcript),
            })
            .map_err(AppFailure::new)?;
        Ok(SampleSummary {
            id: sample.id.to_string(),
            duration_secs: sample.duration_secs,
            mean_pitch_hz: sample.features.mean_pitch_hz,
            transcript: sample.transcript,
        })
    }

    /// Generates (or regenerates) the person's voice from all their samples.
    pub fn build_voice(&self, speaker_id: String) -> Result<VoiceSummary, AppFailure> {
        let profile = self
            .container
            .build_voice_profile()
            .execute(&SpeakerId::new(speaker_id))
            .map_err(AppFailure::new)?;
        Ok(VoiceSummary {
            language: profile.language.to_string(),
            gender: profile.gender.map(Gender::from),
            sample_count: u32::try_from(profile.sample_count).unwrap_or(u32::MAX),
            total_audio_secs: profile.total_audio_secs,
        })
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::api::session::tests::local_session;
    use crate::api::speakers::Gender;

    /// Voiced WAV (harmonics at `f0`) with syllable-like pauses.
    pub(crate) fn wav(secs: f32, f0: f32) -> Vec<u8> {
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 16_000,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut cursor = std::io::Cursor::new(Vec::new());
        let mut w = hound::WavWriter::new(&mut cursor, spec).unwrap();
        for i in 0..(secs * 16_000.0) as usize {
            let t = i as f32 / 16_000.0;
            let v: f32 = if (t % 0.5) < 0.35 {
                (1..6)
                    .map(|k| (std::f32::consts::TAU * f0 * k as f32 * t).sin() / k as f32)
                    .sum::<f32>()
                    * 0.15
            } else {
                0.0
            };
            w.write_sample((v * i16::MAX as f32) as i16).unwrap();
        }
        w.finalize().unwrap();
        cursor.into_inner()
    }

    #[test]
    fn a_sample_is_analyzed_and_a_short_one_refused() {
        let s = local_session("sample");
        let p = s
            .register_speaker("Vó".into(), None, Some("Vó".into()), None)
            .unwrap();
        let sample = s
            .add_sample(p.id.clone(), wav(6.0, 210.0), Some("Bom dia".into()))
            .unwrap();
        assert!((sample.duration_secs - 6.0).abs() < 0.01);
        assert!(
            (sample.mean_pitch_hz - 210.0).abs() < 10.0,
            "{}",
            sample.mean_pitch_hz
        );
        let err = s.add_sample(p.id, wav(0.5, 210.0), None).unwrap_err();
        assert!(err.message.contains("too short"), "{err}");
    }

    #[test]
    fn the_voice_is_generated_from_enough_material() {
        let s = local_session("voice");
        let p = s
            .register_speaker("Vó".into(), None, Some("Vó".into()), None)
            .unwrap();
        s.add_sample(p.id.clone(), wav(6.0, 210.0), None).unwrap();
        let err = s.build_voice(p.id.clone()).unwrap_err();
        assert!(err.message.contains("not enough"), "{err}");
        s.add_sample(p.id.clone(), wav(6.0, 205.0), None).unwrap();
        let voice = s.build_voice(p.id.clone()).unwrap();
        assert_eq!(voice.language, "pt-BR");
        assert_eq!(voice.gender, Some(Gender::Female));
        assert_eq!(voice.sample_count, 2);
        assert!((voice.total_audio_secs - 12.0).abs() < 0.01);
        let listed = s.list_speakers().unwrap();
        assert!(listed[0].voice_ready);
        assert_eq!(listed[0].gender, Some(Gender::Female));
    }
}
