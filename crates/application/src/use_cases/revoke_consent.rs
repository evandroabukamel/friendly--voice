use crate::error::{AppError, AppResult};
use crate::ports::{Clock, SpeakerRepository};
use crate::use_cases::VoiceDataEraser;
use friendly_voice_domain::ids::SpeakerId;
use std::sync::Arc;

/// US-15: revoking consent deletes the person's recordings, voice (local and
/// on the engine) and cached speech, switches off their schedules and keeps
/// the person only as a revoked record, so every later use is refused.
///
/// Idempotent and crash-safe: the person is marked revoked first, then every
/// cleanup step is itself idempotent, so running it again (after a crash or a
/// voice server that was unreachable) finishes what was left.
pub struct RevokeConsent {
    speakers: Arc<dyn SpeakerRepository>,
    eraser: VoiceDataEraser,
    clock: Arc<dyn Clock>,
}

impl RevokeConsent {
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

    pub fn execute(&self, speaker_id: &SpeakerId, revoked_by: Option<String>) -> AppResult<()> {
        let mut speaker = self
            .speakers
            .find(speaker_id)?
            .ok_or_else(|| AppError::not_found("person", speaker_id))?;
        if !speaker.is_revoked() {
            speaker.revoke(revoked_by, self.clock.now());
            self.speakers.save(&speaker)?;
        }
        self.eraser.erase(&speaker.id, &speaker.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fakes::Fakes;
    use crate::ports::{SampleRepository, ScheduleRepository, VoiceProfileRepository};
    use crate::use_cases::{
        AddVoiceSampleInput, CreateScheduleInput, SpeakRequest, UpdateSchedule,
    };
    use chrono::NaiveTime;
    use friendly_voice_domain::{
        message::MessageContent,
        schedule::{Recurrence, Schedule},
        speaker::Speaker,
        DomainError,
    };

    fn input(speaker_id: SpeakerId) -> CreateScheduleInput {
        CreateScheduleInput {
            speaker_id,
            recurrence: Recurrence::Daily {
                at: NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
            },
            content: MessageContent::FixedText("bom dia".into()),
            tone: None,
            quiet_hours: None,
        }
    }

    /// A person with recordings, a voice and a schedule.
    fn person_in_use(f: &Fakes) -> (Speaker, Schedule) {
        let spk = f.speaker();
        f.add_sample(&spk, 12.0, Some("oi"));
        f.voice_profile(&spk);
        let schedule = f.create_schedule().execute(input(spk.id.clone())).unwrap();
        (spk, schedule)
    }

    fn revoked(err: AppError) -> bool {
        matches!(err, AppError::Domain(DomainError::ConsentRevoked(_)))
    }

    #[test]
    fn deletes_the_data_and_keeps_a_revoked_record() {
        let f = Fakes::new();
        let (spk, schedule) = person_in_use(&f);
        let other = f.speaker();
        f.add_sample(&other, 12.0, None);

        f.revoke_consent()
            .execute(&spk.id, Some("Maria".into()))
            .unwrap();

        let kept = f.speakers.find(&spk.id).unwrap().unwrap();
        let revocation = kept.consent_revoked.unwrap();
        assert_eq!(revocation.by.as_deref(), Some("Maria"));
        assert_eq!(revocation.at, f.clock.now());
        assert!(f.samples.list_by_speaker(&spk.id).unwrap().is_empty());
        assert_eq!(f.store.len(), 1, "only the other person's audio is left");
        assert_eq!(f.profiles.find(&spk.id).unwrap(), None);
        assert_eq!(f.engine.forgotten(), vec!["v".to_string()]);
        assert_eq!(f.speech_cache.cleared(), vec![spk.id.clone()]);
        let s = f.schedules.find(&schedule.id).unwrap().unwrap();
        assert!(!s.enabled, "the schedule is kept, switched off");
        assert_eq!(f.samples.list_by_speaker(&other.id).unwrap().len(), 1);
    }

    #[test]
    fn every_later_use_is_refused() {
        let f = Fakes::new();
        let (spk, schedule) = person_in_use(&f);
        f.revoke_consent().execute(&spk.id, None).unwrap();

        let add = f.add_voice_sample().execute(AddVoiceSampleInput {
            speaker_id: spk.id.clone(),
            audio: Fakes::audio_bytes(12.0),
            transcript: None,
        });
        assert!(revoked(add.unwrap_err()));
        f.add_sample(&spk, 12.0, None); // even if material shows up somehow
        assert!(revoked(
            f.build_voice_profile().execute(&spk.id).unwrap_err()
        ));
        f.voice_profile(&spk);
        let speak = f.speak_message().execute(SpeakRequest {
            speaker_id: spk.id.clone(),
            content: MessageContent::FixedText("oi".into()),
            listener: None,
            listener_override: None,
            tone: None,
        });
        assert!(revoked(speak.unwrap_err()));
        assert!(revoked(
            f.create_schedule()
                .execute(input(spk.id.clone()))
                .unwrap_err()
        ));
        let update = UpdateSchedule::new(f.speakers.clone(), f.schedules.clone(), f.clock.clone())
            .execute(&schedule.id, input(spk.id.clone()));
        assert!(revoked(update.unwrap_err()));
        let enable = f.set_schedule_enabled().execute(&schedule.id, true);
        assert!(
            enable
                .unwrap_err()
                .to_string()
                .contains("edit the schedule"),
            "actionable error"
        );
    }

    #[test]
    fn running_again_finishes_an_interrupted_revocation() {
        let f = Fakes::new();
        let (spk, _) = person_in_use(&f);
        *f.engine.forget_fails.lock().unwrap() = true;
        let err = f.revoke_consent().execute(&spk.id, None).unwrap_err();
        assert!(err.to_string().contains("run it again"), "{err}");
        // Already refused, local data already gone; the voice is still known.
        assert!(f.speakers.find(&spk.id).unwrap().unwrap().is_revoked());
        assert!(f.samples.list_by_speaker(&spk.id).unwrap().is_empty());
        assert!(f.profiles.find(&spk.id).unwrap().is_some());

        *f.engine.forget_fails.lock().unwrap() = false;
        f.revoke_consent()
            .execute(&spk.id, Some("late".into()))
            .unwrap();
        assert_eq!(f.profiles.find(&spk.id).unwrap(), None);
        assert_eq!(f.engine.forgotten(), vec!["v".to_string()]);
        let first = f.speakers.find(&spk.id).unwrap().unwrap();
        assert_eq!(
            first.consent_revoked.unwrap().by,
            None,
            "first revocation kept"
        );

        f.revoke_consent().execute(&spk.id, None).unwrap();
        assert_eq!(f.engine.forgotten().len(), 1, "nothing left to forget");
    }

    #[test]
    fn unknown_person() {
        let f = Fakes::new();
        assert!(matches!(
            f.revoke_consent().execute(&SpeakerId::new("ghost"), None),
            Err(AppError::NotFound { .. })
        ));
    }
}
