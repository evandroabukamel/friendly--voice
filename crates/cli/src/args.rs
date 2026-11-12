//! Command line arguments.
//!
//! Commands and options are in English. The former Portuguese names are kept
//! as hidden aliases (e.g. `pessoa adicionar --nome`), so existing scripts and
//! cron jobs keep working. Help texts are user-facing and stay in Portuguese.
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    about = "Friendly Voice — mensagens positivas com a voz de quem você ama.",
    name = "friendly-voice",
    version
)]
pub struct Cli {
    #[arg(
        help = "Pasta onde ficam os dados (pessoas, amostras, vozes, agenda, saída).",
        long = "data-dir",
        alias = "dados",
        env = "FRIENDLY_VOICE_DATA",
        default_value = ".friendly-voice",
        global = true
    )]
    pub data_dir: PathBuf,
    #[arg(
        help = "URL do servidor de voz neural (requer build com `--features http-engine`).",
        long = "engine-url",
        alias = "motor-url",
        env = "FRIENDLY_VOICE_ENGINE_URL",
        global = true
    )]
    pub engine_url: Option<String>,
    #[arg(
        help = "Comando para tocar o áudio gerado (ex.: \"aplay\", \"afplay\", \"ffplay -nodisp -autoexit\").",
        long = "player",
        env = "FRIENDLY_VOICE_PLAYER",
        global = true
    )]
    pub player: Option<String>,
    #[arg(
        help = "Seu nome, para personalizar as mensagens.",
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
    #[command(about = "Pessoas cuja voz será usada.", alias = "pessoa")]
    #[command(subcommand)]
    Person(PersonCmd),
    #[command(about = "Amostras de áudio da pessoa.", alias = "amostra")]
    #[command(subcommand)]
    Sample(SampleCmd),
    #[command(about = "Geração da voz (timbre + sotaque + prosódia).", alias = "voz")]
    #[command(subcommand)]
    Voice(VoiceCmd),
    #[command(
        about = "Mostra como um texto será pronunciado com o sotaque da pessoa.",
        alias = "pronuncia"
    )]
    Pronounce {
        #[arg(long = "person", alias = "pessoa")]
        speaker: String,
        #[arg(long = "intensity", alias = "intensidade", default_value_t = 1.0)]
        intensity: f32,
        text: String,
    },
    #[command(about = "Pede uma mensagem agora.", alias = "falar")]
    Speak(SpeakArgs),
    #[command(about = "Mensagens agendadas.", alias = "agenda")]
    #[command(subcommand)]
    Schedule(ScheduleCmd),
    #[command(about = "Lista os temas de mensagens disponíveis.", alias = "temas")]
    Themes,
    #[command(about = "Lista os tons de voz disponíveis.", alias = "tons")]
    Tones,
}

#[derive(Debug, Subcommand)]
pub enum PersonCmd {
    #[command(about = "Cadastra a pessoa da voz.", alias = "adicionar")]
    Add {
        #[arg(long = "name", alias = "nome")]
        name: String,
        #[arg(long = "relationship", alias = "relacao")]
        relationship: Option<String>,
        #[arg(
            help = "Nome de quem autorizou o uso da voz (obrigatório).",
            long = "consent-by",
            alias = "consentimento-de"
        )]
        consent_by: Option<String>,
    },
    #[command(about = "Lista as pessoas cadastradas.", alias = "listar")]
    List,
}

