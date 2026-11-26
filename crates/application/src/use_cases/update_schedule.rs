use crate::error::{AppError, AppResult};
use crate::ports::{Clock, ScheduleRepository, SpeakerRepository};
use crate::use_cases::CreateScheduleInput;
use friendly_voice_domain::{ids::ScheduleId, schedule::Schedule};
use std::sync::Arc;

/// Edit an alarm: replaces the rule of an existing schedule, keeping its id
/// and its on/off state. Takes the same input as [`crate::use_cases::CreateSchedule`].
pub struct UpdateSchedule {
    speakers: Arc<dyn SpeakerRepository>,
    schedules: Arc<dyn ScheduleRepository>,
    clock: Arc<dyn Clock>,
}

impl UpdateSchedule {
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

    pub fn execute(&self, id: &ScheduleId, input: CreateScheduleInput) -> AppResult<Schedule> {
        let current = self
            .schedules
            .find(id)?
            .ok_or_else(|| AppError::not_found("schedule", id))?;
        self.speakers
            .find(&input.speaker_id)?
            .ok_or_else(|| AppError::not_found("person", &input.speaker_id))?
            .ensure_active()?;
        let revised = current.clone().revise(
            input.speaker_id,
            input.recurrence,
            input.content,
            input.tone,
            input.quiet_hours,
            self.clock.now(),
        )?;
        if revised != current {
            self.schedules.save(&revised)?;
        }
        Ok(revised)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fakes::Fakes;
    use crate::use_cases::CreateSchedule;
    use chrono::NaiveTime;
    use friendly_voice_domain::{
        ids::SpeakerId,
        message::{MessageCategory, MessageContent},
        schedule::{Recurrence, TimeWindow},
        tone::{ToneStyle, VoiceTone},
    };

    fn t(h: u32, m: u32) -> NaiveTime {
        NaiveTime::from_hms_opt(h, m, 0).unwrap()
    }

    fn input(speaker_id: SpeakerId, at: NaiveTime) -> CreateScheduleInput {
        CreateScheduleInput {
            speaker_id,
            recurrence: Recurrence::Daily { at },
            content: MessageContent::FixedText("bom dia".into()),
            tone: None,
            quiet_hours: None,
        }
    }

    fn uc(f: &Fakes) -> UpdateSchedule {
        UpdateSchedule::new(f.speakers.clone(), f.schedules.clone(), f.clock.clone())
    }

    #[test]
    fn replaces_the_rule_keeping_id_and_switch() {
        let f = Fakes::new();
        let mom = f.speaker();
        let dad = f.speaker();
        let created = CreateSchedule::new(
            f.speakers.clone(),
            f.schedules.clone(),
            f.ids.clone(),
            f.clock.clone(),
        )
        .execute(input(mom.id.clone(), t(8, 0)))
        .unwrap();

        let updated = uc(&f)
            .execute(
                &created.id,
                CreateScheduleInput {
                    speaker_id: dad.id.clone(),
                    recurrence: Recurrence::Weekly {
                        days: vec![chrono::Weekday::Fri],
                        at: t(7, 30),
                    },
                    content: MessageContent::Category(MessageCategory::Calm),
                    tone: Some(VoiceTone::of(ToneStyle::Whisper)),
                    quiet_hours: Some(TimeWindow::new(t(22, 0), t(6, 0)).unwrap()),
                },
            )
            .unwrap();
        assert_eq!(updated.id, created.id);
        assert!(updated.enabled);
        assert_eq!(updated.speaker_id, dad.id);
        assert_eq!(
            updated.content,
            MessageContent::Category(MessageCategory::Calm)
        );
        assert_eq!(f.schedules.list().unwrap(), vec![updated.clone()]);

        // Same request again: nothing changes.
        let again = uc(&f)
            .execute(
                &created.id,
                CreateScheduleInput {
                    speaker_id: dad.id,
                    recurrence: updated.recurrence.clone(),
                    content: updated.content.clone(),
                    tone: updated.tone,
                    quiet_hours: updated.quiet_hours,
                },
            )
            .unwrap();
        assert_eq!(again, updated);
    }

    #[test]
    fn rejects_unknown_schedule_and_person() {
        let f = Fakes::new();
        let spk = f.speaker();
        let err = uc(&f)
            .execute(&ScheduleId::new("nope"), input(spk.id, t(8, 0)))
            .unwrap_err();
        assert!(
            matches!(
                err,
                AppError::NotFound {
                    entity: "schedule",
                    ..
                }
            ),
            "{err}"
        );

        let created = CreateSchedule::new(
            f.speakers.clone(),
            f.schedules.clone(),
            f.ids.clone(),
            f.clock.clone(),
        )
        .execute(input(f.speaker().id, t(8, 0)))
        .unwrap();
        let err = uc(&f)
            .execute(&created.id, input(SpeakerId::new("ghost"), t(8, 0)))
            .unwrap_err();
        assert!(
            matches!(
                err,
                AppError::NotFound {
                    entity: "person",
                    ..
                }
            ),
            "{err}"
        );
    }
}
