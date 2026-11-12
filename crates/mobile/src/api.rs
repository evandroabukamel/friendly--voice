use friendly_voice_application::{
    use_cases::{AddVoiceSampleInput, CreateScheduleInput, RegisterSpeakerInput, SpeakRequest},
    AppError,
};
use friendly_voice_bootstrap::{parse, Container};
use friendly_voice_domain::{
    ids::{ScheduleId, SpeakerId},
    message::{MessageCategory, MessageContent},
    schedule::Recurrence,
    tone::{ToneStyle, VoiceTone},
    DomainError,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::PathBuf;
use std::sync::Mutex;

/// Configuração definida pelo app ao iniciar.
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
        .map_err(|_| "estado interno inválido".to_string())?
        .clone()
        .ok_or("chame configure() antes de usar o app")?;
    Container::new(&cfg.data_dir, cfg.engine_url.as_deref(), None)
}

fn non_empty(s: Option<String>) -> Option<String> {
    s.map(|s| s.trim().to_string()).filter(|s| !s.is_empty())
}

/// Define a pasta de dados (ex.: `context.filesDir`) e o servidor neural opcional.
pub fn configure(data_dir: String, engine_url: Option<String>) -> String {
    respond((|| {
        let cfg = Config {
            data_dir: PathBuf::from(data_dir),
            engine_url: non_empty(engine_url),
        };
        let engine = Container::new(&cfg.data_dir, cfg.engine_url.as_deref(), None)?.engine_name;
        *CONFIG.lock().map_err(|_| "estado interno inválido")? = Some(cfg);
        Ok(json!({ "engine": engine }))
    })())
}

/// Temas, tons e tom inicial sugerido por tema (para montar a interface).
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
    })))
}

pub fn list_speakers() -> String {
    respond((|| {
        let c = container()?;
        let speakers = c.speakers.list().map_err(|e| e.to_string())?;
        let mut out = Vec::new();
        for s in speakers {
            let voice = c.profiles.find(&s.id).map_err(|e| e.to_string())?;
            out.push(json!({
                "id": s.id,
                "name": s.name,
                "relationship": s.relationship,
                "voice_ready": voice.is_some(),
                "accent": voice.map(|v| v.accent.label),
            }));
        }
        Ok(Value::Array(out))
    })())
}

pub fn register_speaker(
    name: String,
    relationship: Option<String>,
    consent_by: Option<String>,
) -> String {
    respond((|| {
        let s = container()?
            .register_speaker()
            .execute(RegisterSpeakerInput {
                name,
                relationship: non_empty(relationship),
                consent_granted_by: non_empty(consent_by),
            })
            .map_err(app)?;
        to_value(&s)
    })())
}

pub fn add_sample(
    speaker_id: String,
    wav: Vec<u8>,
    transcript: Option<String>,
    observed_ipa: Option<String>,
) -> String {
    respond((|| {
        let s = container()?
            .add_voice_sample()
            .execute(AddVoiceSampleInput {
                speaker_id: SpeakerId::new(speaker_id),
                audio: wav,
                transcript: non_empty(transcript),
                observed_ipa: non_empty(observed_ipa),
            })
            .map_err(app)?;
        Ok(json!({
            "id": s.id,
            "duration_secs": s.duration_secs,
            "transcript": s.transcript,
            "citation_ipa": s.canonical.ipa(),
            "has_observed_phonemes": s.observed.is_some(),
        }))
    })())
}

pub fn build_voice(speaker_id: String) -> String {
    respond((|| {
        let out = container()?
            .build_voice_profile()
            .execute(&SpeakerId::new(speaker_id))
            .map_err(app)?;
        let rules: Vec<Value> = out
            .accent_analysis
            .iter()
            .flat_map(|a| a.evidence.iter())
            .filter(|e| e.opportunities > 0)
            .map(|e| {
                json!({
                    "description": e.description,
                    "phenomenon": e.phenomenon,
                    "realizations": e.realizations,
                    "opportunities": e.opportunities,
                    "adopted": e.adopted,
                })
            })
            .collect();
        Ok(json!({
            "accent": out.profile.accent.label,
            "prosody": out.profile.prosody,
            "sample_count": out.profile.sample_count,
            "evidence": rules,
        }))
    })())
}

/// Tom pedido pela interface: estilo vazio = sugerido pelo tema.
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

/// Fala uma mensagem agora. Devolve o caminho do WAV gerado.
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
                accent_intensity: 1.0,
                tone: tone(tone_style, pitch_shift)?,
            })
            .map_err(app)?;
        Ok(json!({
            "text": out.message.text(),
            "tone": out.script.tone.to_string(),
            "ipa": out.script.pronunciation.ipa(),
            "audio_path": out.receipt,
        }))
    })())
}

