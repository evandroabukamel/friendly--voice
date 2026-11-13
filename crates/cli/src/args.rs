//! Command line arguments.
//!
//! Commands and options are in English. The former Portuguese names are kept
//! as hidden aliases (e.g. `pessoa adicionar --nome`), so existing scripts and
//! cron jobs keep working.
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    about = "Friendly Voice — positive messages in the voice of someone you love.",
    name = "friendly-voice",
    version
)]
pub struct Cli {
    #[arg(
        help = "Folder for the data (people, samples, voices, schedules, output).",
        long = "data-dir",
        alias = "dados",
        env = "FRIENDLY_VOICE_DATA",
        default_value = ".friendly-voice",
        global = true
    )]
    pub data_dir: PathBuf,
    #[arg(
        help = "URL of the neural voice server (requires building with `--features http-engine`).",
        long = "engine-url",
        alias = "motor-url",
        env = "FRIENDLY_VOICE_ENGINE_URL",
        global = true
    )]
    pub engine_url: Option<String>,
    #[arg(
        help = "Command to play the generated audio (e.g. \"aplay\", \"afplay\", \"ffplay -nodisp -autoexit\"), or \"windows\" to native Windows player.",
        long = "player",
        env = "FRIENDLY_VOICE_PLAYER",
        global = true
    )]
    pub player: Option<String>,
    #[arg(
        help = "Your name, to personalize the messages.",
        long = "my-name",
        alias = "meu-nome",
        env = "FRIENDLY_VOICE_ME",
        global = true
    )]
    pub listener: Option<String>,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(about = "People whose voice is used.", alias = "pessoa")]
    #[command(subcommand)]
    Person(PersonCmd),
    #[command(about = "Audio samples of the person.", alias = "amostra")]
    #[command(subcommand)]
    Sample(SampleCmd),
    #[command(about = "Voice generation (cloned from the samples).", alias = "voz")]
    #[command(subcommand)]
    Voice(VoiceCmd),
    #[command(about = "Asks for a message right now.", alias = "falar")]
    Speak(SpeakArgs),
    #[command(about = "Scheduled messages.", alias = "agenda")]
    #[command(subcommand)]
    Schedule(ScheduleCmd),
    #[command(about = "Lists the available message themes.", alias = "temas")]
    Themes,
    #[command(about = "Lists the available tones of voice.", alias = "tons")]
    Tones,
    #[command(
        about = "Lists the languages a person can speak (see `person add --language`).",
        alias = "idiomas"
    )]
    Languages,
}

#[derive(Debug, Subcommand)]
pub enum PersonCmd {
    #[command(about = "Registers the person of the voice.", alias = "adicionar")]
    Add {
        #[arg(long = "name", alias = "nome")]
        name: String,
        #[arg(long = "relationship", alias = "relacao")]
        relationship: Option<String>,
        #[arg(
            help = "Name of who authorized the use of the voice (required).",
            long = "consent-by",
            alias = "consentimento-de"
        )]
        consent_by: Option<String>,
        #[arg(
            help = "Language spoken in the recordings and the messages, e.g. pt-BR, en, es (see `languages`).",
            long = "language",
            alias = "idioma",
            default_value = "pt-BR"
        )]
        language: String,
    },
    #[command(about = "Lists the registered people.", alias = "listar")]
    List,
}

#[derive(Debug, Subcommand)]
pub enum SampleCmd {
    #[command(
        about = "Adds a .wav or .flac recording. If `<file>.txt` (transcript) exists next to it, it is used.",
        alias = "adicionar"
    )]
    Add {
        #[arg(long = "person", alias = "pessoa")]
        speaker: String,
        #[arg(long = "audio")]
        audio: PathBuf,
        #[arg(
            help = "What is said in the recording (optional).",
            long = "transcript",
            alias = "texto"
        )]
        transcript: Option<String>,
    },
    #[command(about = "Lists the person's samples.", alias = "listar")]
    List {
        #[arg(long = "person", alias = "pessoa")]
        speaker: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum VoiceCmd {
    #[command(
        about = "Clones the voice from all the samples and detects whether it is female or male.",
        alias = "gerar"
    )]
    Generate {
        #[arg(long = "person", alias = "pessoa")]
        speaker: String,
    },
    #[command(about = "Shows the generated voice.", alias = "mostrar")]
    Show {
        #[arg(long = "person", alias = "pessoa")]
        speaker: String,
    },
}

