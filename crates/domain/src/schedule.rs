//! Regras de agendamento definidas pelo usuário.
use crate::ids::{ScheduleId, SpeakerId};
use crate::message::MessageContent;
use crate::DomainError;
use chrono::{Datelike, Duration, NaiveDateTime, NaiveTime, Weekday};
use serde::{Deserialize, Serialize};

/// Janela de horário; aceita virar a meia-noite (ex.: 22:00–07:00).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeWindow {
    pub start: NaiveTime,
    pub end: NaiveTime,
}

impl TimeWindow {
    pub fn new(start: NaiveTime, end: NaiveTime) -> Result<Self, DomainError> {
        if start == end {
            return Err(DomainError::InvalidSchedule(
                "a janela de horário não pode ter início igual ao fim".into(),
            ));
        }
        Ok(Self { start, end })
    }
    pub fn contains(&self, t: NaiveTime) -> bool {
        if self.start < self.end {
            t >= self.start && t < self.end
        } else {
            t >= self.start || t < self.end
        }
    }
}

/// Quando a mensagem deve tocar.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Recurrence {
    Once {
        at: NaiveDateTime,
    },
    Daily {
        at: NaiveTime,
    },
    Weekly {
        days: Vec<Weekday>,
        at: NaiveTime,
    },
    /// A cada N minutos, opcionalmente só dentro de uma janela.
    Interval {
        every_minutes: u32,
        within: Option<TimeWindow>,
    },
}

/// Horizonte máximo de busca da próxima ocorrência.
const HORIZON_DAYS: i64 = 8;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schedule {
    pub id: ScheduleId,
    pub speaker_id: SpeakerId,
    pub recurrence: Recurrence,
    pub content: MessageContent,
    /// "Não perturbe": nunca dispara dentro desta janela.
    pub quiet_hours: Option<TimeWindow>,
    pub enabled: bool,
    pub created_at: NaiveDateTime,
    pub last_fired_at: Option<NaiveDateTime>,
}

impl Schedule {
    pub fn new(
        id: ScheduleId,
        speaker_id: SpeakerId,
        recurrence: Recurrence,
        content: MessageContent,
        quiet_hours: Option<TimeWindow>,
        created_at: NaiveDateTime,
    ) -> Result<Self, DomainError> {
        match &recurrence {
            Recurrence::Weekly { days, .. } if days.is_empty() => {
                return Err(DomainError::InvalidSchedule(
                    "informe ao menos um dia da semana".into(),
                ))
            }
            Recurrence::Interval { every_minutes, .. } if *every_minutes < 1 => {
                return Err(DomainError::InvalidSchedule(
                    "intervalo mínimo de 1 minuto".into(),
                ))
            }
            Recurrence::Once { at } if *at <= created_at => {
                return Err(DomainError::InvalidSchedule(
                    "a data precisa estar no futuro".into(),
                ))
            }
            _ => {}
        }
        if let MessageContent::FixedText(t) = &content {
            if t.trim().is_empty() {
                return Err(DomainError::InvalidSchedule("texto fixo vazio".into()));
            }
        }
        let schedule = Self {
            id,
            speaker_id,
            recurrence,
            content,
            quiet_hours,
            enabled: true,
            created_at,
            last_fired_at: None,
        };
        if schedule.next_occurrence_after(created_at).is_none() {
            return Err(DomainError::InvalidSchedule(
                "a regra nunca dispara (conflito com o horário de silêncio?)".into(),
            ));
        }
        Ok(schedule)
    }

    fn allowed(&self, t: NaiveDateTime) -> bool {
        !self.quiet_hours.is_some_and(|q| q.contains(t.time()))
    }

    /// Próximo disparo estritamente após `after`.
    pub fn next_occurrence_after(&self, after: NaiveDateTime) -> Option<NaiveDateTime> {
        match &self.recurrence {
            Recurrence::Once { at } => (*at > after && self.allowed(*at)).then_some(*at),
            Recurrence::Daily { at } => (0..=HORIZON_DAYS)
                .map(|d| (after.date() + Duration::days(d)).and_time(*at))
                .find(|c| *c > after && self.allowed(*c)),
            Recurrence::Weekly { days, at } => (0..=HORIZON_DAYS)
                .map(|d| (after.date() + Duration::days(d)).and_time(*at))
                .find(|c| *c > after && days.contains(&c.weekday()) && self.allowed(*c)),
            Recurrence::Interval {
                every_minutes,
                within,
            } => {
                let step = Duration::minutes(i64::from(*every_minutes));
                let limit = after + Duration::days(HORIZON_DAYS);
                let mut c = after + step;
                while c <= limit {
                    if let Some(w) = within.filter(|w| !w.contains(c.time())) {
                        // Fora da janela: pula para o próximo início da janela.
                        let start = c.date().and_time(w.start);
                        c = if start > c {
                            start
                        } else {
                            start + Duration::days(1)
                        };
                        continue;
                    }
                    if self.allowed(c) {
                        return Some(c);
                    }
                    c += step;
                }
                None
            }
        }
    }

