use crate::error::{AppError, AppResult};
use crate::ports::{SampleRepository, SpeakerRepository};
use friendly_voice_domain::{ids::SpeakerId, voice::VoiceSample};
use std::sync::Arc;

/// Lists the recordings of one person.
pub struct ListSamples {
    speakers: Arc<dyn SpeakerRepository>,
    samples: Arc<dyn SampleRepository>,
}

impl ListSamples {
    pub fn new(speakers: Arc<dyn SpeakerRepository>, samples: Arc<dyn SampleRepository>) -> Self {
        Self { speakers, samples }
    }

    pub fn execute(&self, speaker_id: &SpeakerId) -> AppResult<Vec<VoiceSample>> {
        self.speakers
            .find(speaker_id)?
            .ok_or_else(|| AppError::not_found("person", speaker_id))?;
        Ok(self.samples.list_by_speaker(speaker_id)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fakes::Fakes;

    #[test]
    fn lists_only_that_persons_samples() {
        let f = Fakes::new();
        let mom = f.speaker();
        let dad = f.speaker();
        f.add_sample(&mom, 4.0, Some("oi"));
        f.add_sample(&dad, 5.0, None);
        let uc = ListSamples::new(f.speakers.clone(), f.samples.clone());
        let samples = uc.execute(&mom.id).unwrap();
        assert_eq!(samples.len(), 1);
        assert_eq!(samples[0].transcript.as_deref(), Some("oi"));
        assert!(matches!(
            uc.execute(&SpeakerId::new("ghost")),
            Err(AppError::NotFound { .. })
        ));
    }
}
