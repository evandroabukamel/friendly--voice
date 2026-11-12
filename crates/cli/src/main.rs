//! Command line interface (presentation layer). User-facing text is in Portuguese.
mod args;

use args::*;
use clap::Parser;
use friendly_voice_application::{
    use_cases::{AddVoiceSampleInput, CreateScheduleInput, RegisterSpeakerInput, SpeakRequest},
    AppError,
};
use friendly_voice_bootstrap::{parse, Container};
use friendly_voice_domain::{
    accent::RulePhenomenon,
    ids::{ScheduleId, SpeakerId},
    message::{MessageCategory, MessageContent},
    schedule::{Recurrence, Schedule},
    tone::{ToneStyle, VoiceTone},
    DomainError,
};
use std::path::Path;
use std::process::ExitCode;

type CliResult = Result<(), String>;

fn app_err(e: AppError) -> String {
    e.to_string()
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("erro: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run(cli: Cli) -> CliResult {
    let c = Container::new(&cli.data_dir, cli.engine_url.as_deref(), cli.player.clone())?;
    let listener = cli.listener.clone();
    match cli.command {
        Command::Person(cmd) => person(&c, cmd),
        Command::Sample(cmd) => sample(&c, cmd),
        Command::Voice(cmd) => voice(&c, cmd),
        Command::Pronounce {
            speaker,
            intensity,
            text,
        } => {
            let p = c
                .preview_pronunciation()
                .execute(&SpeakerId::new(speaker), &text, intensity)
                .map_err(app_err)?;
            println!("Sotaque:          {}", p.accent_label);
            println!("Forma de citação: /{}/", p.citation_ipa);
            println!("Com sotaque:      /{}/", p.accented_ipa);
            Ok(())
        }
        Command::Speak(a) => {
            let out = c
                .speak_message()
                .execute(SpeakRequest {
                    speaker_id: SpeakerId::new(a.speaker),
                    content: content(&a.content)?,
                    listener,
                    accent_intensity: a.intensity,
                    tone: tone(&a.tone)?,
                })
                .map_err(app_err)?;
            println!("💬 {}", out.message.text());
            println!("   tom: {}", out.script.tone);
            println!("   /{}/", out.script.pronunciation.ipa());
            println!("🔊 {} [{}]", out.receipt, c.engine_name);
            Ok(())
        }
        Command::Schedule(cmd) => schedule(&c, cmd, listener),
        Command::Themes => {
            for cat in MessageCategory::ALL {
                println!(
                    "{:<11} ({:<12}) tom inicial: {}",
                    cat.slug(),
                    cat.english_slug(),
                    ToneStyle::default_for(Some(cat))
                );
            }
            Ok(())
        }
        Command::Tones => {
            for t in ToneStyle::ALL {
                println!(
                    "{:<11} ({:<8}) {}",
                    t.slug(),
                    t.english_slug(),
                    t.description()
                );
            }
            println!(
                "\nAjuste fino: --pitch-shift <semitons> (-6 a +6) deixa a voz mais aguda ou grave."
            );
            Ok(())
        }
    }
}

fn tone(a: &ToneArgs) -> Result<Option<VoiceTone>, String> {
    if a.style.is_none() && a.pitch_shift.is_none() {
        return Ok(None);
    }
    let style = match &a.style {
        Some(s) => s.parse::<ToneStyle>().map_err(|e| e.to_string())?,
        None => ToneStyle::Natural,
    };
    VoiceTone::new(style, a.pitch_shift.unwrap_or(0.0))
        .map(Some)
        .map_err(|e: DomainError| e.to_string())
}

fn content(a: &ContentArgs) -> Result<MessageContent, String> {
    Ok(match (&a.text, &a.category) {
        (Some(t), _) => MessageContent::FixedText(t.clone()),
        (None, Some(cat)) => {
            MessageContent::Category(cat.parse().map_err(|e: DomainError| e.to_string())?)
        }
        (None, None) => MessageContent::Category(MessageCategory::Motivation),
    })
}

fn person(c: &Container, cmd: PersonCmd) -> CliResult {
    match cmd {
        PersonCmd::Add {
            name,
            relationship,
            consent_by,
        } => {
            let s = c
                .register_speaker()
                .execute(RegisterSpeakerInput {
                    name,
                    relationship,
                    consent_granted_by: consent_by,
                })
                .map_err(app_err)?;
            println!("Pessoa cadastrada: {} (id: {})", s.name, s.id);
            println!("Consentimento: {}", s.consent.statement);
        }
        PersonCmd::List => {
            for s in c.speakers.list().map_err(|e| e.to_string())? {
                let voice = c.profiles.find(&s.id).map_err(|e| e.to_string())?;
                println!(
                    "{}  {}{}  — voz: {}",
                    s.id,
                    s.name,
                    s.relationship
                        .map(|r| format!(" ({r})"))
                        .unwrap_or_default(),
                    voice
                        .map(|v| format!("gerada, sotaque {}", v.accent.label))
                        .unwrap_or("não gerada".into())
                );
            }
        }
    }
    Ok(())
}

fn sidecar(audio: &Path, ext: &str) -> Option<String> {
    std::fs::read_to_string(audio.with_extension(ext))
        .ok()
        .map(|s| s.trim().to_string())
}

fn sample(c: &Container, cmd: SampleCmd) -> CliResult {
    match cmd {
        SampleCmd::Add {
            speaker,
            audio,
            transcript,
            observed_ipa,
        } => {
            let bytes = std::fs::read(&audio).map_err(|e| format!("{}: {e}", audio.display()))?;
            let s = c
                .add_voice_sample()
                .execute(AddVoiceSampleInput {
                    speaker_id: SpeakerId::new(speaker),
                    audio: bytes,
                    transcript: transcript.or_else(|| sidecar(&audio, "txt")),
                    observed_ipa: observed_ipa.or_else(|| sidecar(&audio, "ipa")),
                })
                .map_err(app_err)?;
            println!("Amostra {} adicionada ({:.1}s)", s.id, s.duration_secs);
            println!("  Texto:    {}", s.transcript);
            println!("  Citação:  /{}/", s.canonical.ipa());
            match &s.observed {
                Some(o) => println!(
                    "  Ouvido:   /{}/",
                    o.iter().map(|p| p.symbol()).collect::<String>()
                ),
                None => println!(
                    "  Ouvido:   (sem fonemas — o sotaque não será aprendido desta amostra)"
                ),
            }
            println!(
                "  Pitch médio {:.0} Hz, variação {:.0} Hz, pausas {:.0}%",
                s.features.mean_pitch_hz,
                s.features.pitch_spread_hz,
                s.features.pause_ratio * 100.0
            );
        }
        SampleCmd::List { speaker } => {
            for s in c
                .samples
                .list_by_speaker(&SpeakerId::new(speaker))
                .map_err(|e| e.to_string())?
            {
                println!("{}  {:>5.1}s  {}", s.id, s.duration_secs, s.transcript);
            }
        }
    }
    Ok(())
}

fn phenomenon_pt(p: RulePhenomenon) -> &'static str {
    match p {
        RulePhenomenon::Segmental => "pronúncia",
        RulePhenomenon::WordEnding => "término",
        RulePhenomenon::Juncture => "junção",
    }
}

