//! User-defined schedules and the alarms that wake the app to speak them.
use super::failure::AppFailure;
use super::messages::{content, tone};
use super::session::{non_empty, Session};
use chrono::{Local, NaiveDateTime, TimeZone};
use friendly_voice_application::use_cases::CreateScheduleInput;
use friendly_voice_bootstrap::parse;
use friendly_voice_domain::{
    ids::{ScheduleId, SpeakerId},
    schedule::{Recurrence, Schedule},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScheduleKind {
    Daily,
    Weekly,
    Interval,
    Once,
}

/// A schedule as typed in the UI. Flat on purpose: optional fields per
/// kind (Dart gets a plain class, no code generation needed).
#[derive(Debug, Clone, PartialEq)]
pub struct ScheduleRequest {
    pub speaker_id: String,
    pub kind: ScheduleKind,
    /// `HH:MM` (daily/weekly) or `YYYY-MM-DD HH:MM` (once).
    pub at: Option<String>,
    /// Weekly: `mon,wed,fri`.
    pub days: Option<String>,
    /// Interval: every N minutes...
    pub every_minutes: Option<u32>,
    /// ...optionally only inside `HH:MM-HH:MM`.
    pub within: Option<String>,
    /// Do not disturb: `HH:MM-HH:MM`.
    pub quiet: Option<String>,
    pub theme: Option<String>,
    pub text: Option<String>,
    pub tone: Option<String>,
    pub pitch_shift: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScheduleSummary {
    pub id: String,
    pub speaker_id: String,
    pub tone_label: String,
    /// Next firing as `dd/mm HH:MM`, if any.
    pub next_due: Option<String>,
}

/// One schedule fired by `run_due_schedules`.
#[derive(Debug, Clone, PartialEq)]
pub struct DueMessage {
    pub schedule_id: String,
    pub text: Option<String>,
    pub tone: Option<String>,
    pub audio_path: Option<String>,
    pub error: Option<String>,
}

/// An exact time at which the app should be woken by a notification.
#[derive(Debug, Clone, PartialEq)]
pub struct Alarm {
    pub schedule_id: String,
    /// Milliseconds since the Unix epoch.
    pub at_epoch_ms: i64,
}

fn recurrence(r: &ScheduleRequest) -> Result<Recurrence, AppFailure> {
    let at = || non_empty(r.at.clone()).ok_or_else(|| AppFailure::new("provide the time (HH:MM)"));
    let parsed = match r.kind {
        ScheduleKind::Daily => parse::time(&at()?).map(|at| Recurrence::Daily { at }),
        ScheduleKind::Weekly => {
            let days = non_empty(r.days.clone())
                .ok_or_else(|| AppFailure::new("provide the weekdays (mon,wed,fri)"))?;
            parse::weekly(&format!("{days}@{}", at()?))
                .map(|(days, at)| Recurrence::Weekly { days, at })
        }
        ScheduleKind::Interval => {
            let every_minutes = r
                .every_minutes
                .ok_or_else(|| AppFailure::new("provide the interval in minutes"))?;
            non_empty(r.within.clone())
                .as_deref()
                .map(parse::window)
                .transpose()
                .map(|within| Recurrence::Interval {
                    every_minutes,
                    within,
                })
        }
        ScheduleKind::Once => parse::datetime(&at()?).map(|at| Recurrence::Once { at }),
    };
    parsed.map_err(AppFailure::new)
}

fn summary(s: &Schedule) -> ScheduleSummary {
    ScheduleSummary {
        id: s.id.to_string(),
        speaker_id: s.speaker_id.to_string(),
        tone_label: s
            .tone
            .map(|t| t.to_string())
            .unwrap_or_else(|| "suggested by the theme".into()),
        next_due: s
            .next_due()
            .filter(|_| s.enabled)
            .map(|d| d.format("%d/%m %H:%M").to_string()),
    }
}

fn epoch_ms(local: NaiveDateTime) -> Option<i64> {
    Local
        .from_local_datetime(&local)
        .earliest()
        .map(|d| d.timestamp_millis())
}

impl Session {
    #[flutter_rust_bridge::frb(sync)]
    pub fn create_schedule(&self, request: ScheduleRequest) -> Result<ScheduleSummary, AppFailure> {
        let schedule = self
            .container
            .create_schedule()
            .execute(CreateScheduleInput {
                speaker_id: SpeakerId::new(request.speaker_id.clone()),
                recurrence: recurrence(&request)?,
                content: content(request.theme.clone(), request.text.clone())?,
                tone: tone(request.tone.clone(), request.pitch_shift)?,
                quiet_hours: non_empty(request.quiet.clone())
                    .as_deref()
                    .map(parse::window)
                    .transpose()
                    .map_err(AppFailure::new)?,
            })
            .map_err(AppFailure::new)?;
        Ok(summary(&schedule))
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn list_schedules(&self) -> Result<Vec<ScheduleSummary>, AppFailure> {
        let all = self
            .container
            .list_schedules()
            .execute()
            .map_err(AppFailure::new)?;
        Ok(all.iter().map(summary).collect())
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn remove_schedule(&self, id: String) -> Result<(), AppFailure> {
        self.container
            .remove_schedule()
            .execute(&ScheduleId::new(id))
            .map_err(AppFailure::new)
    }

    /// Speaks every schedule that is due (each one exactly once).
    pub fn run_due_schedules(
        &self,
        listener: Option<String>,
    ) -> Result<Vec<DueMessage>, AppFailure> {
        let outcomes = self
            .container
            .run_due_schedules()
            .with_listener(non_empty(listener))
            .execute()
            .map_err(AppFailure::new)?;
        Ok(outcomes
            .into_iter()
            .map(|o| match o.result {
                Ok(m) => DueMessage {
                    schedule_id: o.schedule_id.to_string(),
                    text: Some(m.message.text().to_string()),
                    tone: Some(m.script.tone.to_string()),
                    audio_path: Some(m.receipt),
                    error: None,
                },
                Err(e) => DueMessage {
                    schedule_id: o.schedule_id.to_string(),
                    text: None,
                    tone: None,
                    audio_path: None,
                    error: Some(e.to_string()),
                },
            })
            .collect())
    }

    /// The next `limit` firings of all schedules, soonest first, within the
    /// look-ahead horizon (iOS keeps at most 64 pending notifications).
    #[flutter_rust_bridge::frb(sync)]
    pub fn upcoming_alarms(&self, limit: u32) -> Result<Vec<Alarm>, AppFailure> {
        let now = Local::now().naive_local();
        let limit = limit as usize;
        let schedules = self.container.schedules.list().map_err(AppFailure::new)?;
        let mut alarms: Vec<Alarm> = schedules
            .iter()
            .flat_map(|s| {
                s.upcoming(now, limit).into_iter().filter_map(|t| {
                    epoch_ms(t).map(|at_epoch_ms| Alarm {
                        schedule_id: s.id.to_string(),
                        at_epoch_ms,
                    })
                })
            })
            .collect();
        alarms.sort_by_key(|a| a.at_epoch_ms);
        alarms.truncate(limit);
        Ok(alarms)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::session::tests::local_session;

    fn request(speaker_id: &str, kind: ScheduleKind) -> ScheduleRequest {
        ScheduleRequest {
            speaker_id: speaker_id.into(),
            kind,
            at: None,
            days: None,
            every_minutes: None,
            within: None,
            quiet: None,
            theme: Some("good-morning".into()),
            text: None,
            tone: None,
            pitch_shift: 0.0,
        }
    }

    #[test]
    fn creates_lists_and_removes_schedules() {
        let s = local_session("schedules");
        let p = s
            .register_speaker("Vó".into(), None, Some("Vó".into()), None)
            .unwrap();
        let daily = s
            .create_schedule(ScheduleRequest {
                at: Some("08:00".into()),
                tone: Some("whisper".into()),
                ..request(&p.id, ScheduleKind::Daily)
            })
            .unwrap();
        assert!(
            daily
                .next_due
                .as_deref()
                .is_some_and(|d| d.ends_with("08:00")),
            "{daily:?}"
        );
        assert_eq!(daily.tone_label, "whisper");
        let weekly = s
            .create_schedule(ScheduleRequest {
                at: Some("07:30".into()),
                days: Some("mon,fri".into()),
                quiet: Some("22:00-07:00".into()),
                ..request(&p.id, ScheduleKind::Weekly)
            })
            .unwrap();
        assert_eq!(weekly.tone_label, "suggested by the theme");
        assert_eq!(s.list_schedules().unwrap().len(), 2);
        s.remove_schedule(daily.id.clone()).unwrap();
        assert_eq!(s.list_schedules().unwrap().len(), 1);
        assert!(s.remove_schedule(daily.id).is_err());
    }

    #[test]
    fn invalid_rules_are_refused_with_a_clear_message() {
        let s = local_session("bad-schedules");
        let p = s
            .register_speaker("Vó".into(), None, Some("Vó".into()), None)
            .unwrap();
        let no_time = s
            .create_schedule(request(&p.id, ScheduleKind::Daily))
            .unwrap_err();
        assert!(no_time.message.contains("time"), "{no_time}");
        let bad_day = s
            .create_schedule(ScheduleRequest {
                at: Some("08:00".into()),
                days: Some("xyz".into()),
                ..request(&p.id, ScheduleKind::Weekly)
            })
            .unwrap_err();
        assert!(bad_day.message.contains("weekday"), "{bad_day}");
        let no_every = s
            .create_schedule(request(&p.id, ScheduleKind::Interval))
            .unwrap_err();
        assert!(no_every.message.contains("minutes"), "{no_every}");
    }

    #[test]
    fn upcoming_alarms_are_sorted_epoch_times_and_capped() {
        let s = local_session("alarms");
        let p = s
            .register_speaker("Vó".into(), None, Some("Vó".into()), None)
            .unwrap();
        for at in ["08:00", "20:00"] {
            s.create_schedule(ScheduleRequest {
                at: Some(at.into()),
                ..request(&p.id, ScheduleKind::Daily)
            })
            .unwrap();
        }
        let alarms = s.upcoming_alarms(10).unwrap();
        assert_eq!(alarms.len(), 10);
        assert!(alarms
            .windows(2)
            .all(|w| w[0].at_epoch_ms <= w[1].at_epoch_ms));
        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;
        assert!(alarms[0].at_epoch_ms > now_ms);
        assert!(
            s.upcoming_alarms(100).unwrap().len() <= 16,
            "8-day horizon, 2 per day"
        );
    }

    #[test]
    fn nothing_is_due_right_after_creation() {
        let s = local_session("due");
        let p = s
            .register_speaker("Vó".into(), None, Some("Vó".into()), None)
            .unwrap();
        s.create_schedule(ScheduleRequest {
            at: Some("08:00".into()),
            ..request(&p.id, ScheduleKind::Daily)
        })
        .unwrap();
        assert!(s.run_due_schedules(None).unwrap().is_empty());
    }
}
