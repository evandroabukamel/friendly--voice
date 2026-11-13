use crate::error::{AppError, AppResult};
use crate::ports::{
    AcousticAnalyzer, AudioDecoder, AudioStore, IdGenerator, SampleRepository, SpeakerRepository,
    Transcriber,
};
use friendly_voice_domain::{
    audio::SampleQualityPolicy,
    ids::{SampleId, SpeakerId},
    voice::VoiceSample,
};
use std::sync::Arc;

#[derive(Debug, Clone, Default)]
pub struct AddVoiceSampleInput {
    pub speaker_id: SpeakerId,
    pub audio: Vec<u8>,
    /// What was said (optional). If absent, the [`Transcriber`] is used when
    /// configured; the voice can be cloned without it.
    pub transcript: Option<String>,
}

/// US-02: upload audio files of the person.
pub struct AddVoiceSample {
    speakers: Arc<dyn SpeakerRepository>,
    samples: Arc<dyn SampleRepository>,
    store: Arc<dyn AudioStore>,
    decoder: Arc<dyn AudioDecoder>,
    analyzer: Arc<dyn AcousticAnalyzer>,
    ids: Arc<dyn IdGenerator>,
    transcriber: Option<Arc<dyn Transcriber>>,
    policy: SampleQualityPolicy,
}

impl AddVoiceSample {
    pub fn new(
        speakers: Arc<dyn SpeakerRepository>,
        samples: Arc<dyn SampleRepository>,
        store: Arc<dyn AudioStore>,
        decoder: Arc<dyn AudioDecoder>,
        analyzer: Arc<dyn AcousticAnalyzer>,
        ids: Arc<dyn IdGenerator>,
    ) -> Self {
        Self {
            speakers,
            samples,
            store,
            decoder,
            analyzer,
            ids,
            transcriber: None,
            policy: SampleQualityPolicy::default(),
        }
    }
    pub fn with_transcriber(mut self, t: Arc<dyn Transcriber>) -> Self {
        self.transcriber = Some(t);
        self
    }
    pub fn with_policy(mut self, policy: SampleQualityPolicy) -> Self {
        self.policy = policy;
        self
    }

    pub fn execute(&self, input: AddVoiceSampleInput) -> AppResult<VoiceSample> {
        let speaker = self
            .speakers
            .find(&input.speaker_id)?
            .ok_or_else(|| AppError::not_found("person", &input.speaker_id))?;

        let clip = self.decoder.decode(&input.audio)?;
        self.policy.validate(&clip)?;

        let transcript = match (
            input.transcript.filter(|t| !t.trim().is_empty()),
            &self.transcriber,
        ) {
            (Some(t), _) => Some(t),
            (None, Some(tr)) => {
                Some(tr.transcribe(&clip, &speaker.language)?).filter(|t| !t.trim().is_empty())
            }
            (None, None) => None,
        };

        let id = SampleId::new(self.ids.next_id("smp"));
        let locator = self.store.store(&input.speaker_id, &id, &input.audio)?;
        let sample = VoiceSample {
            id,
            speaker_id: input.speaker_id,
            audio_locator: locator,
            duration_secs: clip.duration_secs(),
            transcript,
            features: self.analyzer.analyze(&clip),
        };
        self.samples.save(&sample)?;
        Ok(sample)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fakes::Fakes;

    fn uc(f: &Fakes) -> AddVoiceSample {
        AddVoiceSample::new(
            f.speakers.clone(),
            f.samples.clone(),
            f.store.clone(),
            f.decoder.clone(),
            f.analyzer.clone(),
            f.ids.clone(),
        )
    }

    #[test]
    fn stores_analyzed_sample() {
        let f = Fakes::new();
        let spk = f.speaker();
        let s = uc(&f)
            .execute(AddVoiceSampleInput {
                speaker_id: spk.id.clone(),
                audio: Fakes::audio_bytes(5.0),
                transcript: Some("leite quente".into()),
            })
            .unwrap();
        assert_eq!(s.transcript.as_deref(), Some("leite quente"));
        assert!((s.duration_secs - 5.0).abs() < 1e-3);
        assert_eq!(f.samples.list_by_speaker(&spk.id).unwrap().len(), 1);
    }

    #[test]
    fn transcript_is_optional() {
        let f = Fakes::new();
        let spk = f.speaker();
        let s = uc(&f)
            .execute(AddVoiceSampleInput {
                speaker_id: spk.id,
                audio: Fakes::audio_bytes(5.0),
                ..Default::default()
            })
            .unwrap();
        assert_eq!(s.transcript, None);
    }

    #[test]
    fn transcribes_in_the_persons_language() {
        let f = Fakes::new();
        let spk = f.speaker_speaking("es".parse().unwrap());
        let s = uc(&f)
            .with_transcriber(f.transcriber.clone())
            .execute(AddVoiceSampleInput {
                speaker_id: spk.id,
                audio: Fakes::audio_bytes(5.0),
                ..Default::default()
            })
            .unwrap();
        assert_eq!(s.transcript.as_deref(), Some("bom dia"));
        assert_eq!(f.transcriber.language().unwrap().code(), "es");
    }

    #[test]
    fn rejects_bad_audio_and_unknown_speaker() {
        let f = Fakes::new();
        let spk = f.speaker();
        let short = AddVoiceSampleInput {
            speaker_id: spk.id,
            audio: Fakes::audio_bytes(0.5),
            transcript: Some("oi".into()),
        };
        assert!(matches!(
            uc(&f).execute(short.clone()),
            Err(AppError::Domain(_))
        ));
        let unknown = AddVoiceSampleInput {
            speaker_id: SpeakerId::new("nope"),
            ..short
        };
        assert!(matches!(
            uc(&f).execute(unknown),
            Err(AppError::NotFound { .. })
        ));
    }
}
