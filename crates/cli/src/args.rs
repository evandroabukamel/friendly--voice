//! Command line arguments. Help texts are user-facing and therefore in Portuguese.
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
        long = "dados",
        env = "FRIENDLY_VOICE_DATA",
        default_value = ".friendly-voice",
        global = true
    )]
    pub data_dir: PathBuf,
    #[arg(
        help = "URL do servidor de voz neural (requer build com `--features http-engine`).",
        long = "motor-url",
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
        long = "meu-nome",
        env = "FRIENDLY_VOICE_ME",
        global = true
    )]
    pub listener: Option<String>,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(about = "Pessoas cuja voz será usada.")]
    #[command(subcommand)]
    Pessoa(PessoaCmd),
    #[command(about = "Amostras de áudio da pessoa.")]
    #[command(subcommand)]
    Amostra(AmostraCmd),
    #[command(about = "Geração da voz (timbre + sotaque + prosódia).")]
    #[command(subcommand)]
    Voz(VozCmd),
    #[command(about = "Mostra como um texto será pronunciado com o sotaque da pessoa.")]
    Pronuncia {
        #[arg(long = "pessoa")]
        speaker: String,
        #[arg(long = "intensidade", default_value_t = 1.0)]
        intensity: f32,
        text: String,
    },
    #[command(about = "Pede uma mensagem agora.")]
    Falar(FalarArgs),
    #[command(about = "Mensagens agendadas.")]
    #[command(subcommand)]
    Agenda(AgendaCmd),
    #[command(about = "Lista os temas de mensagens disponíveis.")]
    Temas,
    #[command(about = "Lista os tons de voz disponíveis.")]
    Tons,
}

#[derive(Debug, Subcommand)]
pub enum PessoaCmd {
    Adicionar {
        #[arg(long = "nome")]
        name: String,
        #[arg(long = "relacao")]
        relationship: Option<String>,
        #[arg(
            help = "Nome de quem autorizou o uso da voz (obrigatório).",
            long = "consentimento-de"
        )]
        consent_by: Option<String>,
    },
    Listar,
}

#[derive(Debug, Subcommand)]
pub enum AmostraCmd {
    #[command(
        about = "Adiciona um áudio .wav ou .flac. Se existirem `<arquivo>.txt` (transcrição) ou `<arquivo>.ipa` (fonemas ouvidos) ao lado, eles são usados."
    )]
    Adicionar {
        #[arg(long = "pessoa")]
        speaker: String,
        #[arg(long = "audio")]
        audio: PathBuf,
        #[arg(help = "O que é dito no áudio.", long = "texto")]
        transcript: Option<String>,
        #[arg(
            help = "Transcrição fonética (IPA) do que foi ouvido, ex.: \"lejtʃi kẽtʃi\".",
            long = "fonemas"
        )]
        observed_ipa: Option<String>,
    },
    Listar {
        #[arg(long = "pessoa")]
        speaker: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum VozCmd {
    #[command(about = "Analisa as amostras e gera a voz.")]
    Gerar {
        #[arg(long = "pessoa")]
        speaker: String,
    },
    #[command(about = "Mostra o perfil de voz/sotaque gerado.")]
    Mostrar {
        #[arg(long = "pessoa")]
        speaker: String,
    },
}

#[derive(Debug, Args)]
pub struct ContentArgs {
    #[arg(
        help = "Tema (veja `temas`). Padrão: motivacao.",
        long = "tema",
        conflicts_with = "text"
    )]
    pub category: Option<String>,
    #[arg(help = "Texto fixo.", long = "texto")]
    pub text: Option<String>,
}

/// Tone of voice of the message. Without `--tom`, the initial tone suggested
/// by the theme is used (e.g. calma → calmo, celebracao → animado; free text → natural).
#[derive(Debug, Args)]
pub struct ToneArgs {
    #[arg(
        help = "natural, carinhoso, animado, calmo, firme ou sussurrado (veja `tons`).",
        long = "tom"
    )]
    pub style: Option<String>,
    #[arg(
        help = "Deixa a voz mais aguda (+) ou mais grave (-), em semitons (-6 a +6).",
        long = "ajuste-tom",
        allow_hyphen_values = true
    )]
    pub pitch_shift: Option<f32>,
}

#[derive(Debug, Args)]
pub struct FalarArgs {
    #[arg(long = "pessoa")]
    pub speaker: String,
    #[command(flatten)]
    pub content: ContentArgs,
    #[command(flatten)]
    pub tone: ToneArgs,
    #[arg(
        help = "0.0 (sem sotaque) a 1.0 (sotaque completo).",
        long = "intensidade",
        default_value_t = 1.0
    )]
    pub intensity: f32,
}

#[derive(Debug, Subcommand)]
pub enum AgendaCmd {
    Criar(Box<CriarAgendaArgs>),
    Listar,
    Remover {
        id: String,
    },
    #[command(about = "Dispara os agendamentos vencidos.")]
    Executar {
        #[arg(help = "Fica rodando e verificando periodicamente.", long = "continuo")]
        watch: bool,
        #[arg(
            help = "Intervalo de verificação em segundos (modo contínuo).",
            long = "intervalo",
            default_value_t = 30
        )]
        interval_secs: u64,
        #[arg(long = "intensidade", default_value_t = 1.0)]
        intensity: f32,
    },
}

#[derive(Debug, Args)]
#[command(group(clap::ArgGroup::new("regra").required(true).args(["daily", "weekly", "every", "once"])))]
pub struct CriarAgendaArgs {
    #[arg(long = "pessoa")]
    pub speaker: String,
    #[arg(help = "Todo dia em um horário: 08:00", long = "diario")]
    pub daily: Option<String>,
    #[arg(help = "Dias da semana e horário: seg,qua,sex@07:30", long = "semanal")]
    pub weekly: Option<String>,
    #[arg(help = "A cada N minutos.", long = "a-cada")]
    pub every: Option<u32>,
    #[arg(
        help = "Janela para `--a-cada`: 08:00-20:00",
        long = "entre",
        requires = "every"
    )]
    pub within: Option<String>,
    #[arg(help = "Uma única vez: \"2026-11-20 09:00\"", long = "em")]
    pub once: Option<String>,
    #[arg(help = "Não perturbe: 22:00-07:00", long = "silencio")]
    pub quiet: Option<String>,
    #[command(flatten)]
    pub content: ContentArgs,
    #[command(flatten)]
    pub tone: ToneArgs,
}
