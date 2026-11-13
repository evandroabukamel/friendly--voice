//! Command line interface (presentation layer).
mod args;

use args::*;
use clap::Parser;
use friendly_voice_application::{
    use_cases::{AddVoiceSampleInput, CreateScheduleInput, RegisterSpeakerInput, SpeakRequest},
    AppError,
};
use friendly_voice_bootstrap::{parse, Container};
use friendly_voice_domain::{
    ids::{ScheduleId, SpeakerId},
    language::{Language, SUPPORTED},
    message::{MessageCategory, MessageContent},
    schedule::{Recurrence, Schedule},
    tone::{ToneStyle, VoiceTone},
    voice::{VoiceGender, VoiceProfile},
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
            eprintln!("error: {e}");
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
        Command::Speak(a) => {
            let out = c
                .speak_message()
                .execute(SpeakRequest {
                    speaker_id: SpeakerId::new(a.speaker),
                    content: content(&a.content)?,
                    listener,
                    tone: tone(&a.tone)?,
                })
                .map_err(app_err)?;
            println!("💬 {}", out.message.text());
            println!(
                "   tone: {}  language: {}",
                out.script.tone, out.script.language
            );
            println!("🔊 {} [{}]", out.receipt, c.engine_name);
            Ok(())
        }
        Command::Schedule(cmd) => schedule(&c, cmd, listener),
        Command::Themes => {
            for cat in MessageCategory::ALL {
                println!(
                    "{:<13} ({:<11}) initial tone: {}",
                    cat.slug(),
                    cat.pt_slug(),
                    ToneStyle::default_for(Some(cat))
                );
            }
            Ok(())
        }
        Command::Tones => {
            for t in ToneStyle::ALL {
                println!("{:<9} ({:<10}) {}", t.slug(), t.pt_slug(), t.description());
            }
            println!(
                "\nFine tuning: --pitch-shift <semitones> (-6 to +6) makes the voice higher or lower."
            );
            Ok(())
        }
        Command::Languages => {
            for (code, name) in SUPPORTED {
                println!("{code:<3} {name}");
            }
            println!("\nA region may follow the code, e.g. pt-BR, en-US. Themes are in Portuguese; other languages speak --text.");
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
            language,
        } => {
            let language: Language = language.parse().map_err(|e: DomainError| e.to_string())?;
            let s = c
                .register_speaker()
                .execute(RegisterSpeakerInput {
                    name,
                    relationship,
                    consent_granted_by: consent_by,
                    language: Some(language),
                })
                .map_err(app_err)?;
            println!(
                "Person registered: {} (id: {}, speaks {})",
                s.name,
                s.id,
                s.language.name()
            );
            println!("Consent: {}", s.consent.statement);
        }
        PersonCmd::List => {
            for s in c.speakers.list().map_err(|e| e.to_string())? {
                let voice = c.profiles.find(&s.id).map_err(|e| e.to_string())?;
                println!(
                    "{}  {}{}  [{}]  — voice: {}",
                    s.id,
                    s.name,
                    s.relationship
                        .map(|r| format!(" ({r})"))
                        .unwrap_or_default(),
                    s.language,
                    voice
                        .map(|v| format!("generated, {}", gender_label(v.gender)))
                        .unwrap_or("not generated".into())
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
        } => {
            let bytes = std::fs::read(&audio).map_err(|e| format!("{}: {e}", audio.display()))?;
            let s = c
                .add_voice_sample()
                .execute(AddVoiceSampleInput {
                    speaker_id: SpeakerId::new(speaker),
                    audio: bytes,
                    transcript: transcript.or_else(|| sidecar(&audio, "txt")),
                })
                .map_err(app_err)?;
            println!("Sample {} added ({:.1}s)", s.id, s.duration_secs);
            if let Some(t) = &s.transcript {
                println!("  Text: {t}");
            }
            println!(
                "  Mean pitch {:.0} Hz, spread {:.0} Hz, pauses {:.0}%",
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
                println!(
                    "{}  {:>5.1}s  {}",
                    s.id,
                    s.duration_secs,
                    s.transcript.as_deref().unwrap_or("")
                );
            }
        }
    }
    Ok(())
}

fn gender_label(gender: Option<VoiceGender>) -> &'static str {
    match gender {
        Some(VoiceGender::Female) => "female voice",
        Some(VoiceGender::Male) => "male voice",
        None => "gender not detected",
    }
}

