use crate::error::{AppError, AppResult};
use crate::ports::{
    AudioStore, ReferenceAudio, SampleRepository, SpeakerRepository, VoiceCloningEngine,
    VoiceProfileRepository,
};
use friendly_voice_domain::{
    accent::{AccentAnalysis, AccentAnalyzer, AccentProfile, AnalysisEvidence},
    ids::SpeakerId,
    voice::{EnrollmentPolicy, ProsodyProfile, VoiceProfile},
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct BuildVoiceProfileOutput {
    pub profile: VoiceProfile,
    /// `None` when no sample had observed phonemes.
    pub accent_analysis: Option<AccentAnalysis>,
}

/// US-03/US-04: generate the person's voice (timbre + accent + prosody).
pub struct BuildVoiceProfile {
    speakers: Arc<dyn SpeakerRepository>,
    samples: Arc<dyn SampleRepository>,
    profiles: Arc<dyn VoiceProfileRepository>,
    store: Arc<dyn AudioStore>,
    engine: Arc<dyn VoiceCloningEngine>,
    analyzer: AccentAnalyzer,
    policy: EnrollmentPolicy,
}

impl BuildVoiceProfile {
    pub fn new(
        speakers: Arc<dyn SpeakerRepository>,
        samples: Arc<dyn SampleRepository>,
        profiles: Arc<dyn VoiceProfileRepository>,
        store: Arc<dyn AudioStore>,
        engine: Arc<dyn VoiceCloningEngine>,
    ) -> Self {
        Self {
            speakers,
            samples,
            profiles,
            store,
            engine,
            analyzer: AccentAnalyzer::default(),
            policy: EnrollmentPolicy::default(),
        }
    }
    pub fn with_policy(mut self, policy: EnrollmentPolicy) -> Self {
        self.policy = policy;
        self
    }
    pub fn with_analyzer(mut self, analyzer: AccentAnalyzer) -> Self {
        self.analyzer = analyzer;
        self
    }

    pub fn execute(&self, speaker_id: &SpeakerId) -> AppResult<BuildVoiceProfileOutput> {
        let speaker = self
            .speakers
            .find(speaker_id)?
            .ok_or_else(|| AppError::not_found("person", speaker_id))?;
        let samples = self.samples.list_by_speaker(speaker_id)?;
        self.policy.check(&samples)?;

        // 1. Accent: learned from the samples that have observed phonemes.
        let evidence: Vec<AnalysisEvidence> = samples
            .iter()
            .filter_map(|s| {
                s.observed.as_ref().map(|obs| AnalysisEvidence {
                    canonical: s.canonical.clone(),
                    observed: obs.clone(),
                })
            })
            .collect();
        let analysis = (!evidence.is_empty()).then(|| self.analyzer.analyze(&evidence));
        let accent = analysis
            .as_ref()
            .map(|a| a.profile.clone())
            .unwrap_or_else(AccentProfile::neutral);

        // 2. Prosody (melody and rhythm).
        let prosody = ProsodyProfile::aggregate(&samples);

        // 3. Timbre: delegated to the cloning engine.
        let references = samples
            .iter()
            .map(|s| {
                Ok(ReferenceAudio {
                    bytes: self.store.load(&s.audio_locator)?,
                    transcript: s.transcript.clone(),
                })
            })
            .collect::<AppResult<Vec<_>>>()?;
        let engine_voice_id = self.engine.enroll(&speaker, &references)?;

        let profile = VoiceProfile {
            speaker_id: speaker_id.clone(),
            engine_voice_id,
            accent,
            prosody,
            sample_count: samples.len(),
            total_audio_secs: samples.iter().map(|s| s.duration_secs).sum(),
        };
        self.profiles.save(&profile)?;
        Ok(BuildVoiceProfileOutput {
            profile,
            accent_analysis: analysis,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fakes::Fakes;

    #[test]
    fn builds_profile_with_learned_accent() {
        let f = Fakes::new();
        let spk = f.speaker();
        f.add_sample(&spk, 6.0, "leite quente", Some("lejtʃi kẽtʃi"));
        f.add_sample(&spk, 6.0, "noite de festa", Some("nojtʃi dʒi fɛʃta"));
        let uc = BuildVoiceProfile::new(
            f.speakers.clone(),
            f.samples.clone(),
            f.profiles.clone(),
            f.store.clone(),
            f.engine.clone(),
        );
        let out = uc.execute(&spk.id).unwrap();
        let codes = out.profile.accent.rule_codes();
        assert!(codes.contains(&"final-e-raising"), "{codes:?}");
        assert!(codes.contains(&"t-palatalization"));
        assert_eq!(
            out.profile.engine_voice_id,
            format!("fake-voice:{}", spk.id)
        );
        assert_eq!(f.engine.enrolled_references(), 2);
        assert!(f.profiles.find(&spk.id).unwrap().is_some());
    }

    #[test]
    fn neutral_accent_without_observed_phones_and_policy_enforced() {
        let f = Fakes::new();
        let spk = f.speaker();
        let uc = BuildVoiceProfile::new(
            f.speakers.clone(),
            f.samples.clone(),
            f.profiles.clone(),
            f.store.clone(),
            f.engine.clone(),
        );
        assert!(matches!(uc.execute(&spk.id), Err(AppError::Domain(_))));
        f.add_sample(&spk, 12.0, "leite quente", None);
        let out = uc.execute(&spk.id).unwrap();
        assert!(out.accent_analysis.is_none());
        assert!(out.profile.accent.segment_rules.is_empty());
    }
}
