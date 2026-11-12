use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

/// Friendly Voice — mensagens positivas com a voz de quem você ama.
#[derive(Debug, Parser)]
#[command(name = "friendly-voice", version, about)]
pub struct Cli {
    /// Pasta onde ficam os dados (pessoas, amostras, vozes, agenda, saída).
    #[arg(
        long = "dados",
        env = "FRIENDLY_VOICE_DATA",
        default_value = ".friendly-voice",
        global = true
    )]
    pub data_dir: PathBuf,
    /// URL do servidor de voz neural (requer build com `--features http-engine`).
    #[arg(long = "motor-url", env = "FRIENDLY_VOICE_ENGINE_URL", global = true)]
    pub engine_url: Option<String>,
    /// Comando para tocar o áudio gerado (ex.: "aplay", "afplay", "ffplay -nodisp -autoexit").
    #[arg(long = "player", env = "FRIENDLY_VOICE_PLAYER", global = true)]
    pub player: Option<String>,
    /// Seu nome, para personalizar as mensagens.
    #[arg(long = "meu-nome", env = "FRIENDLY_VOICE_ME", global = true)]
    pub listener: Option<String>,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Pessoas cuja voz será usada.
    #[command(subcommand)]
    Pessoa(PessoaCmd),
    /// Amostras de áudio da pessoa.
    #[command(subcommand)]
    Amostra(AmostraCmd),
    /// Geração da voz (timbre + sotaque + prosódia).
    #[command(subcommand)]
    Voz(VozCmd),
    /// Mostra como um texto será pronunciado com o sotaque da pessoa.
    Pronuncia {
        #[arg(long = "pessoa")]
        speaker: String,
        #[arg(long = "intensidade", default_value_t = 1.0)]
        intensity: f32,
        text: String,
    },
    /// Pede uma mensagem agora.
    Falar(FalarArgs),
    /// Mensagens agendadas.
    #[command(subcommand)]
    Agenda(AgendaCmd),
    /// Lista os temas de mensagens disponíveis.
    Temas,
    /// Lista os tons de voz disponíveis.
    Tons,
}

#[derive(Debug, Subcommand)]
pub enum PessoaCmd {
    Adicionar {
        #[arg(long = "nome")]
        name: String,
        #[arg(long = "relacao")]
        relationship: Option<String>,
        /// Nome de quem autorizou o uso da voz (obrigatório).
        #[arg(long = "consentimento-de")]
        consent_by: Option<String>,
    },
    Listar,
}

#[derive(Debug, Subcommand)]
pub enum AmostraCmd {
    /// Adiciona um áudio .wav. Se existirem `<arquivo>.txt` (transcrição) ou
    /// `<arquivo>.ipa` (fonemas ouvidos) ao lado, eles são usados.
    Adicionar {
        #[arg(long = "pessoa")]
        speaker: String,
        #[arg(long = "audio")]
        audio: PathBuf,
        /// O que é dito no áudio.
        #[arg(long = "texto")]
        transcript: Option<String>,
        /// Transcrição fonética (IPA) do que foi ouvido, ex.: "lejtʃi kẽtʃi".
        #[arg(long = "fonemas")]
        observed_ipa: Option<String>,
    },
    Listar {
        #[arg(long = "pessoa")]
        speaker: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum VozCmd {
    /// Analisa as amostras e gera a voz.
    Gerar {
        #[arg(long = "pessoa")]
        speaker: String,
    },
    /// Mostra o perfil de voz/sotaque gerado.
    Mostrar {
        #[arg(long = "pessoa")]
        speaker: String,
    },
}

#[derive(Debug, Args)]
pub struct ContentArgs {
    /// Tema (veja `temas`). Padrão: motivacao.
    #[arg(long = "tema", conflicts_with = "text")]
    pub category: Option<String>,
    /// Texto fixo.
    #[arg(long = "texto")]
    pub text: Option<String>,
}

/// Tom de voz da mensagem. Sem `--tom`, usa o tom inicial sugerido pelo tema
/// (ex.: calma → calmo, celebracao → animado; texto livre → natural).
#[derive(Debug, Args)]
pub struct ToneArgs {
    /// natural, carinhoso, animado, calmo, firme ou sussurrado (veja `tons`).
    #[arg(long = "tom")]
    pub style: Option<String>,
    /// Deixa a voz mais aguda (+) ou mais grave (-), em semitons (-6 a +6).
    #[arg(long = "ajuste-tom", allow_hyphen_values = true)]
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
    /// 0.0 (sem sotaque) a 1.0 (sotaque completo).
    #[arg(long = "intensidade", default_value_t = 1.0)]
    pub intensity: f32,
}

#[derive(Debug, Subcommand)]
pub enum AgendaCmd {
    Criar(Box<CriarAgendaArgs>),
    Listar,
    Remover {
        id: String,
    },
    /// Dispara os agendamentos vencidos.
    Executar {
        /// Fica rodando e verificando periodicamente.
        #[arg(long = "continuo")]
        watch: bool,
        /// Intervalo de verificação em segundos (modo contínuo).
        #[arg(long = "intervalo", default_value_t = 30)]
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
    /// Todo dia em um horário: 08:00
    #[arg(long = "diario")]
    pub daily: Option<String>,
    /// Dias da semana e horário: seg,qua,sex@07:30
    #[arg(long = "semanal")]
    pub weekly: Option<String>,
    /// A cada N minutos.
    #[arg(long = "a-cada")]
    pub every: Option<u32>,
    /// Janela para `--a-cada`: 08:00-20:00
    #[arg(long = "entre", requires = "every")]
    pub within: Option<String>,
    /// Uma única vez: "2026-11-20 09:00"
    #[arg(long = "em")]
    pub once: Option<String>,
    /// Não perturbe: 22:00-07:00
    #[arg(long = "silencio")]
    pub quiet: Option<String>,
    #[command(flatten)]
    pub content: ContentArgs,
    #[command(flatten)]
    pub tone: ToneArgs,
}