    /// Momento de referência para calcular o próximo disparo.
    fn anchor(&self) -> NaiveDateTime {
        self.last_fired_at.unwrap_or(self.created_at)
    }

    pub fn next_due(&self) -> Option<NaiveDateTime> {
        self.next_occurrence_after(self.anchor())
    }

    /// Está na hora? (Se o app ficou desligado, dispara uma única vez.)
    pub fn is_due(&self, now: NaiveDateTime) -> bool {
        self.enabled && self.next_due().is_some_and(|t| t <= now)
    }

    pub fn mark_fired(&mut self, now: NaiveDateTime) {
        self.last_fired_at = Some(now);
        if matches!(self.recurrence, Recurrence::Once { .. }) {
            self.enabled = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::MessageCategory;
    use chrono::NaiveDate;

    fn dt(d: u32, h: u32, m: u32) -> NaiveDateTime {
        // 2026-11-02 é uma segunda-feira
        NaiveDate::from_ymd_opt(2026, 11, d)
            .unwrap()
            .and_hms_opt(h, m, 0)
            .unwrap()
    }
    fn t(h: u32, m: u32) -> NaiveTime {
        NaiveTime::from_hms_opt(h, m, 0).unwrap()
    }
    fn make(r: Recurrence, quiet: Option<TimeWindow>) -> Result<Schedule, DomainError> {
        Schedule::new(
            ScheduleId::new("1"),
            SpeakerId::new("s"),
            r,
            MessageContent::Category(MessageCategory::Motivation),
            quiet,
            dt(2, 10, 0),
        )
    }

    #[test]
    fn daily_fires_next_day_when_time_passed() {
        let s = make(Recurrence::Daily { at: t(8, 0) }, None).unwrap();
        assert_eq!(s.next_due(), Some(dt(3, 8, 0)));
        assert!(!s.is_due(dt(3, 7, 59)));
        assert!(s.is_due(dt(3, 8, 0)));
    }

    #[test]
    fn weekly_respects_days() {
        let s = make(
            Recurrence::Weekly {
                days: vec![Weekday::Fri],
                at: t(7, 30),
            },
            None,
        )
        .unwrap();
        assert_eq!(s.next_due(), Some(dt(6, 7, 30)));
    }

    #[test]
    fn interval_respects_window_and_quiet_hours() {
        let window = TimeWindow::new(t(9, 0), t(18, 0)).unwrap();
        let quiet = TimeWindow::new(t(12, 0), t(14, 0)).unwrap();
        let mut s = make(
            Recurrence::Interval {
                every_minutes: 60,
                within: Some(window),
            },
            Some(quiet),
        )
        .unwrap();
        assert_eq!(s.next_due(), Some(dt(2, 11, 0)));
        s.mark_fired(dt(2, 11, 0));
        assert_eq!(s.next_due(), Some(dt(2, 14, 0)));
        s.mark_fired(dt(2, 17, 0));
        assert_eq!(s.next_due(), Some(dt(3, 9, 0)));
    }

    #[test]
    fn interval_starts_at_window_opening() {
        let window = TimeWindow::new(t(9, 0), t(18, 0)).unwrap();
        let s = make(
            Recurrence::Interval {
                every_minutes: 120,
                within: Some(window),
            },
            None,
        )
        .unwrap();
        assert_eq!(s.next_occurrence_after(dt(3, 3, 30)), Some(dt(3, 9, 0)));
    }

    #[test]
    fn quiet_hours_wrap_midnight() {
        let q = TimeWindow::new(t(22, 0), t(7, 0)).unwrap();
        assert!(q.contains(t(23, 0)) && q.contains(t(6, 59)) && !q.contains(t(7, 0)));
    }

    #[test]
    fn once_disables_after_firing() {
        let mut s = make(Recurrence::Once { at: dt(2, 11, 0) }, None).unwrap();
        assert!(s.is_due(dt(2, 11, 0)));
        s.mark_fired(dt(2, 11, 0));
        assert!(!s.is_due(dt(5, 0, 0)));
    }

    #[test]
    fn rejects_invalid_rules() {
        assert!(make(
            Recurrence::Weekly {
                days: vec![],
                at: t(8, 0)
            },
            None
        )
        .is_err());
        assert!(make(Recurrence::Once { at: dt(1, 0, 0) }, None).is_err());
        let all_day = TimeWindow::new(t(0, 0), t(23, 59)).unwrap();
        assert!(make(Recurrence::Daily { at: t(8, 0) }, Some(all_day)).is_err());
    }
}
