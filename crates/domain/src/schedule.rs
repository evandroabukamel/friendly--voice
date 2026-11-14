//! User-defined scheduling rules.
use crate::ids::{ScheduleId, SpeakerId};
use crate::message::MessageContent;
use crate::tone::VoiceTone;
use crate::DomainError;
use chrono::{Datelike, Duration, NaiveDateTime, NaiveTime, Weekday};
use serde::{Deserialize, Serialize};

/// Time window; may wrap around midnight (e.g. 22:00–07:00).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeWindow {
    pub start: NaiveTime,
    pub end: NaiveTime,
}

impl TimeWindow {
    pub fn new(start: NaiveTime, end: NaiveTime) -> Result<Self, DomainError> {
        if start == end {
            return Err(DomainError::InvalidSchedule(
                "the time window cannot start and end at the same time".into(),
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

/// When the message should play.
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
    /// Every N minutes, optionally only inside a window.
    Interval {
        every_minutes: u32,
        within: Option<TimeWindow>,
    },
}

/// Maximum horizon when searching for the next occurrence.
const HORIZON_DAYS: i64 = 8;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schedule {
    pub id: ScheduleId,
    pub speaker_id: SpeakerId,
    pub recurrence: Recurrence,
    pub content: MessageContent,
    /// Chosen tone; `None` = initial tone suggested by the theme.
    #[serde(default)]
    pub tone: Option<VoiceTone>,
    /// "Do not disturb": never fires inside this window.
    pub quiet_hours: Option<TimeWindow>,
    pub enabled: bool,
    pub created_at: NaiveDateTime,
    pub last_fired_at: Option<NaiveDateTime>,
    /// Set when the schedule is re-enabled or edited: occurrences before this
    /// instant are never fired (no catch-up of what was missed meanwhile).
    #[serde(default)]
    pub active_since: Option<NaiveDateTime>,
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
                    "provide at least one weekday".into(),
                ))
            }
            Recurrence::Interval { every_minutes, .. } if *every_minutes < 1 => {
                return Err(DomainError::InvalidSchedule(
                    "the minimum interval is 1 minute".into(),
                ))
            }
            Recurrence::Once { at } if *at <= created_at => {
                return Err(DomainError::InvalidSchedule(
                    "the date must be in the future".into(),
                ))
            }
            _ => {}
        }
        if let MessageContent::FixedText(t) = &content {
            if t.trim().is_empty() {
                return Err(DomainError::InvalidSchedule("empty fixed text".into()));
            }
        }
        let schedule = Self {
            id,
            speaker_id,
            recurrence,
            content,
            tone: None,
            quiet_hours,
            enabled: true,
            created_at,
            last_fired_at: None,
            active_since: None,
        };
        if schedule.next_occurrence_after(created_at).is_none() {
            return Err(DomainError::InvalidSchedule(
                "the rule never fires (conflict with the quiet hours?)".into(),
            ));
        }
        Ok(schedule)
    }

    pub fn with_tone(mut self, tone: Option<VoiceTone>) -> Self {
        self.tone = tone;
        self
    }

    fn allowed(&self, t: NaiveDateTime) -> bool {
        !self.quiet_hours.is_some_and(|q| q.contains(t.time()))
    }

    /// Next firing strictly after `after`.
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
                        // Outside the window: jump to the next window start.
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

    /// Reference instant used to compute the next firing.
    fn anchor(&self) -> NaiveDateTime {
        [self.last_fired_at, self.active_since]
            .into_iter()
            .flatten()
            .fold(self.created_at, NaiveDateTime::max)
    }

    pub fn next_due(&self) -> Option<NaiveDateTime> {
        self.next_occurrence_after(self.anchor())
    }

    /// Is it time? (If the app was off, it fires only once.)
    pub fn is_due(&self, now: NaiveDateTime) -> bool {
        self.enabled && self.next_due().is_some_and(|t| t <= now)
    }

    /// Turns the schedule on or off. Idempotent: setting the current state
    /// changes nothing. Re-enabling only fires future occurrences (US-08: what
    /// was missed while off is not caught up).
    pub fn set_enabled(&mut self, enabled: bool, now: NaiveDateTime) -> Result<(), DomainError> {
        if self.enabled == enabled {
            return Ok(());
        }
        if enabled && self.next_occurrence_after(now.max(self.anchor())).is_none() {
            return Err(DomainError::InvalidSchedule(
                "the rule never fires again (a one-time message that already passed?); edit it with a new date".into(),
            ));
        }
        self.enabled = enabled;
        if enabled {
            self.active_since = Some(now);
        }
        Ok(())
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
        // 2026-11-02 is a Monday
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

    #[test]
    fn re_enabling_fires_only_future_occurrences() {
        let mut s = make(Recurrence::Daily { at: t(8, 0) }, None).unwrap();
        s.set_enabled(false, dt(2, 11, 0)).unwrap();
        assert!(!s.is_due(dt(5, 9, 0)), "disabled never fires");
        // Re-enabled on the 5th at 09:00: the missed 08:00 runs are not caught up.
        s.set_enabled(true, dt(5, 9, 0)).unwrap();
        assert!(!s.is_due(dt(5, 9, 0)));
        assert_eq!(s.next_due(), Some(dt(6, 8, 0)));
        assert!(s.is_due(dt(6, 8, 0)));
    }

    #[test]
    fn enabling_is_idempotent() {
        let mut s = make(Recurrence::Daily { at: t(8, 0) }, None).unwrap();
        // Already enabled: a second "enable" must not skip a pending catch-up.
        s.set_enabled(true, dt(3, 9, 0)).unwrap();
        assert!(s.is_due(dt(3, 9, 0)));
        let before = s.clone();
        s.set_enabled(false, dt(3, 9, 0)).unwrap();
        s.set_enabled(false, dt(4, 9, 0)).unwrap();
        s.set_enabled(true, dt(4, 9, 0)).unwrap();
        let again = s.clone();
        s.set_enabled(true, dt(4, 10, 0)).unwrap();
        assert_eq!(s, again);
        assert_ne!(s, before);
    }

    #[test]
    fn a_passed_one_time_message_cannot_be_re_enabled() {
        let mut s = make(Recurrence::Once { at: dt(2, 11, 0) }, None).unwrap();
        s.mark_fired(dt(2, 11, 0));
        let err = s.set_enabled(true, dt(3, 9, 0)).unwrap_err();
        assert!(err.to_string().contains("new date"), "{err}");
        assert!(!s.enabled, "left unchanged on error");
    }

    #[test]
    fn reads_schedules_saved_before_re_enabling_existed() {
        let old = r#"{"id":"sch-1","speaker_id":"s","recurrence":{"Daily":{"at":"08:00:00"}},
            "content":{"FixedText":"oi"},"quiet_hours":null,"enabled":true,
            "created_at":"2026-11-02T10:00:00","last_fired_at":null}"#;
        let s: Schedule = serde_json::from_str(old).unwrap();
        assert_eq!(s.active_since, None);
        assert_eq!(s.next_due(), Some(dt(3, 8, 0)));
    }
}
