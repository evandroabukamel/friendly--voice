use crate::error::{AppError, AppResult};
use crate::ports::{Clock, IdGenerator, ScheduleRepository, SpeakerRepository};
use crate::use_cases::speak_message::{SpeakMessage, SpeakRequest, SpokenMessage};
use friendly_voice_domain::{
    ids::{ScheduleId, SpeakerId},
    message::MessageContent,
    schedule::{Recurrence, Schedule, TimeWindow},
    tone::VoiceTone,
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct CreateScheduleInput {
    pub speaker_id: SpeakerId,
    pub recurrence: Recurrence,
    pub content: MessageContent,
    /// Tom de voz das mensagens deste agendamento (`None` = sugerido pelo tema).
    pub tone: Option<VoiceTone>,
    pub quiet_hours: Option<TimeWindow>,
}

/// US-07: agendar mensagens com regras definidas pelo usuário.
pub struct CreateSchedule {
    speakers: Arc<dyn SpeakerRepository>,
    schedules: Arc<dyn ScheduleRepository>,
    ids: Arc<dyn IdGenerator>,
    clock: Arc<dyn Clock>,
}

impl CreateSchedule {
    pub fn new(
        speakers: Arc<dyn SpeakerRepository>,
        schedules: Arc<dyn ScheduleRepository>,
        ids: Arc<dyn IdGenerator>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            speakers,
            schedules,
            ids,
            clock,
        }
    }

    pub fn execute(&self, input: CreateScheduleInput) -> AppResult<Schedule> {
        self.speakers
            .find(&input.speaker_id)?
            .ok_or_else(|| AppError::not_found("pessoa", &input.speaker_id))?;
        let schedule = Schedule::new(
            ScheduleId::new(self.ids.next_id("sch")),
            input.speaker_id,
            input.recurrence,
            input.content,
            input.quiet_hours,
            self.clock.now(),
        )?
        .with_tone(input.tone);
        self.schedules.save(&schedule)?;
        Ok(schedule)
    }
}

pub struct ListSchedules {
    schedules: Arc<dyn ScheduleRepository>,
}

impl ListSchedules {
    pub fn new(schedules: Arc<dyn ScheduleRepository>) -> Self {
        Self { schedules }
    }
    pub fn execute(&self) -> AppResult<Vec<Schedule>> {
        let mut all = self.schedules.list()?;
        all.sort_by_key(|s| s.next_due());
        Ok(all)
    }
}

pub struct RemoveSchedule {
    schedules: Arc<dyn ScheduleRepository>,
}

impl RemoveSchedule {
    pub fn new(schedules: Arc<dyn ScheduleRepository>) -> Self {
        Self { schedules }
    }
    pub fn execute(&self, id: &ScheduleId) -> AppResult<()> {
        if self.schedules.delete(id)? {
            Ok(())
        } else {
            Err(AppError::not_found("agendamento", id))
        }
    }
}

#[derive(Debug)]
pub struct ScheduleRunOutcome {
    pub schedule_id: ScheduleId,
    pub result: AppResult<SpokenMessage>,
}

/// US-08: disparar os agendamentos vencidos.
pub struct RunDueSchedules {
    schedules: Arc<dyn ScheduleRepository>,
    speak: Arc<SpeakMessage>,
    clock: Arc<dyn Clock>,
    listener: Option<String>,
    accent_intensity: f32,
}

impl RunDueSchedules {
    pub fn new(
        schedules: Arc<dyn ScheduleRepository>,
        speak: Arc<SpeakMessage>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            schedules,
            speak,
            clock,
            listener: None,
            accent_intensity: 1.0,
        }
    }
    pub fn with_listener(mut self, listener: Option<String>) -> Self {
        self.listener = listener;
        self
    }
    pub fn with_accent_intensity(mut self, intensity: f32) -> Self {
        self.accent_intensity = intensity;
        self
    }

    /// Um agendamento com falha não impede os demais; o erro é reportado e
    /// o agendamento é marcado como disparado para não repetir em loop.
    pub fn execute(&self) -> AppResult<Vec<ScheduleRunOutcome>> {
        let now = self.clock.now();
        let mut outcomes = Vec::new();
        for mut schedule in self.schedules.list()?.into_iter().filter(|s| s.is_due(now)) {
            let result = self.speak.execute(SpeakRequest {
                speaker_id: schedule.speaker_id.clone(),
                content: schedule.content.clone(),
                listener: self.listener.clone(),
                accent_intensity: self.accent_intensity,
                tone: schedule.tone,
            });
            schedule.mark_fired(now);
            self.schedules.save(&schedule)?;
            outcomes.push(ScheduleRunOutcome {
                schedule_id: schedule.id.clone(),
                result,
            });
        }
        Ok(outcomes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fakes::Fakes;
    use chrono::{Duration, NaiveTime};

    #[test]
    fn creates_and_runs_due_schedules_once() {
        let f = Fakes::new();
        let spk = f.speaker();
        f.carioca_profile(&spk);
        let create = CreateSchedule::new(
            f.speakers.clone(),
            f.schedules.clone(),
            f.ids.clone(),
            f.clock.clone(),
        );
        let at = (f.clock.now() + Duration::minutes(5)).time();
        create
            .execute(CreateScheduleInput {
                speaker_id: spk.id.clone(),
                recurrence: Recurrence::Daily { at },
                content: MessageContent::FixedText("bom dia".into()),
                tone: Some(VoiceTone::of(
                    friendly_voice_domain::tone::ToneStyle::Whisper,
                )),
                quiet_hours: None,
            })
            .unwrap();

        let run = RunDueSchedules::new(
            f.schedules.clone(),
            Arc::new(f.speak_message()),
            f.clock.clone(),
        );
        assert!(run.execute().unwrap().is_empty());

        f.clock.advance(Duration::minutes(6));
        let out = run.execute().unwrap();
        assert_eq!(out.len(), 1);
        let spoken = out[0].result.as_ref().unwrap();
        assert_eq!(
            spoken.script.tone.style,
            friendly_voice_domain::tone::ToneStyle::Whisper
        );
        assert!(run.execute().unwrap().is_empty(), "não repete no mesmo dia");

        f.clock.advance(Duration::days(1));
        assert_eq!(run.execute().unwrap().len(), 1);
        assert_eq!(f.delivery.delivered().len(), 2);
    }

    #[test]
    fn list_and_remove() {
        let f = Fakes::new();
        let spk = f.speaker();
        let create = CreateSchedule::new(
            f.speakers.clone(),
            f.schedules.clone(),
            f.ids.clone(),
            f.clock.clone(),
        );
        let s = create
            .execute(CreateScheduleInput {
                speaker_id: spk.id,
                recurrence: Recurrence::Daily {
                    at: NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
                },
                content: MessageContent::FixedText("oi".into()),
                tone: None,
                quiet_hours: None,
            })
            .unwrap();
        assert_eq!(
            ListSchedules::new(f.schedules.clone())
                .execute()
                .unwrap()
                .len(),
            1
        );
        RemoveSchedule::new(f.schedules.clone())
            .execute(&s.id)
            .unwrap();
        assert!(RemoveSchedule::new(f.schedules.clone())
            .execute(&s.id)
            .is_err());
    }
}
