use chrono::NaiveTime;
use friendly_voice_application::{
    use_cases::{
        AddVoiceSampleInput, CreateScheduleInput, RegisterSpeakerInput, SpeakRequest,
        SpeakerOverview,
    },
    AppError,
};
use friendly_voice_bootstrap::{parse, Container};
use friendly_voice_domain::{
    ids::{ScheduleId, SpeakerId},
    message::{MessageCategory, MessageContent},
    schedule::{Recurrence, Schedule, TimeWindow},
    speaker::Speaker,
    tone::{ToneStyle, VoiceTone},
    voice::VoiceSample,
    DomainError,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::PathBuf;
use std::sync::Mutex;

/// Configuration set by the app at startup.
#[derive(Debug, Clone, Default)]
pub struct Config {
    pub data_dir: PathBuf,
    pub engine_url: Option<String>,
}

static CONFIG: Mutex<Option<Config>> = Mutex::new(None);

type ApiResult = Result<Value, String>;

fn respond(result: ApiResult) -> String {
    match result {
        Ok(data) => json!({ "ok": true, "data": data }),
        Err(error) => json!({ "ok": false, "error": error }),
    }
    .to_string()
}

fn app(e: AppError) -> String {
    e.to_string()
}

fn domain(e: DomainError) -> String {
    e.to_string()
}

fn to_value<T: Serialize>(v: &T) -> ApiResult {
    serde_json::to_value(v).map_err(|e| e.to_string())
}

fn container() -> Result<Container, String> {
    let cfg = CONFIG
        .lock()
        .map_err(|_| "invalid internal state".to_string())?
        .clone()
        .ok_or("call configure() before using the app")?;
    Container::new(&cfg.data_dir, cfg.engine_url.as_deref(), None)
}

fn non_empty(s: Option<String>) -> Option<String> {
    s.map(|s| s.trim().to_string()).filter(|s| !s.is_empty())
}

/// Sets the data folder (e.g. `context.filesDir`) and the optional neural server.
pub fn configure(data_dir: String, engine_url: Option<String>) -> String {
    respond((|| {
        let cfg = Config {
            data_dir: PathBuf::from(data_dir),
            engine_url: non_empty(engine_url),
        };
        let engine = Container::new(&cfg.data_dir, cfg.engine_url.as_deref(), None)?.engine_name;
        *CONFIG.lock().map_err(|_| "invalid internal state")? = Some(cfg);
        Ok(json!({ "engine": engine }))
    })())
}

/// Themes, tones and the initial tone suggested per theme (to build the UI).
pub fn catalog() -> String {
    respond(Ok(json!({
        "categories": MessageCategory::ALL.iter().map(|c| json!({
            "slug": c.slug(),
            "default_tone": ToneStyle::default_for(Some(*c)).slug(),
        })).collect::<Vec<_>>(),
        "tones": ToneStyle::ALL.iter().map(|t| json!({
            "slug": t.slug(),
            "description": t.description(),
        })).collect::<Vec<_>>(),
        "max_pitch_shift": friendly_voice_domain::tone::MAX_PITCH_SHIFT_SEMITONES,
        "languages": friendly_voice_domain::language::SUPPORTED.iter().map(|(code, name)| json!({
            "code": code,
            "name": name,
        })).collect::<Vec<_>>(),
    })))
}

/// One person as the app shows it (item of `listSpeakers`).
fn speaker_item(o: &SpeakerOverview) -> Value {
    let s = &o.speaker;
    json!({
        "id": s.id,
        "name": s.name,
        "relationship": s.relationship,
        "language": s.language,
        "consent_granted_by": s.consent.granted_by,
        "listener_name": s.listener_name,
        "voice_ready": o.voice.is_some(),
        "gender": o.voice.as_ref().and_then(|v| v.gender),
        "sample_count": o.sample_count,
        "total_sample_secs": o.total_sample_secs,
    })
}

fn speaker_item_of(c: &Container, s: &Speaker) -> ApiResult {
    let o = c.list_speakers().one(s.clone()).map_err(app)?;
    Ok(speaker_item(&o))
}

pub fn list_speakers() -> String {
    respond((|| {
        let all = container()?.list_speakers().execute().map_err(app)?;
        Ok(Value::Array(all.iter().map(speaker_item).collect()))
    })())
}

/// One recording as the app shows it (item of `listSamples`).
fn sample_item(s: &VoiceSample) -> Value {
    json!({
        "id": s.id,
        "duration_secs": s.duration_secs,
        "transcript": s.transcript,
        "mean_pitch_hz": s.features.mean_pitch_hz,
    })
}

pub fn list_samples(speaker_id: String) -> String {
    respond((|| {
        let all = container()?
            .list_samples()
            .execute(&SpeakerId::new(speaker_id))
            .map_err(app)?;
        Ok(Value::Array(all.iter().map(sample_item).collect()))
    })())
}

/// Registers a person. `listener_name` is how this person calls the
/// listener; when set it wins over the name passed to `speak`/`runDueSchedules`.
pub fn register_speaker(
    name: String,
    relationship: Option<String>,
    consent_by: Option<String>,
    language: Option<String>,
    listener_name: Option<String>,
) -> String {
    respond((|| {
        let language = non_empty(language)
            .map(|l| l.parse().map_err(domain))
            .transpose()?;
        let c = container()?;
        let s = c
            .register_speaker()
            .execute(RegisterSpeakerInput {
                name,
                relationship: non_empty(relationship),
                consent_granted_by: non_empty(consent_by),
                language,
                listener_name: non_empty(listener_name),
            })
            .map_err(app)?;
        let mut v = to_value(&s)?;
        if let (Value::Object(v), Value::Object(item)) = (&mut v, speaker_item_of(&c, &s)?) {
            v.extend(item);
        }
        Ok(v)
    })())
}

/// Sets (or clears, with null/blank) how this person calls the listener.
pub fn set_listener_name(speaker_id: String, listener_name: Option<String>) -> String {
    respond((|| {
        let c = container()?;
        let s = c
            .set_listener_name()
            .execute(&SpeakerId::new(speaker_id), listener_name)
            .map_err(app)?;
        speaker_item_of(&c, &s)
    })())
}

pub fn add_sample(speaker_id: String, wav: Vec<u8>, transcript: Option<String>) -> String {
    respond((|| {
        let s = container()?
            .add_voice_sample()
            .execute(AddVoiceSampleInput {
                speaker_id: SpeakerId::new(speaker_id),
                audio: wav,
                transcript: non_empty(transcript),
            })
            .map_err(app)?;
        Ok(sample_item(&s))
    })())
}

pub fn build_voice(speaker_id: String) -> String {
    respond((|| {
        let profile = container()?
            .build_voice_profile()
            .execute(&SpeakerId::new(speaker_id))
            .map_err(app)?;
        Ok(json!({
            "language": profile.language,
            "gender": profile.gender,
            "prosody": profile.prosody,
            "sample_count": profile.sample_count,
            "total_audio_secs": profile.total_audio_secs,
        }))
    })())
}

/// Tone requested by the UI: empty style = suggested by the theme.
fn tone(style: Option<String>, pitch_shift: f32) -> Result<Option<VoiceTone>, String> {
    match non_empty(style) {
        None if pitch_shift == 0.0 => Ok(None),
        None => VoiceTone::new(ToneStyle::Natural, pitch_shift)
            .map(Some)
            .map_err(domain),
        Some(s) => {
            let style = s.parse::<ToneStyle>().map_err(domain)?;
            VoiceTone::new(style, pitch_shift).map(Some).map_err(domain)
        }
    }
}

fn content(category: Option<String>, text: Option<String>) -> Result<MessageContent, String> {
    Ok(match (non_empty(text), non_empty(category)) {
        (Some(t), _) => MessageContent::FixedText(t),
        (None, Some(c)) => MessageContent::Category(c.parse().map_err(domain)?),
        (None, None) => MessageContent::Category(MessageCategory::Motivation),
    })
}

/// Speaks a message now. Returns the path of the generated WAV.
pub fn speak(
    speaker_id: String,
    category: Option<String>,
    text: Option<String>,
    tone_style: Option<String>,
    pitch_shift: f32,
    listener: Option<String>,
) -> String {
    respond((|| {
        let out = container()?
            .speak_message()
            .execute(SpeakRequest {
                speaker_id: SpeakerId::new(speaker_id),
                content: content(category, text)?,
                listener: non_empty(listener),
                tone: tone(tone_style, pitch_shift)?,
            })
            .map_err(app)?;
        Ok(json!({
            "text": out.message.text(),
            "tone": out.script.tone.to_string(),
            "language": out.script.language,
            "audio_path": out.receipt,
        }))
    })())
}

/// Schedule request coming from the app (JSON).
#[derive(Debug, Deserialize)]
pub struct ScheduleRequest {
    pub speaker_id: String,
    /// "daily" | "weekly" | "interval" | "once"
    pub kind: String,
    /// "HH:MM" (daily/weekly) or "YYYY-MM-DD HH:MM" (once)
    #[serde(default)]
    pub at: Option<String>,
    /// "mon,wed,fri" (weekly)
    #[serde(default)]
    pub days: Option<String>,
    #[serde(default)]
    pub every_minutes: Option<u32>,
    /// "HH:MM-HH:MM"
    #[serde(default)]
    pub within: Option<String>,
    #[serde(default)]
    pub quiet: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub tone: Option<String>,
    #[serde(default)]
    pub pitch_shift: f32,
}

fn recurrence(r: &ScheduleRequest) -> Result<Recurrence, String> {
    let at = || r.at.as_deref().ok_or("provide the time".to_string());
    Ok(match r.kind.as_str() {
        "daily" | "diario" => Recurrence::Daily {
            at: parse::time(at()?)?,
        },
        "weekly" | "semanal" => {
            let days = r.days.as_deref().ok_or("provide the weekdays")?;
            let (days, at) = parse::weekly(&format!("{days}@{}", at()?))?;
            Recurrence::Weekly { days, at }
        }
        "interval" | "intervalo" => Recurrence::Interval {
            every_minutes: r.every_minutes.ok_or("provide the interval in minutes")?,
            within: non_empty(r.within.clone())
                .as_deref()
                .map(parse::window)
                .transpose()?,
        },
        "once" | "uma-vez" => Recurrence::Once {
            at: parse::datetime(at()?)?,
        },
        other => return Err(format!("unknown schedule kind: {other}")),
    })
}

/// US-15: revokes the consent of a person. Their recordings, voice (also on
/// the voice server) and cached speech are deleted, their schedules switched
/// off; the person is no longer listed. Idempotent.
pub fn revoke_consent(speaker_id: String, revoked_by: Option<String>) -> String {
    respond((|| {
        container()?
            .revoke_consent()
            .execute(&SpeakerId::new(speaker_id), non_empty(revoked_by))
            .map_err(app)?;
        Ok(Value::Null)
    })())
}

/// Deletes a person and everything made from their voice; their schedules
/// are switched off and kept. Idempotent (an unknown id is fine).
pub fn delete_speaker(speaker_id: String) -> String {
    respond((|| {
        container()?
            .delete_speaker()
            .execute(&SpeakerId::new(speaker_id))
            .map_err(app)?;
        Ok(Value::Null)
    })())
}

/// Parses the app's schedule request into the use-case input.
fn schedule_input(request_json: &str) -> Result<CreateScheduleInput, String> {
    let r: ScheduleRequest =
        serde_json::from_str(request_json).map_err(|e| format!("invalid request: {e}"))?;
    Ok(CreateScheduleInput {
        speaker_id: SpeakerId::new(r.speaker_id.clone()),
        recurrence: recurrence(&r)?,
        content: content(r.category.clone(), r.text.clone())?,
        tone: tone(r.tone.clone(), r.pitch_shift)?,
        quiet_hours: non_empty(r.quiet.clone())
            .as_deref()
            .map(parse::window)
            .transpose()?,
    })
}

pub fn create_schedule(request_json: String) -> String {
    respond((|| {
        let input = schedule_input(&request_json)?;
        let c = container()?;
        let s = c.create_schedule().execute(input).map_err(app)?;
        schedule_item(&c, &s)
    })())
}

/// Edits a schedule (same request as `create_schedule`), keeping its id and
/// its on/off state.
pub fn update_schedule(id: String, request_json: String) -> String {
    respond((|| {
        let input = schedule_input(&request_json)?;
        let c = container()?;
        let s = c
            .update_schedule()
            .execute(&ScheduleId::new(id), input)
            .map_err(app)?;
        schedule_item(&c, &s)
    })())
}

fn hhmm(t: NaiveTime) -> String {
    t.format("%H:%M").to_string()
}

fn window_text(w: TimeWindow) -> String {
    format!("{}-{}", hhmm(w.start), hhmm(w.end))
}

/// The schedule written back as the request that creates it; sending it to
/// `update_schedule` yields the same schedule.
fn schedule_request(s: &Schedule) -> Value {
    let (kind, at, days, every_minutes, within) = match &s.recurrence {
        Recurrence::Daily { at } => ("daily", Some(hhmm(*at)), None, None, None),
        Recurrence::Weekly { days, at } => (
            "weekly",
            Some(hhmm(*at)),
            Some(
                days.iter()
                    .map(|d| parse::weekday_short(*d))
                    .collect::<Vec<_>>()
                    .join(","),
            ),
            None,
            None,
        ),
        Recurrence::Interval {
            every_minutes,
            within,
        } => (
            "interval",
            None,
            None,
            Some(*every_minutes),
            within.map(window_text),
        ),
        Recurrence::Once { at } => (
            "once",
            Some(at.format("%Y-%m-%d %H:%M").to_string()),
            None,
            None,
            None,
        ),
    };
    let (category, text) = match &s.content {
        MessageContent::Category(c) => (Some(c.slug()), None),
        MessageContent::FixedText(t) => (None, Some(t.as_str())),
    };
    json!({
        "speaker_id": s.speaker_id,
        "kind": kind,
        "at": at,
        "days": days,
        "every_minutes": every_minutes,
        "within": within,
        "quiet": s.quiet_hours.map(window_text),
        "category": category,
        "text": text,
        "tone": s.tone.map(|t| t.style.slug()),
        "pitch_shift": s.tone.map_or(0.0, |t| t.pitch_shift_semitones),
    })
}

/// One schedule as the app shows it (item of `listSchedules`, also returned
/// by create/update/setEnabled).
fn schedule_item(c: &Container, s: &Schedule) -> ApiResult {
    let speaker = c.speakers.find(&s.speaker_id).map_err(|e| e.to_string())?;
    let voice_available = speaker.as_ref().is_some_and(|p| !p.is_revoked());
    let speaker_name = speaker.map(|p| p.name);
    let time = match &s.recurrence {
        Recurrence::Daily { at } | Recurrence::Weekly { at, .. } => Some(hhmm(*at)),
        Recurrence::Once { at } => Some(hhmm(at.time())),
        Recurrence::Interval { .. } => None,
    };
    let mut v = to_value(s)?;
    v["speaker_name"] = json!(speaker_name);
    v["voice_available"] = json!(voice_available);
    v["next_due"] = json!(s
        .next_due()
        .filter(|_| s.enabled)
        .map(|d| d.format("%d/%m %H:%M").to_string()));
    v["time"] = json!(time);
    v["tone_label"] = json!(s
        .tone
        .map(|t| t.to_string())
        .unwrap_or("suggested by the theme".into()));
    v["request"] = schedule_request(s);
    Ok(v)
}

pub fn list_schedules() -> String {
    respond((|| {
        let c = container()?;
        let all = c.list_schedules().execute().map_err(app)?;
        all.iter()
            .map(|s| schedule_item(&c, s))
            .collect::<Result<Vec<_>, _>>()
            .map(Value::Array)
    })())
}

/// Alarm on/off switch. Re-enabling only fires future occurrences.
pub fn set_schedule_enabled(id: String, enabled: bool) -> String {
    respond((|| {
        let c = container()?;
        let s = c
            .set_schedule_enabled()
            .execute(&ScheduleId::new(id), enabled)
            .map_err(app)?;
        schedule_item(&c, &s)
    })())
}

pub fn remove_schedule(id: String) -> String {
    respond((|| {
        container()?
            .remove_schedule()
            .execute(&ScheduleId::new(id))
            .map_err(app)?;
        Ok(Value::Null)
    })())
}

/// Fires the schedules that are due (called by WorkManager).
pub fn run_due_schedules(listener: Option<String>) -> String {
    respond((|| {
        let outcomes = container()?
            .run_due_schedules()
            .with_listener(non_empty(listener))
            .execute()
            .map_err(app)?;
        Ok(Value::Array(
            outcomes
                .into_iter()
                .map(|o| match o.result {
                    Ok(m) => json!({
                        "schedule_id": o.schedule_id,
                        "ok": true,
                        "text": m.message.text(),
                        "tone": m.script.tone.to_string(),
                        "audio_path": m.receipt,
                    }),
                    Err(e) => {
                        json!({ "schedule_id": o.schedule_id, "ok": false, "error": e.to_string() })
                    }
                })
                .collect(),
        ))
    })())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data(json: &str) -> Value {
        let v: Value = serde_json::from_str(json).unwrap();
        assert_eq!(v["ok"], true, "{json}");
        v["data"].clone()
    }

    fn wav(secs: f32) -> Vec<u8> {
        let mut cursor = std::io::Cursor::new(Vec::new());
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 16_000,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut w = hound::WavWriter::new(&mut cursor, spec).unwrap();
        for i in 0..(secs * 16_000.0) as usize {
            let t = i as f32 / 16_000.0;
            let v = if (t % 0.5) < 0.35 {
                (std::f32::consts::TAU * 200.0 * t).sin() * 0.3
            } else {
                0.0
            };
            w.write_sample((v * i16::MAX as f32) as i16).unwrap();
        }
        w.finalize().unwrap();
        cursor.into_inner()
    }

    fn rules_daily(id: &str) -> String {
        json!({ "speaker_id": id, "kind": "daily", "at": "08:15", "category": "calma" }).to_string()
    }

    /// App flow: configure → register → samples → voice → speak with a tone → schedule.
    #[test]
    fn app_flow_through_the_bridge() {
        let dir = std::env::temp_dir().join(format!("fv-mobile-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        data(&configure(dir.display().to_string(), None));

        let cat = data(&catalog());
        assert_eq!(cat["tones"].as_array().unwrap().len(), 6);

        assert!(cat["languages"].as_array().unwrap().len() > 20);

        let err: Value =
            serde_json::from_str(&register_speaker("Vó".into(), None, None, None, None)).unwrap();
        assert_eq!(err["ok"], false);
        let bad_language: Value = serde_json::from_str(&register_speaker(
            "Vó".into(),
            None,
            Some("Vó".into()),
            Some("xx".into()),
            None,
        ))
        .unwrap();
        assert_eq!(bad_language["ok"], false);
        let id = data(&register_speaker(
            "Vó".into(),
            Some("avó".into()),
            Some("Vó".into()),
            Some("pt-BR".into()),
            None,
        ))["id"]
            .as_str()
            .unwrap()
            .to_string();

        let named = data(&set_listener_name(id.clone(), Some(" Filhote ".into())));
        assert_eq!(named["listener_name"], "Filhote");
        assert_eq!(named["id"], id.as_str());
        assert_eq!(data(&list_speakers())[0], named);
        let missing: Value =
            serde_json::from_str(&set_listener_name("ghost".into(), None)).unwrap();
        assert_eq!(missing["ok"], false);
        let with_name = data(&register_speaker(
            "Pai".into(),
            None,
            Some("Pai".into()),
            None,
            Some("Campeão".into()),
        ));
        assert_eq!(with_name["listener_name"], "Campeão");

        data(&add_sample(
            id.clone(),
            wav(6.0),
            Some("Leite quente de noite, tudo certo.".into()),
        ));
        data(&add_sample(id.clone(), wav(6.0), None));
        let samples = data(&list_samples(id.clone()));
        assert_eq!(samples.as_array().unwrap().len(), 2);
        assert_eq!(
            samples[0]["transcript"],
            "Leite quente de noite, tudo certo."
        );
        assert!(samples[1]["mean_pitch_hz"].as_f64().unwrap() > 0.0);
        assert!(samples[1]["id"].is_string() && samples[1]["duration_secs"].is_number());
        let ghost: Value = serde_json::from_str(&list_samples("ghost".into())).unwrap();
        assert_eq!(ghost["ok"], false);
        let people = data(&list_speakers());
        let vo = people
            .as_array()
            .unwrap()
            .iter()
            .find(|p| p["id"] == id.as_str())
            .unwrap();
        assert_eq!(vo["sample_count"], 2);
        assert!((vo["total_sample_secs"].as_f64().unwrap() - 12.0).abs() < 0.1);
        assert_eq!(vo["voice_ready"], false);
        assert_eq!(vo["consent_granted_by"], "Vó");
        let voice = data(&build_voice(id.clone()));
        assert_eq!(voice["language"], "pt-BR");
        assert_eq!(voice["sample_count"], 2);
        let people = data(&list_speakers());
        let vo = people
            .as_array()
            .unwrap()
            .iter()
            .find(|p| p["id"] == id.as_str())
            .unwrap();
        assert_eq!(vo["voice_ready"], true);
        assert_eq!(vo["gender"], "female");

        let spoken = data(&speak(
            id.clone(),
            Some("calma".into()),
            None,
            Some("sussurrado".into()),
            -1.0,
            Some("Evandro".into()),
        ));
        assert_eq!(spoken["tone"], "whisper (-1 st)");
        assert!(
            spoken["text"].as_str().unwrap().contains("Filhote"),
            "{spoken}"
        );
        data(&set_listener_name(id.clone(), None));
        assert!(std::path::Path::new(spoken["audio_path"].as_str().unwrap()).exists());
        let default_tone = data(&speak(
            id.clone(),
            Some("calma".into()),
            None,
            None,
            0.0,
            None,
        ));
        assert_eq!(default_tone["tone"], "calm");

        // Synthesized speech is cached under the data folder, per person.
        let cached = || {
            std::fs::read_dir(dir.join("cache").join("speech").join(&id))
                .map(|d| d.count())
                .unwrap_or(0)
        };
        let before = cached();
        assert!(before >= 1, "speech is cached");
        data(&speak(
            id.clone(),
            None,
            Some("Oi, tudo bem?".into()),
            None,
            0.0,
            None,
        ));
        data(&speak(
            id.clone(),
            None,
            Some("Oi, tudo bem?".into()),
            None,
            0.0,
            None,
        ));
        assert_eq!(cached(), before + 1, "the same talk is stored once");

        let req = json!({ "speaker_id": id, "kind": "weekly", "days": "mon,fri", "at": "07:30", "category": "good-morning", "tone": "warm" });
        let s = data(&create_schedule(req.to_string()));
        assert_eq!(s["tone"]["style"], "Warm");
        assert_eq!(data(&list_schedules())[0]["tone_label"], "warm");
        let sid = s["id"].as_str().unwrap().to_string();
        let off = data(&set_schedule_enabled(sid.clone(), false));
        assert_eq!(off["enabled"], false);
        assert_eq!(off["next_due"], Value::Null);
        assert_eq!(data(&set_schedule_enabled(sid.clone(), false)), off);
        let on = data(&set_schedule_enabled(sid.clone(), true));
        assert_eq!(on["enabled"], true);
        assert!(on["next_due"].is_string());
        assert_eq!(data(&list_schedules())[0], on);
        let missing: Value =
            serde_json::from_str(&set_schedule_enabled("nope".into(), true)).unwrap();
        assert_eq!(missing["ok"], false);

        let edit = json!({ "speaker_id": id, "kind": "interval", "every_minutes": 90, "within": "09:00-18:00", "quiet": "12:00-13:00", "text": "Respira fundo", "tone": "calm", "pitch_shift": 1.0 });
        let edited = data(&update_schedule(sid.clone(), edit.to_string()));
        assert_eq!(edited["id"], sid);
        assert_eq!(edited["enabled"], true);
        assert_eq!(edited["content"]["FixedText"], "Respira fundo");
        assert_eq!(data(&list_schedules()).as_array().unwrap().len(), 1);
        let unknown: Value =
            serde_json::from_str(&update_schedule("nope".into(), edit.to_string())).unwrap();
        assert!(unknown["error"].as_str().unwrap().contains("not found"));
        assert_eq!(edited["speaker_name"], "Vó");
        assert_eq!(edited["time"], Value::Null);
        assert_eq!(
            edited["request"],
            json!({ "speaker_id": id, "kind": "interval", "at": null, "days": null, "every_minutes": 90, "within": "09:00-18:00", "quiet": "12:00-13:00", "category": null, "text": "Respira fundo", "tone": "calm", "pitch_shift": 1.0 })
        );

        // `request` round-trips: sending it back changes nothing.
        let rules = [
            json!({ "speaker_id": id, "kind": "daily", "at": "08:15", "category": "calma" }),
            json!({ "speaker_id": id, "kind": "weekly", "days": "fri,mon", "at": "07:30", "quiet": "22:00-06:00", "tone": "natural", "pitch_shift": -1.5 }),
            json!({ "speaker_id": id, "kind": "interval", "every_minutes": 45, "text": "Bebe água" }),
            json!({ "speaker_id": id, "kind": "once", "at": "2099-01-02 09:05", "category": "good-morning", "tone": "warm" }),
        ];
        for rule in rules {
            let created = data(&create_schedule(rule.to_string()));
            let cid = created["id"].as_str().unwrap().to_string();
            let again = data(&update_schedule(
                cid.clone(),
                created["request"].to_string(),
            ));
            assert_eq!(again, created, "{rule}");
            data(&remove_schedule(cid));
        }
        let daily = data(&create_schedule(rules_daily(&id)));
        assert_eq!(daily["time"], "08:15");
        assert_eq!(daily["request"]["category"], "calm");
        assert_eq!(daily["request"]["tone"], Value::Null);
        assert_eq!(daily["request"]["pitch_shift"], 0.0);
        data(&remove_schedule(daily["id"].as_str().unwrap().into()));
        data(&remove_schedule(s["id"].as_str().unwrap().into()));
        data(&run_due_schedules(None));

        let bad: Value = serde_json::from_str(&speak(
            id.clone(),
            None,
            None,
            Some("bravo".into()),
            0.0,
            None,
        ))
        .unwrap();
        assert!(bad["error"].as_str().unwrap().contains("bravo"));

        // US-15: revoke consent / delete a person.
        let failed = |json: String| -> String {
            let v: Value = serde_json::from_str(&json).unwrap();
            assert_eq!(v["ok"], false, "{json}");
            v["error"].as_str().unwrap().to_string()
        };
        let aunt = data(&register_speaker(
            "Tia".into(),
            None,
            Some("Tia".into()),
            None,
            None,
        ))["id"]
            .as_str()
            .unwrap()
            .to_string();
        data(&add_sample(aunt.clone(), wav(12.0), None));
        data(&build_voice(aunt.clone()));
        let rule = json!({ "speaker_id": aunt, "kind": "daily", "at": "09:00", "text": "Oi" });
        let alarm = data(&create_schedule(rule.to_string()));
        assert_eq!(alarm["voice_available"], true);
        let alarm_id = alarm["id"].as_str().unwrap().to_string();

        assert_eq!(
            data(&revoke_consent(aunt.clone(), Some("Tia".into()))),
            Value::Null
        );
        assert_eq!(
            data(&revoke_consent(aunt.clone(), None)),
            Value::Null,
            "idempotent"
        );
        assert!(!data(&list_speakers())
            .as_array()
            .unwrap()
            .iter()
            .any(|p| p["id"] == aunt.as_str()));
        assert!(data(&list_samples(aunt.clone()))
            .as_array()
            .unwrap()
            .is_empty());
        assert!(failed(speak(
            aunt.clone(),
            None,
            Some("Oi".into()),
            None,
            0.0,
            None
        ))
        .contains("revoked"));
        assert!(failed(add_sample(aunt.clone(), wav(6.0), None)).contains("revoked"));
        assert!(!dir.join("cache").join("speech").join(&aunt).exists());
        assert!(!dir.join("audio").join(&aunt).exists());

        let off = data(&list_schedules())
            .as_array()
            .unwrap()
            .iter()
            .find(|s| s["id"] == alarm_id.as_str())
            .unwrap()
            .clone();
        assert_eq!(off["enabled"], false);
        assert_eq!(off["voice_available"], false);
        assert_eq!(off["speaker_name"], "Tia");
        assert!(failed(set_schedule_enabled(alarm_id.clone(), true)).contains("edit the schedule"));
        let mut moved = off["request"].clone();
        moved["speaker_id"] = json!(id);
        let moved = data(&update_schedule(alarm_id.clone(), moved.to_string()));
        assert_eq!(moved["voice_available"], true);
        assert_eq!(moved["enabled"], false, "keeps its on/off state");
        assert_eq!(
            data(&set_schedule_enabled(alarm_id.clone(), true))["enabled"],
            true
        );

        let uncle = data(&register_speaker(
            "Tio".into(),
            None,
            Some("Tio".into()),
            None,
            None,
        ))["id"]
            .as_str()
            .unwrap()
            .to_string();
        let rule = json!({ "speaker_id": uncle, "kind": "daily", "at": "10:00", "text": "Oi" });
        let other = data(&create_schedule(rule.to_string()));
        assert_eq!(data(&delete_speaker(uncle.clone())), Value::Null);
        assert_eq!(
            data(&delete_speaker(uncle.clone())),
            Value::Null,
            "idempotent"
        );
        let orphan = data(&list_schedules())
            .as_array()
            .unwrap()
            .iter()
            .find(|s| s["id"] == other["id"])
            .unwrap()
            .clone();
        assert_eq!(orphan["speaker_name"], Value::Null);
        assert_eq!(orphan["voice_available"], false);
        assert_eq!(orphan["enabled"], false);
        std::fs::remove_dir_all(dir).unwrap();
    }
}
