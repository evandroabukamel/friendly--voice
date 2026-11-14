use crate::error::{AppError, AppResult};
use crate::ports::{Clock, ScheduleRepository, SpeakerRepository};
use friendly_voice_domain::{ids::ScheduleId, schedule::Schedule};
use std::sync::Arc;

/// Alarm on/off switch: turns a schedule on or off without losing it.
pub struct SetScheduleEnabled {
    speakers: Arc<dyn SpeakerRepository>,
    schedules: Arc<dyn ScheduleRepository>,
    clock: Arc<dyn Clock>,
}

impl SetScheduleEnabled {
    pub fn new(
        speakers: Arc<dyn SpeakerRepository>,
        schedules: Arc<dyn ScheduleRepository>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            speakers,
            schedules,
            clock,
        }
    }

    pub fn execute(&self, id: &ScheduleId, enabled: bool) -> AppResult<Schedule> {
        let mut schedule = self
            .schedules
            .find(id)?
            .ok_or_else(|| AppError::not_found("schedule", id))?;
        if enabled && !schedule.enabled {
            let available = self
                .speakers
                .find(&schedule.speaker_id)?
                .is_some_and(|s| !s.is_revoked());
            if !available {
                return Err(AppError::Precondition(
                    "the voice of this schedule is no longer available (consent revoked or person deleted); edit the schedule to choose another voice".into(),
                ));
            }
        }
        if schedule.enabled != enabled {
            schedule.set_enabled(enabled, self.clock.now())?;
            self.schedules.save(&schedule)?;
        }
        Ok(schedule)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fakes::Fakes;
    use crate::use_cases::{CreateSchedule, CreateScheduleInput, RunDueSchedules};
    use chrono::{Duration, NaiveTime};
    use friendly_voice_domain::{message::MessageContent, schedule::Recurrence};

    fn daily_at_eight(f: &Fakes) -> Schedule {
        let spk = f.speaker();
        f.voice_profile(&spk);
        CreateSchedule::new(
            f.speakers.clone(),
            f.schedules.clone(),
            f.ids.clone(),
            f.clock.clone(),
        )
        .execute(CreateScheduleInput {
            speaker_id: spk.id,
            recurrence: Recurrence::Daily {
                at: NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
            },
            content: MessageContent::FixedText("bom dia".into()),
            tone: None,
            quiet_hours: None,
        })
        .unwrap()
    }

    #[test]
    fn switching_off_and_on_skips_what_was_missed() {
        let f = Fakes::new();
        let s = daily_at_eight(&f);
        let uc = f.set_schedule_enabled();
        let run = RunDueSchedules::new(
            f.schedules.clone(),
            std::sync::Arc::new(f.speak_message()),
            f.clock.clone(),
        );

        assert!(!uc.execute(&s.id, false).unwrap().enabled);
        f.clock.advance(Duration::days(2));
        assert!(run.execute().unwrap().is_empty(), "off: nothing fires");

        let on = uc.execute(&s.id, true).unwrap();
        assert!(on.enabled);
        assert!(
            run.execute().unwrap().is_empty(),
            "no catch-up after re-enabling"
        );
        f.clock.advance(Duration::days(1));
        assert_eq!(
            run.execute().unwrap().len(),
            1,
            "next future occurrence fires"
        );
    }

    #[test]
    fn is_idempotent_and_reports_unknown_ids() {
        let f = Fakes::new();
        let s = daily_at_eight(&f);
        let uc = f.set_schedule_enabled();
        let first = uc.execute(&s.id, true).unwrap();
        assert_eq!(first, s, "enabling an enabled schedule changes nothing");
        let off = uc.execute(&s.id, false).unwrap();
        assert_eq!(uc.execute(&s.id, false).unwrap(), off);
        assert_eq!(f.schedules.list().unwrap(), vec![off]);

        let err = uc.execute(&ScheduleId::new("nope"), true).unwrap_err();
        assert!(matches!(err, AppError::NotFound { .. }), "{err}");
    }
}