#[derive(Debug, Subcommand)]
pub enum SampleCmd {
    #[command(
        about = "Adiciona um áudio .wav ou .flac. Se existirem `<arquivo>.txt` (transcrição) ou `<arquivo>.ipa` (fonemas ouvidos) ao lado, eles são usados.",
        alias = "adicionar"
    )]
    Add {
        #[arg(long = "person", alias = "pessoa")]
        speaker: String,
        #[arg(long = "audio")]
        audio: PathBuf,
        #[arg(help = "O que é dito no áudio.", long = "transcript", alias = "texto")]
        transcript: Option<String>,
        #[arg(
            help = "Transcrição fonética (IPA) do que foi ouvido, ex.: \"lejtʃi kẽtʃi\".",
            long = "phonemes",
            alias = "fonemas"
        )]
        observed_ipa: Option<String>,
    },
    #[command(about = "Lista as amostras da pessoa.", alias = "listar")]
    List {
        #[arg(long = "person", alias = "pessoa")]
        speaker: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum VoiceCmd {
    #[command(about = "Analisa as amostras e gera a voz.", alias = "gerar")]
    Generate {
        #[arg(long = "person", alias = "pessoa")]
        speaker: String,
    },
    #[command(about = "Mostra o perfil de voz/sotaque gerado.", alias = "mostrar")]
    Show {
        #[arg(long = "person", alias = "pessoa")]
        speaker: String,
    },
}

#[derive(Debug, Args)]
pub struct ContentArgs {
    #[arg(
        help = "Tema (veja `themes`). Padrão: motivacao.",
        long = "theme",
        alias = "tema",
        conflicts_with = "text"
    )]
    pub category: Option<String>,
    #[arg(help = "Texto fixo.", long = "text", alias = "texto")]
    pub text: Option<String>,
}

/// Tone of voice of the message. Without `--tone`, the initial tone suggested
/// by the theme is used (e.g. calma → calmo, celebracao → animado; free text → natural).
#[derive(Debug, Args)]
pub struct ToneArgs {
    #[arg(
        help = "natural, carinhoso, animado, calmo, firme ou sussurrado (veja `tones`).",
        long = "tone",
        alias = "tom"
    )]
    pub style: Option<String>,
    #[arg(
        help = "Deixa a voz mais aguda (+) ou mais grave (-), em semitons (-6 a +6).",
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
    #[arg(
        help = "0.0 (sem sotaque) a 1.0 (sotaque completo).",
        long = "intensity",
        alias = "intensidade",
        default_value_t = 1.0
    )]
    pub intensity: f32,
}

#[derive(Debug, Subcommand)]
pub enum ScheduleCmd {
    #[command(about = "Cria um agendamento.", alias = "criar")]
    Create(Box<CreateScheduleArgs>),
    #[command(about = "Lista os agendamentos.", alias = "listar")]
    List,
    #[command(about = "Remove um agendamento.", alias = "remover")]
    Remove { id: String },
    #[command(about = "Dispara os agendamentos vencidos.", alias = "executar")]
    Run {
        #[arg(
            help = "Fica rodando e verificando periodicamente.",
            long = "watch",
            alias = "continuo"
        )]
        watch: bool,
        #[arg(
            help = "Intervalo de verificação em segundos (modo contínuo).",
            long = "interval",
            alias = "intervalo",
            default_value_t = 30
        )]
        interval_secs: u64,
        #[arg(long = "intensity", alias = "intensidade", default_value_t = 1.0)]
        intensity: f32,
    },
}

#[derive(Debug, Args)]
#[command(group(clap::ArgGroup::new("rule").required(true).args(["daily", "weekly", "every", "once"])))]
pub struct CreateScheduleArgs {
    #[arg(long = "person", alias = "pessoa")]
    pub speaker: String,
    #[arg(
        help = "Todo dia em um horário: 08:00",
        long = "daily",
        alias = "diario"
    )]
    pub daily: Option<String>,
    #[arg(
        help = "Dias da semana e horário: seg,qua,sex@07:30 (ou mon,wed,fri@07:30)",
        long = "weekly",
        alias = "semanal"
    )]
    pub weekly: Option<String>,
    #[arg(help = "A cada N minutos.", long = "every", alias = "a-cada")]
    pub every: Option<u32>,
    #[arg(
        help = "Janela para `--every`: 08:00-20:00",
        long = "between",
        alias = "entre",
        requires = "every"
    )]
    pub within: Option<String>,
    #[arg(
        help = "Uma única vez: \"2026-11-20 09:00\"",
        long = "at",
        alias = "em"
    )]
    pub once: Option<String>,
    #[arg(help = "Não perturbe: 22:00-07:00", long = "quiet", alias = "silencio")]
    pub quiet: Option<String>,
    #[command(flatten)]
    pub content: ContentArgs,
    #[command(flatten)]
    pub tone: ToneArgs,
}