#[derive(Debug, Args)]
pub struct ContentArgs {
    #[arg(
        help = "Theme (see `themes`). Default: motivation.",
        long = "theme",
        alias = "tema",
        conflicts_with = "text"
    )]
    pub category: Option<String>,
    #[arg(
        help = "Fixed text, written in the person's language.",
        long = "text",
        alias = "texto"
    )]
    pub text: Option<String>,
}

/// Tone of voice of the message. Without `--tone`, the initial tone suggested
/// by the theme is used (e.g. calm → calm, celebration → cheerful; free text → natural).
#[derive(Debug, Args)]
pub struct ToneArgs {
    #[arg(
        help = "natural, warm, cheerful, calm, firm or whisper (see `tones`).",
        long = "tone",
        alias = "tom"
    )]
    pub style: Option<String>,
    #[arg(
        help = "Makes the voice higher (+) or lower (-), in semitones (-6 to +6).",
        long = "pitch-shift",
        alias = "ajuste-tom",
        allow_hyphen_values = true
    )]
    pub pitch_shift: Option<f32>,
}

#[derive(Debug, Args)]
pub struct SpeakArgs {
    #[arg(long = "person", alias = "pessoa")]
    pub speaker: String,
    #[command(flatten)]
    pub content: ContentArgs,
    #[command(flatten)]
    pub tone: ToneArgs,
}

#[derive(Debug, Subcommand)]
pub enum ScheduleCmd {
    #[command(about = "Creates a schedule.", alias = "criar")]
    Create(Box<CreateScheduleArgs>),
    #[command(about = "Lists the schedules.", alias = "listar")]
    List,
    #[command(about = "Removes a schedule.", alias = "remover")]
    Remove { id: String },
    #[command(about = "Fires the schedules that are due.", alias = "executar")]
    Run {
        #[arg(
            help = "Keeps running and checking periodically.",
            long = "watch",
            alias = "continuo"
        )]
        watch: bool,
        #[arg(
            help = "Check interval in seconds (with --watch).",
            long = "interval",
            alias = "intervalo",
            default_value_t = 30
        )]
        interval_secs: u64,
    },
}

#[derive(Debug, Args)]
#[command(group(clap::ArgGroup::new("rule").required(true).args(["daily", "weekly", "every", "once"])))]
pub struct CreateScheduleArgs {
    #[arg(long = "person", alias = "pessoa")]
    pub speaker: String,
    #[arg(help = "Every day at a time: 08:00", long = "daily", alias = "diario")]
    pub daily: Option<String>,
    #[arg(
        help = "Weekdays and time: mon,wed,fri@07:30",
        long = "weekly",
        alias = "semanal"
    )]
    pub weekly: Option<String>,
    #[arg(help = "Every N minutes.", long = "every", alias = "a-cada")]
    pub every: Option<u32>,
    #[arg(
        help = "Time window for `--every`: 08:00-20:00",
        long = "between",
        alias = "entre",
        requires = "every"
    )]
    pub within: Option<String>,
    #[arg(help = "Only once: \"2026-11-20 09:00\"", long = "at", alias = "em")]
    pub once: Option<String>,
    #[arg(
        help = "Do not disturb: 22:00-07:00",
        long = "quiet",
        alias = "silencio"
    )]
    pub quiet: Option<String>,
    #[command(flatten)]
    pub content: ContentArgs,
    #[command(flatten)]
    pub tone: ToneArgs,
}
