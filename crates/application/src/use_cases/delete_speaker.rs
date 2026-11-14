use crate::error::AppResult;
use crate::ports::{Clock, SpeakerRepository};
use crate::use_cases::VoiceDataEraser;
use friendly_voice_domain::ids::SpeakerId;
use std::sync::Arc;

/// Deletes a person: the same cleanup as revoking consent, plus the person's
/// record. Their schedules are switched off and kept (they can be edited to
/// another voice). Idempotent: an unknown id is fine.
pub struct DeleteSpeaker {
    speakers: Arc<dyn SpeakerRepository>,
    eraser: VoiceDataEraser,
    clock: Arc<dyn Clock>,
}

impl DeleteSpeaker {
    pub fn new(
        speakers: Arc<dyn SpeakerRepository>,
        eraser: VoiceDataEraser,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            speakers,
            eraser,
            clock,
        }
    }

    pub fn execute(&self, speaker_id: &SpeakerId) -> AppResult<()> {
        let speaker = self.speakers.find(speaker_id)?;
        let name = match speaker {
            Some(mut s) => {
                // Unusable from now on, even if the cleanup is interrupted.
                if !s.is_revoked() {
                    s.revoke(None, self.clock.now());
                    self.speakers.save(&s)?;
                }
                s.name
            }
            None => speaker_id.to_string(),
        };
        self.eraser.erase(speaker_id, &name)?;
        self.speakers.delete(speaker_id)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::fakes::Fakes;
    use crate::ports::{
        SampleRepository, ScheduleRepository, SpeakerRepository, VoiceProfileRepository,
    };
    use crate::use_cases::CreateScheduleInput;
    use chrono::NaiveTime;
    use friendly_voice_domain::{ids::SpeakerId, message::MessageContent, schedule::Recurrence};

    #[test]
    fn deletes_the_person_and_everything_made_from_the_voice() {
        let f = Fakes::new();
        let spk = f.speaker();
        f.add_sample(&spk, 12.0, None);
        f.voice_profile(&spk);
        let schedule = f
            .create_schedule()
            .execute(CreateScheduleInput {
                speaker_id: spk.id.clone(),
                recurrence: Recurrence::Daily {
                    at: NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
                },
                content: MessageContent::FixedText("oi".into()),
                tone: None,
                quiet_hours: None,
            })
            .unwrap();

        f.delete_speaker().execute(&spk.id).unwrap();
        assert_eq!(f.speakers.find(&spk.id).unwrap(), None);
        assert!(f.samples.list_by_speaker(&spk.id).unwrap().is_empty());
        assert_eq!(f.store.len(), 0);
        assert_eq!(f.profiles.find(&spk.id).unwrap(), None);
        assert_eq!(f.engine.forgotten(), vec!["v".to_string()]);
        assert_eq!(f.speech_cache.cleared(), vec![spk.id.clone()]);
        let kept = f.schedules.find(&schedule.id).unwrap().unwrap();
        assert!(!kept.enabled);

        f.delete_speaker().execute(&spk.id).unwrap();
        f.delete_speaker()
            .execute(&SpeakerId::new("ghost"))
            .unwrap();
    }

    #[test]
    fn an_interrupted_delete_refuses_the_voice_and_finishes_on_retry() {
        let f = Fakes::new();
        let spk = f.speaker();
        f.voice_profile(&spk);
        *f.engine.forget_fails.lock().unwrap() = true;
        assert!(f.delete_speaker().execute(&spk.id).is_err());
        let left = f.speakers.find(&spk.id).unwrap().unwrap();
        assert!(
            left.is_revoked(),
            "already unusable while the cleanup is pending"
        );

        *f.engine.forget_fails.lock().unwrap() = false;
        f.delete_speaker().execute(&spk.id).unwrap();
        assert_eq!(f.speakers.find(&spk.id).unwrap(), None);
        assert_eq!(f.profiles.find(&spk.id).unwrap(), None);
    }
}
