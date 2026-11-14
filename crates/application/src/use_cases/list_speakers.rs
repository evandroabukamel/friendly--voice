use crate::error::AppResult;
use crate::ports::{SampleRepository, SpeakerRepository, VoiceProfileRepository};
use friendly_voice_domain::{speaker::Speaker, voice::VoiceProfile};
use std::sync::Arc;

/// A person with what the UI needs to know about their recordings and voice.
#[derive(Debug, Clone, PartialEq)]
pub struct SpeakerOverview {
    pub speaker: Speaker,
    pub voice: Option<VoiceProfile>,
    pub sample_count: usize,
    pub total_sample_secs: f32,
}

/// Lists the people whose consent stands, with sample totals (so the UI knows whether there is
/// enough material to generate the voice) and their voice, if generated.
pub struct ListSpeakers {
    speakers: Arc<dyn SpeakerRepository>,
    samples: Arc<dyn SampleRepository>,
    profiles: Arc<dyn VoiceProfileRepository>,
}

impl ListSpeakers {
    pub fn new(
        speakers: Arc<dyn SpeakerRepository>,
        samples: Arc<dyn SampleRepository>,
        profiles: Arc<dyn VoiceProfileRepository>,
    ) -> Self {
        Self {
            speakers,
            samples,
            profiles,
        }
    }

    pub fn execute(&self) -> AppResult<Vec<SpeakerOverview>> {
        self.speakers
            .list()?
            .into_iter()
            // A revoked person stays only as a record; it can't be chosen.
            .filter(|s| !s.is_revoked())
            .map(|s| self.one(s))
            .collect()
    }

    /// Overview of one person already loaded.
    pub fn one(&self, speaker: Speaker) -> AppResult<SpeakerOverview> {
        let samples = self.samples.list_by_speaker(&speaker.id)?;
        Ok(SpeakerOverview {
            voice: self.profiles.find(&speaker.id)?,
            sample_count: samples.len(),
            total_sample_secs: samples.iter().map(|s| s.duration_secs).sum(),
            speaker,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fakes::Fakes;

    #[test]
    fn counts_samples_and_reports_the_voice() {
        let f = Fakes::new();
        let mom = f.speaker();
        let dad = f.speaker();
        f.add_sample(&mom, 4.0, None);
        f.add_sample(&mom, 7.5, Some("oi"));
        f.voice_profile(&mom);
        let uc = ListSpeakers::new(f.speakers.clone(), f.samples.clone(), f.profiles.clone());
        let all = uc.execute().unwrap();
        assert_eq!(all.len(), 2);
        let m = all.iter().find(|o| o.speaker.id == mom.id).unwrap();
        assert_eq!(m.sample_count, 2);
        assert!((m.total_sample_secs - 11.5).abs() < 1e-4);
        assert!(m.voice.is_some());
        let d = all.iter().find(|o| o.speaker.id == dad.id).unwrap();
        assert_eq!(
            (d.sample_count, d.total_sample_secs, d.voice.is_none()),
            (0, 0.0, true)
        );
        assert_eq!(uc.one(mom).unwrap(), *m);
    }

    #[test]
    fn revoked_people_are_not_listed() {
        let f = Fakes::new();
        let mom = f.speaker();
        let dad = f.speaker();
        f.revoke_consent().execute(&dad.id, None).unwrap();
        let uc = ListSpeakers::new(f.speakers.clone(), f.samples.clone(), f.profiles.clone());
        let ids: Vec<_> = uc
            .execute()
            .unwrap()
            .into_iter()
            .map(|o| o.speaker.id)
            .collect();
        assert_eq!(ids, vec![mom.id]);
    }
}