fn voice(c: &Container, cmd: VoiceCmd) -> CliResult {
    match cmd {
        VoiceCmd::Generate { speaker } => {
            let out = c
                .build_voice_profile()
                .execute(&SpeakerId::new(speaker))
                .map_err(app_err)?;
            let p = &out.profile;
            println!(
                "Voz gerada no motor {}: {}",
                c.engine_name, p.engine_voice_id
            );
            println!(
                "Material: {} amostra(s), {:.1}s",
                p.sample_count, p.total_audio_secs
            );
            println!(
                "Prosódia: pitch {:.0} Hz (±{:.0}), {:.1} sílabas/s, pausas {:.0}%",
                p.prosody.mean_pitch_hz,
                p.prosody.pitch_spread_hz / 2.0,
                p.prosody.syllables_per_second,
                p.prosody.pause_ratio * 100.0
            );
            match &out.accent_analysis {
                None => println!("Sotaque: nenhuma amostra com fonemas; usando pronúncia neutra."),
                Some(a) => {
                    println!("Sotaque: {}", p.accent.label);
                    if let Some((name, score)) = &a.closest_preset {
                        println!(
                            "  Mais próximo de: {name} ({:.0}% de semelhança)",
                            score * 100.0
                        );
                    }
                    println!("  {:<4} {:<11} {:>10}  fenômeno", "", "tipo", "frequência");
                    for e in a.evidence.iter().filter(|e| e.opportunities > 0) {
                        println!(
                            "  {:<4} {:<11} {:>4}/{:<5}  {}",
                            if e.adopted { "✔" } else { "·" },
                            phenomenon_pt(e.phenomenon),
                            e.realizations,
                            e.opportunities,
                            e.description
                        );
                    }
                }
            }
        }
        VoiceCmd::Show { speaker } => {
            let p = c
                .profiles
                .find(&SpeakerId::new(&speaker))
                .map_err(|e| e.to_string())?
                .ok_or(format!("voz de '{speaker}' ainda não gerada"))?;
            println!("Sotaque: {}", p.accent.label);
            for r in &p.accent.segment_rules {
                println!(
                    "  [{}] {:.0}%  {}",
                    phenomenon_pt(r.phenomenon),
                    r.confidence * 100.0,
                    r.description
                );
            }
            for r in &p.accent.juncture_rules {
                println!("  [junção] {:.0}%  {}", r.confidence * 100.0, r.description);
            }
        }
    }
    Ok(())
}

