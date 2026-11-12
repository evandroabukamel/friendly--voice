use crate::error::{AppError, AppResult};
use crate::ports::{
    AcousticAnalyzer, AudioDecoder, AudioStore, IdGenerator, PhoneRecognizer, Phonemizer,
    SampleRepository, SpeakerRepository, Transcriber,
};
use friendly_voice_domain::{
    audio::SampleQualityPolicy,
    ids::{SampleId, SpeakerId},
    phonetics::parse_ipa,
    voice::VoiceSample,
};
use std::sync::Arc;

#[derive(Debug, Clone, Default)]
pub struct AddVoiceSampleInput {
    pub speaker_id: SpeakerId,
    pub audio: Vec<u8>,
    /// What was said. If absent, the [`Transcriber`] is used (when configured).
    pub transcript: Option<String>,
    /// Phonetic transcription of what was heard (IPA). If absent, the
    /// [`PhoneRecognizer`] is used (when configured).
    pub observed_ipa: Option<String>,
}

/// US-02: upload audio files of the person.
pub struct AddVoiceSample {
    speakers: Arc<dyn SpeakerRepository>,
    samples: Arc<dyn SampleRepository>,
    store: Arc<dyn AudioStore>,
    decoder: Arc<dyn AudioDecoder>,
    analyzer: Arc<dyn AcousticAnalyzer>,
    phonemizer: Arc<dyn Phonemizer>,
    ids: Arc<dyn IdGenerator>,
    transcriber: Option<Arc<dyn Transcriber>>,
    recognizer: Option<Arc<dyn PhoneRecognizer>>,
    policy: SampleQualityPolicy,
}

impl AddVoiceSample {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        speakers: Arc<dyn SpeakerRepository>,
        samples: Arc<dyn SampleRepository>,
        store: Arc<dyn AudioStore>,
        decoder: Arc<dyn AudioDecoder>,
        analyzer: Arc<dyn AcousticAnalyzer>,
        phonemizer: Arc<dyn Phonemizer>,
        ids: Arc<dyn IdGenerator>,
    ) -> Self {
        Self {
            speakers,
            samples,
            store,
            decoder,
            analyzer,
            phonemizer,
            ids,
            transcriber: None,
            recognizer: None,
            policy: SampleQualityPolicy::default(),
        }
    }
    pub fn with_transcriber(mut self, t: Arc<dyn Transcriber>) -> Self {
        self.transcriber = Some(t);
        self
    }
    pub fn with_phone_recognizer(mut self, r: Arc<dyn PhoneRecognizer>) -> Self {
        self.recognizer = Some(r);
        self
    }
    pub fn with_policy(mut self, policy: SampleQualityPolicy) -> Self {
        self.policy = policy;
        self
    }

    pub fn execute(&self, input: AddVoiceSampleInput) -> AppResult<VoiceSample> {
        self.speakers
            .find(&input.speaker_id)?
            .ok_or_else(|| AppError::not_found("person", &input.speaker_id))?;

        let clip = self.decoder.decode(&input.audio)?;
        self.policy.validate(&clip)?;

        let transcript = match (
            input.transcript.filter(|t| !t.trim().is_empty()),
            &self.transcriber,
        ) {
            (Some(t), _) => t,
            (None, Some(tr)) => tr.transcribe(&clip)?,
            (None, None) => {
                return Err(AppError::Precondition(
                    "provide the transcript of the audio (no speech recognizer is configured)"
                        .into(),
                ))
            }
        };
        let canonical = self.phonemizer.phonemize(&transcript);
        if canonical.words.is_empty() {
            return Err(AppError::Precondition("the transcript has no words".into()));
        }

        let observed = match (
            input.observed_ipa.filter(|s| !s.trim().is_empty()),
            &self.recognizer,
        ) {
            (Some(ipa), _) => Some(parse_ipa(&ipa)),
            // An empty result means the recognizer is not available.
            (None, Some(r)) => Some(r.recognize(&clip, &transcript)?).filter(|p| !p.is_empty()),
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
            canonical,
            observed,
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
            f.phonemizer.clone(),
            f.ids.clone(),
        )
    }

    #[test]
    fn stores_analyzed_sample_with_observed_phones() {
        let f = Fakes::new();
        let spk = f.speaker();
        let s = uc(&f)
            .execute(AddVoiceSampleInput {
                speaker_id: spk.id.clone(),
                audio: Fakes::audio_bytes(5.0),
                transcript: Some("leite quente".into()),
                observed_ipa: Some("lejtʃi kẽtʃi".into()),
            })
            .unwrap();
        assert_eq!(s.canonical.ipa(), "lejte kẽte");
        assert_eq!(s.observed.as_ref().unwrap().len(), 9);
        assert!((s.duration_secs - 5.0).abs() < 1e-3);
        assert_eq!(f.samples.list_by_speaker(&spk.id).unwrap().len(), 1);
    }

    #[test]
    fn requires_transcript_when_no_transcriber() {
        let f = Fakes::new();
        let spk = f.speaker();
        let err = uc(&f)
            .execute(AddVoiceSampleInput {
                speaker_id: spk.id,
                audio: Fakes::audio_bytes(5.0),
                ..Default::default()
            })
            .unwrap_err();
        assert!(matches!(err, AppError::Precondition(_)));
    }

    #[test]
    fn rejects_bad_audio_and_unknown_speaker() {
        let f = Fakes::new();
        let spk = f.speaker();
        let short = AddVoiceSampleInput {
            speaker_id: spk.id,
            audio: Fakes::audio_bytes(0.5),
            transcript: Some("oi".into()),
            observed_ipa: None,
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
