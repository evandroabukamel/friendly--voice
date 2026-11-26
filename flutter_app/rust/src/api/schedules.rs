//! User-defined schedules and the alarms that wake the app to speak them.
use super::failure::AppFailure;
use super::messages::{content, tone};
use super::session::{non_empty, Session};
use chrono::{Local, NaiveDateTime, NaiveTime, TimeZone};
use friendly_voice_application::use_cases::CreateScheduleInput;
use friendly_voice_bootstrap::parse;
use friendly_voice_domain::{
    ids::{ScheduleId, SpeakerId},
    message::MessageContent,
    schedule::{Recurrence, Schedule, TimeWindow},
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
    /// The person's name; `None` once the person was deleted.
    pub speaker_name: Option<String>,
    /// False when the person was deleted or revoked: choose another voice.
    pub voice_available: bool,
    pub enabled: bool,
    /// Clock time `HH:MM` of daily, weekly and one-time rules (`None` for
    /// intervals).
    pub time: Option<String>,
    pub tone_label: String,
    /// Next firing as `dd/mm HH:MM`, if any (`None` when switched off).
    pub next_due: Option<String>,
    /// The request that creates this schedule: sending it back to
    /// `update_schedule` yields the same schedule.
    pub request: ScheduleRequest,
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

fn input(r: &ScheduleRequest) -> Result<CreateScheduleInput, AppFailure> {
    Ok(CreateScheduleInput {
        speaker_id: SpeakerId::new(r.speaker_id.clone()),
        recurrence: recurrence(r)?,
        content: content(r.theme.clone(), r.text.clone())?,
        tone: tone(r.tone.clone(), r.pitch_shift)?,
        quiet_hours: non_empty(r.quiet.clone())
            .as_deref()
            .map(parse::window)
            .transpose()
            .map_err(AppFailure::new)?,
    })
}

fn hhmm(t: NaiveTime) -> String {
    t.format("%H:%M").to_string()
}

fn window_text(w: TimeWindow) -> String {
    format!("{}-{}", hhmm(w.start), hhmm(w.end))
}

/// The schedule written back as the request that creates it.
fn request_of(s: &Schedule) -> ScheduleRequest {
    let blank = ScheduleRequest {
        speaker_id: s.speaker_id.to_string(),
        kind: ScheduleKind::Daily,
        at: None,
        days: None,
        every_minutes: None,
        within: None,
        quiet: s.quiet_hours.map(window_text),
        theme: None,
        text: None,
        tone: s.tone.map(|t| t.style.slug().to_string()),
        pitch_shift: s.tone.map_or(0.0, |t| t.pitch_shift_semitones),
    };
    let rule = match &s.recurrence {
        Recurrence::Daily { at } => ScheduleRequest {
            at: Some(hhmm(*at)),
            ..blank
        },
        Recurrence::Weekly { days, at } => ScheduleRequest {
            kind: ScheduleKind::Weekly,
            at: Some(hhmm(*at)),
            days: Some(
                days.iter()
                    .map(|d| parse::weekday_short(*d))
                    .collect::<Vec<_>>()
                    .join(","),
            ),
            ..blank
        },
        Recurrence::Interval {
            every_minutes,
            within,
        } => ScheduleRequest {
            kind: ScheduleKind::Interval,
            every_minutes: Some(*every_minutes),
            within: within.map(window_text),
            ..blank
        },
        Recurrence::Once { at } => ScheduleRequest {
            kind: ScheduleKind::Once,
            at: Some(at.format("%Y-%m-%d %H:%M").to_string()),
            ..blank
        },
    };
    match &s.content {
        MessageContent::Category(c) => ScheduleRequest {
            theme: Some(c.slug().to_string()),
            ..rule
        },
        MessageContent::FixedText(t) => ScheduleRequest {
            text: Some(t.clone()),
            ..rule
        },
    }
}

fn clock_time(r: &Recurrence) -> Option<String> {
    match r {
        Recurrence::Daily { at } | Recurrence::Weekly { at, .. } => Some(hhmm(*at)),
        Recurrence::Once { at } => Some(hhmm(at.time())),
        Recurrence::Interval { .. } => None,
    }
}

fn epoch_ms(local: NaiveDateTime) -> Option<i64> {
    Local
        .from_local_datetime(&local)
        .earliest()
        .map(|d| d.timestamp_millis())
}

impl Session {
    fn schedule_summary(&self, s: &Schedule) -> Result<ScheduleSummary, AppFailure> {
        let speaker = self
            .container
            .speakers
            .find(&s.speaker_id)
            .map_err(AppFailure::new)?;
        Ok(ScheduleSummary {
            id: s.id.to_string(),
            speaker_id: s.speaker_id.to_string(),
            voice_available: speaker.as_ref().is_some_and(|p| !p.is_revoked()),
            speaker_name: speaker.map(|p| p.name),
            enabled: s.enabled,
            time: clock_time(&s.recurrence),
            tone_label: s
                .tone
                .map(|t| t.to_string())
                .unwrap_or_else(|| "suggested by the theme".into()),
            next_due: s
                .next_due()
                .filter(|_| s.enabled)
                .map(|d| d.format("%d/%m %H:%M").to_string()),
            request: request_of(s),
        })
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn create_schedule(&self, request: ScheduleRequest) -> Result<ScheduleSummary, AppFailure> {
        let schedule = self
            .container
            .create_schedule()
            .execute(input(&request)?)
            .map_err(AppFailure::new)?;
        self.schedule_summary(&schedule)
    }

    /// Replaces the rule of a schedule, keeping its id and on/off state.
    /// Sending the same request again changes nothing.
    #[flutter_rust_bridge::frb(sync)]
    pub fn update_schedule(
        &self,
        id: String,
        request: ScheduleRequest,
    ) -> Result<ScheduleSummary, AppFailure> {
        let schedule = self
            .container
            .update_schedule()
            .execute(&ScheduleId::new(id), input(&request)?)
            .map_err(AppFailure::new)?;
        self.schedule_summary(&schedule)
    }

    /// The on/off switch. Switching on only fires future occurrences.
    #[flutter_rust_bridge::frb(sync)]
    pub fn set_schedule_enabled(
        &self,
        id: String,
        enabled: bool,
    ) -> Result<ScheduleSummary, AppFailure> {
        let schedule = self
            .container
            .set_schedule_enabled()
            .execute(&ScheduleId::new(id), enabled)
            .map_err(AppFailure::new)?;
        self.schedule_summary(&schedule)
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn list_schedules(&self) -> Result<Vec<ScheduleSummary>, AppFailure> {
        let all = self
            .container
            .list_schedules()
            .execute()
            .map_err(AppFailure::new)?;
        all.iter().map(|s| self.schedule_summary(s)).collect()
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
            .register_speaker("Vó".into(), None, Some("Vó".into()), None, None)
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
            .register_speaker("Vó".into(), None, Some("Vó".into()), None, None)
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
            .register_speaker("Vó".into(), None, Some("Vó".into()), None, None)
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
    fn a_listed_schedule_describes_itself_and_its_request_round_trips() {
        let s = local_session("round-trip");
        let p = s
            .register_speaker("Vó".into(), None, Some("Vó".into()), None, None)
            .unwrap();
        let sent = ScheduleRequest {
            at: Some("07:30".into()),
            days: Some("mon,fri".into()),
            quiet: Some("22:00-07:00".into()),
            tone: Some("warm".into()),
            pitch_shift: 1.0,
            ..request(&p.id, ScheduleKind::Weekly)
        };
        let created = s.create_schedule(sent.clone()).unwrap();
        assert!(created.enabled);
        assert_eq!(created.speaker_name.as_deref(), Some("Vó"));
        assert!(created.voice_available);
        assert_eq!(created.time.as_deref(), Some("07:30"));
        assert_eq!(created.request, sent);
        let again = s
            .update_schedule(created.id.clone(), created.request.clone())
            .unwrap();
        assert_eq!(again, created);
    }

    #[test]
    fn an_edit_keeps_the_id_and_the_switch() {
        let s = local_session("edit");
        let p = s
            .register_speaker("Vó".into(), None, Some("Vó".into()), None, None)
            .unwrap();
        let daily = s
            .create_schedule(ScheduleRequest {
                at: Some("08:00".into()),
                ..request(&p.id, ScheduleKind::Daily)
            })
            .unwrap();
        let off = s.set_schedule_enabled(daily.id.clone(), false).unwrap();
        assert!(!off.enabled);
        assert_eq!(off.next_due, None);
        assert!(s.upcoming_alarms(10).unwrap().is_empty());
        let edited = s
            .update_schedule(
                daily.id.clone(),
                ScheduleRequest {
                    at: Some("09:15".into()),
                    ..daily.request.clone()
                },
            )
            .unwrap();
        assert_eq!(edited.id, daily.id);
        assert!(!edited.enabled);
        assert_eq!(edited.time.as_deref(), Some("09:15"));
        assert!(s.set_schedule_enabled(daily.id, true).unwrap().enabled);
    }

    #[test]
    fn an_interval_has_no_clock_time() {
        let s = local_session("interval");
        let p = s
            .register_speaker("Vó".into(), None, Some("Vó".into()), None, None)
            .unwrap();
        let every = s
            .create_schedule(ScheduleRequest {
                every_minutes: Some(90),
                within: Some("09:00-18:00".into()),
                text: Some("Respira fundo".into()),
                theme: None,
                ..request(&p.id, ScheduleKind::Interval)
            })
            .unwrap();
        assert_eq!(every.time, None);
        assert_eq!(every.request.every_minutes, Some(90));
        assert_eq!(every.request.within.as_deref(), Some("09:00-18:00"));
        assert_eq!(every.request.text.as_deref(), Some("Respira fundo"));
    }

    #[test]
    fn a_schedule_whose_person_was_deleted_reports_its_voice_unavailable() {
        let s = local_session("unavailable");
        let p = s
            .register_speaker("Vó".into(), None, Some("Vó".into()), None, None)
            .unwrap();
        s.create_schedule(ScheduleRequest {
            at: Some("08:00".into()),
            ..request(&p.id, ScheduleKind::Daily)
        })
        .unwrap();
        s.delete_speaker(p.id).unwrap();
        let listed = &s.list_schedules().unwrap()[0];
        assert!(!listed.voice_available);
        assert_eq!(listed.speaker_name, None);
        assert!(!listed.enabled);
    }

    #[test]
    fn nothing_is_due_right_after_creation() {
        let s = local_session("due");
        let p = s
            .register_speaker("Vó".into(), None, Some("Vó".into()), None, None)
            .unwrap();
        s.create_schedule(ScheduleRequest {
            at: Some("08:00".into()),
            ..request(&p.id, ScheduleKind::Daily)
        })
        .unwrap();
        assert!(s.run_due_schedules(None).unwrap().is_empty());
    }
}