fn describe(s: &Schedule) -> String {
    let rule = match &s.recurrence {
        Recurrence::Once { at } => format!("uma vez em {}", at.format("%d/%m/%Y %H:%M")),
        Recurrence::Daily { at } => format!("todo dia às {}", at.format("%H:%M")),
        Recurrence::Weekly { days, at } => format!(
            "{} às {}",
            days.iter()
                .map(|d| parse::weekday_pt(*d))
                .collect::<Vec<_>>()
                .join(","),
            at.format("%H:%M")
        ),
        Recurrence::Interval {
            every_minutes,
            within,
        } => format!(
            "a cada {every_minutes} min{}",
            within
                .map(|w| format!(
                    " entre {}-{}",
                    w.start.format("%H:%M"),
                    w.end.format("%H:%M")
                ))
                .unwrap_or_default()
        ),
    };
    let what = match &s.content {
        MessageContent::Category(c) => format!("tema {c}"),
        MessageContent::FixedText(t) => format!("\"{t}\""),
    };
    let quiet = s
        .quiet_hours
        .map(|q| {
            format!(
                ", silêncio {}-{}",
                q.start.format("%H:%M"),
                q.end.format("%H:%M")
            )
        })
        .unwrap_or_default();
    let next = s
        .next_due()
        .filter(|_| s.enabled)
        .map(|n| n.format("%d/%m %H:%M").to_string())
        .unwrap_or("—".into());
    let tone = s
        .tone
        .map(|t| t.to_string())
        .unwrap_or_else(|| "sugerido pelo tema".into());
    format!(
        "{}  pessoa {}  {rule}{quiet}  {what}  tom: {tone}  próxima: {next}",
        s.id, s.speaker_id
    )
}

fn schedule(c: &Container, cmd: ScheduleCmd, listener: Option<String>) -> CliResult {
    match cmd {
        ScheduleCmd::Create(a) => {
            let recurrence = if let Some(t) = &a.daily {
                Recurrence::Daily {
                    at: parse::time(t)?,
                }
            } else if let Some(w) = &a.weekly {
                let (days, at) = parse::weekly(w)?;
                Recurrence::Weekly { days, at }
            } else if let Some(m) = a.every {
                Recurrence::Interval {
                    every_minutes: m,
                    within: a.within.as_deref().map(parse::window).transpose()?,
                }
            } else {
                Recurrence::Once {
                    at: parse::datetime(a.once.as_deref().unwrap_or_default())?,
                }
            };
            let s = c
                .create_schedule()
                .execute(CreateScheduleInput {
                    speaker_id: SpeakerId::new(a.speaker),
                    recurrence,
                    content: content(&a.content)?,
                    tone: tone(&a.tone)?,
                    quiet_hours: a.quiet.as_deref().map(parse::window).transpose()?,
                })
                .map_err(app_err)?;
            println!("Agendado: {}", describe(&s));
        }
        ScheduleCmd::List => {
            for s in c.list_schedules().execute().map_err(app_err)? {
                println!("{}", describe(&s));
            }
        }
        ScheduleCmd::Remove { id } => {
            c.remove_schedule()
                .execute(&ScheduleId::new(id))
                .map_err(app_err)?;
            println!("Removido.");
        }
        ScheduleCmd::Run {
            watch,
            interval_secs,
            intensity,
        } => {
            let runner = c
                .run_due_schedules()
                .with_listener(listener)
                .with_accent_intensity(intensity);
            loop {
                for o in runner.execute().map_err(app_err)? {
                    match o.result {
                        Ok(m) => println!(
                            "[{}] 💬 {}  🔊 {}",
                            o.schedule_id,
                            m.message.text(),
                            m.receipt
                        ),
                        Err(e) => eprintln!("[{}] falhou: {e}", o.schedule_id),
                    }
                }
                if !watch {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_secs(interval_secs.max(1)));
            }
        }
    }
    Ok(())
}