fn print_voice(p: &VoiceProfile) {
    println!(
        "Language: {} ({})  ·  {} (mean pitch {:.0} Hz)",
        p.language.name(),
        p.language,
        gender_label(p.gender),
        p.prosody.mean_pitch_hz
    );
    println!(
        "Material: {} sample(s), {:.1}s",
        p.sample_count, p.total_audio_secs
    );
    let rate = if p.prosody.syllables_per_second > 0.0 {
        format!("{:.1} syllables/s, ", p.prosody.syllables_per_second)
    } else {
        String::new()
    };
    println!(
        "Prosody: pitch spread ±{:.0} Hz, {rate}pauses {:.0}%",
        p.prosody.pitch_spread_hz / 2.0,
        p.prosody.pause_ratio * 100.0
    );
}

fn voice(c: &Container, cmd: VoiceCmd) -> CliResult {
    match cmd {
        VoiceCmd::Generate { speaker } => {
            let p = c
                .build_voice_profile()
                .execute(&SpeakerId::new(speaker))
                .map_err(app_err)?;
            println!(
                "Voice generated with the {} engine: {}",
                c.engine_name, p.engine_voice_id
            );
            print_voice(&p);
        }
        VoiceCmd::Show { speaker } => {
            let p = c
                .profiles
                .find(&SpeakerId::new(&speaker))
                .map_err(|e| e.to_string())?
                .ok_or(format!(
                    "the voice of '{speaker}' has not been generated yet"
                ))?;
            println!("Engine voice: {}", p.engine_voice_id);
            print_voice(&p);
        }
    }
    Ok(())
}

fn describe(s: &Schedule) -> String {
    let rule = match &s.recurrence {
        Recurrence::Once { at } => format!("once at {}", at.format("%Y-%m-%d %H:%M")),
        Recurrence::Daily { at } => format!("every day at {}", at.format("%H:%M")),
        Recurrence::Weekly { days, at } => format!(
            "{} at {}",
            days.iter()
                .map(|d| parse::weekday_short(*d))
                .collect::<Vec<_>>()
                .join(","),
            at.format("%H:%M")
        ),
        Recurrence::Interval {
            every_minutes,
            within,
        } => format!(
            "every {every_minutes} min{}",
            within
                .map(|w| format!(
                    " between {}-{}",
                    w.start.format("%H:%M"),
                    w.end.format("%H:%M")
                ))
                .unwrap_or_default()
        ),
    };
    let what = match &s.content {
        MessageContent::Category(c) => format!("theme {c}"),
        MessageContent::FixedText(t) => format!("\"{t}\""),
    };
    let quiet = s
        .quiet_hours
        .map(|q| {
            format!(
                ", quiet {}-{}",
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
        .unwrap_or_else(|| "suggested by the theme".into());
    format!(
        "{}  person {}  {rule}{quiet}  {what}  tone: {tone}  next: {next}",
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
            println!("Scheduled: {}", describe(&s));
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
            println!("Removed.");
        }
        ScheduleCmd::Run {
            watch,
            interval_secs,
        } => {
            let runner = c.run_due_schedules().with_listener(listener);
            loop {
                for o in runner.execute().map_err(app_err)? {
                    match o.result {
                        Ok(m) => println!(
                            "[{}] 💬 {}  🔊 {}",
                            o.schedule_id,
                            m.message.text(),
                            m.receipt
                        ),
                        Err(e) => eprintln!("[{}] failed: {e}", o.schedule_id),
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
