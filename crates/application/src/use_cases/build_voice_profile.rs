use crate::error::{AppError, AppResult};
use crate::ports::{
    AudioStore, Clock, ReferenceAudio, SampleRepository, SpeakerRepository, SpeechCache,
    VoiceCloningEngine, VoiceProfileRepository,
};
use friendly_voice_domain::{
    ids::SpeakerId,
    voice::{EnrollmentPolicy, ProsodyProfile, VoiceGender, VoiceProfile},
};
use std::sync::Arc;

/// US-11: generate the person's voice — clone it from every recording, in
/// the person's language, and detect whether it is a female or male voice.
pub struct BuildVoiceProfile {
    speakers: Arc<dyn SpeakerRepository>,
    samples: Arc<dyn SampleRepository>,
    profiles: Arc<dyn VoiceProfileRepository>,
    store: Arc<dyn AudioStore>,
    engine: Arc<dyn VoiceCloningEngine>,
    cache: Arc<dyn SpeechCache>,
    clock: Arc<dyn Clock>,
    policy: EnrollmentPolicy,
}

impl BuildVoiceProfile {
    pub fn new(
        speakers: Arc<dyn SpeakerRepository>,
        samples: Arc<dyn SampleRepository>,
        profiles: Arc<dyn VoiceProfileRepository>,
        store: Arc<dyn AudioStore>,
        engine: Arc<dyn VoiceCloningEngine>,
        cache: Arc<dyn SpeechCache>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            speakers,
            samples,
            profiles,
            store,
            engine,
            cache,
            clock,
            policy: EnrollmentPolicy::default(),
        }
    }
    pub fn with_policy(mut self, policy: EnrollmentPolicy) -> Self {
        self.policy = policy;
        self
    }

    pub fn execute(&self, speaker_id: &SpeakerId) -> AppResult<VoiceProfile> {
        let speaker = self
            .speakers
            .find(speaker_id)?
            .ok_or_else(|| AppError::not_found("person", speaker_id))?;
        let samples = self.samples.list_by_speaker(speaker_id)?;
        self.policy.check(&samples)?;

        let prosody = ProsodyProfile::aggregate(&samples);
        let gender = VoiceGender::from_pitch(prosody.mean_pitch_hz);

        let references = samples
            .iter()
            .map(|s| {
                Ok(ReferenceAudio {
                    bytes: self.store.load(&s.audio_locator)?,
                    transcript: s.transcript.clone(),
                })
            })
            .collect::<AppResult<Vec<_>>>()?;
        let engine_voice_id = self.engine.enroll(&speaker, gender, &references)?;

        let profile = VoiceProfile {
            speaker_id: speaker_id.clone(),
            engine_voice_id,
            language: speaker.language.clone(),
            gender,
            prosody,
            sample_count: samples.len(),
            total_audio_secs: samples.iter().map(|s| s.duration_secs).sum(),
            built_at: Some(self.clock.now()),
        };
        self.profiles.save(&profile)?;
        // The old voice's talks are unreachable (the key holds `built_at`);
        // delete them so they do not pile up.
        self.cache.clear(speaker_id)?;
        Ok(profile)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fakes::Fakes;
    use crate::ports::Clock;

    fn uc(f: &Fakes) -> BuildVoiceProfile {
        BuildVoiceProfile::new(
            f.speakers.clone(),
            f.samples.clone(),
            f.profiles.clone(),
            f.store.clone(),
            f.engine.clone(),
            f.speech_cache.clone(),
            f.clock.clone(),
        )
    }

    #[test]
    fn clones_from_every_sample_and_detects_the_gender() {
        let f = Fakes::new();
        let spk = f.speaker_speaking("en".parse().unwrap());
        f.add_sample(&spk, 6.0, Some("good morning"));
        f.add_sample(&spk, 6.0, None);
        let profile = uc(&f).execute(&spk.id).unwrap();
        assert_eq!(profile.engine_voice_id, format!("fake-voice:{}", spk.id));
        assert_eq!(f.engine.enrolled_references(), 2);
        // the fake analyzer measures 200 Hz
        assert_eq!(profile.gender, Some(VoiceGender::Female));
        assert_eq!(f.engine.enrolled_gender(), Some(VoiceGender::Female));
        assert_eq!(profile.language.code(), "en");
        assert_eq!(profile.built_at, Some(f.clock.now()), "versions the voice");
        assert!(f.profiles.find(&spk.id).unwrap().is_some());
        assert_eq!(
            f.speech_cache.cleared(),
            vec![spk.id.clone()],
            "the old voice's cached speech is deleted"
        );
    }

    #[test]
    fn policy_enforced() {
        let f = Fakes::new();
        let spk = f.speaker();
        assert!(matches!(uc(&f).execute(&spk.id), Err(AppError::Domain(_))));
        assert!(
            f.speech_cache.cleared().is_empty(),
            "a failed build keeps the cache"
        );
        f.add_sample(&spk, 12.0, None);
        assert!(uc(&f).execute(&spk.id).is_ok());
    }
}