/// Pedido de agendamento vindo do app (JSON).
#[derive(Debug, Deserialize)]
pub struct ScheduleRequest {
    pub speaker_id: String,
    /// "diario" | "semanal" | "intervalo" | "uma-vez"
    pub kind: String,
    /// "HH:MM" (diário/semanal) ou "AAAA-MM-DD HH:MM" (uma vez)
    #[serde(default)]
    pub at: Option<String>,
    /// "seg,qua,sex" (semanal)
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
    let at = || r.at.as_deref().ok_or("informe o horário".to_string());
    Ok(match r.kind.as_str() {
        "diario" => Recurrence::Daily {
            at: parse::time(at()?)?,
        },
        "semanal" => {
            let days = r.days.as_deref().ok_or("informe os dias da semana")?;
            let (days, at) = parse::weekly(&format!("{days}@{}", at()?))?;
            Recurrence::Weekly { days, at }
        }
        "intervalo" => Recurrence::Interval {
            every_minutes: r.every_minutes.ok_or("informe o intervalo em minutos")?,
            within: non_empty(r.within.clone())
                .as_deref()
                .map(parse::window)
                .transpose()?,
        },
        "uma-vez" => Recurrence::Once {
            at: parse::datetime(at()?)?,
        },
        other => return Err(format!("tipo de agendamento desconhecido: {other}")),
    })
}

pub fn create_schedule(request_json: String) -> String {
    respond((|| {
        let r: ScheduleRequest =
            serde_json::from_str(&request_json).map_err(|e| format!("pedido inválido: {e}"))?;
        let s = container()?
            .create_schedule()
            .execute(CreateScheduleInput {
                speaker_id: SpeakerId::new(r.speaker_id.clone()),
                recurrence: recurrence(&r)?,
                content: content(r.category.clone(), r.text.clone())?,
                tone: tone(r.tone.clone(), r.pitch_shift)?,
                quiet_hours: non_empty(r.quiet.clone())
                    .as_deref()
                    .map(parse::window)
                    .transpose()?,
            })
            .map_err(app)?;
        let mut v = to_value(&s)?;
        v["next_due"] = json!(s.next_due().map(|d| d.format("%d/%m %H:%M").to_string()));
        Ok(v)
    })())
}

pub fn list_schedules() -> String {
    respond((|| {
        let all = container()?.list_schedules().execute().map_err(app)?;
        let mut out = Vec::new();
        for s in all {
            let mut v = to_value(&s)?;
            v["next_due"] = json!(s
                .next_due()
                .filter(|_| s.enabled)
                .map(|d| d.format("%d/%m %H:%M").to_string()));
            v["tone_label"] = json!(s
                .tone
                .map(|t| t.to_string())
                .unwrap_or("sugerido pelo tema".into()));
            out.push(v);
        }
        Ok(Value::Array(out))
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

/// Dispara os agendamentos vencidos (chamado pelo WorkManager).
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

    /// Fluxo do app: configurar → cadastrar → amostras → voz → falar com tom → agendar.
    #[test]
    fn app_flow_through_the_bridge() {
        let dir = std::env::temp_dir().join(format!("fv-mobile-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        data(&configure(dir.display().to_string(), None));

        let cat = data(&catalog());
        assert_eq!(cat["tones"].as_array().unwrap().len(), 6);

        let err: Value = serde_json::from_str(&register_speaker("Vó".into(), None, None)).unwrap();
        assert_eq!(err["ok"], false);
        let id = data(&register_speaker(
            "Vó".into(),
            Some("avó".into()),
            Some("Vó".into()),
        ))["id"]
            .as_str()
            .unwrap()
            .to_string();

        for (txt, ipa) in [
            (
                "Leite quente de noite, tudo certo.",
                "lejtʃi kẽtʃi dʒi nojtʃi tudu sɛhtu",
            ),
            (
                "A festa é mesmo na porta do mar.",
                "a fɛʃta ɛ meʒmu na pɔhta du ma",
            ),
        ] {
            data(&add_sample(
                id.clone(),
                wav(6.0),
                Some(txt.into()),
                Some(ipa.into()),
            ));
        }
        let voice = data(&build_voice(id.clone()));
        assert!(voice["evidence"].as_array().unwrap().len() > 3);

        let spoken = data(&speak(
            id.clone(),
            Some("calma".into()),
            None,
            Some("sussurrado".into()),
            -1.0,
            Some("Evandro".into()),
        ));
        assert_eq!(spoken["tone"], "sussurrado (-1 st)");
        assert!(std::path::Path::new(spoken["audio_path"].as_str().unwrap()).exists());
        let default_tone = data(&speak(
            id.clone(),
            Some("calma".into()),
            None,
            None,
            0.0,
            None,
        ));
        assert_eq!(default_tone["tone"], "calmo");

        let req = json!({ "speaker_id": id, "kind": "semanal", "days": "seg,sex", "at": "07:30", "category": "bom-dia", "tone": "carinhoso" });
        let s = data(&create_schedule(req.to_string()));
        assert_eq!(s["tone"]["style"], "Warm");
        assert_eq!(data(&list_schedules())[0]["tone_label"], "carinhoso");
        data(&remove_schedule(s["id"].as_str().unwrap().into()));
        data(&run_due_schedules(None));

        let bad: Value =
            serde_json::from_str(&speak(id, None, None, Some("bravo".into()), 0.0, None)).unwrap();
        assert!(bad["error"].as_str().unwrap().contains("bravo"));
        std::fs::remove_dir_all(dir).unwrap();
    }
}
