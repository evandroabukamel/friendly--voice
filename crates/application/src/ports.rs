//! Portas (interfaces) que a aplicação exige do mundo externo.
//!
//! Interfaces pequenas e coesas (ISP): cada adaptador implementa só o que
//! precisa, e cada caso de uso depende só do que usa.
use chrono::NaiveDateTime;
use friendly_voice_domain::{
    audio::{AcousticFeatures, AudioClip},
    ids::{SampleId, ScheduleId, SpeakerId},
    message::{MessageCategory, MotivationalMessage},
    phonetics::{Phoneme, PhoneticUtterance},
    schedule::Schedule,
    speaker::Speaker,
    voice::{SpeechScript, VoiceProfile, VoiceSample},
};
use std::fmt;

/// Erro genérico de um adaptador.
#[derive(Debug, Clone, PartialEq)]
pub struct PortError(pub String);

impl fmt::Display for PortError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for PortError {}

impl PortError {
    pub fn new(msg: impl fmt::Display) -> Self {
        Self(msg.to_string())
    }
}

pub type PortResult<T> = Result<T, PortError>;

// ---------- Persistência ----------

pub trait SpeakerRepository: Send + Sync {
    fn save(&self, speaker: &Speaker) -> PortResult<()>;
    fn find(&self, id: &SpeakerId) -> PortResult<Option<Speaker>>;
    fn list(&self) -> PortResult<Vec<Speaker>>;
}

pub trait SampleRepository: Send + Sync {
    fn save(&self, sample: &VoiceSample) -> PortResult<()>;
    fn list_by_speaker(&self, speaker: &SpeakerId) -> PortResult<Vec<VoiceSample>>;
}

pub trait VoiceProfileRepository: Send + Sync {
    fn save(&self, profile: &VoiceProfile) -> PortResult<()>;
    fn find(&self, speaker: &SpeakerId) -> PortResult<Option<VoiceProfile>>;
}

pub trait ScheduleRepository: Send + Sync {
    fn save(&self, schedule: &Schedule) -> PortResult<()>;
    fn list(&self) -> PortResult<Vec<Schedule>>;
    fn delete(&self, id: &ScheduleId) -> PortResult<bool>;
}

/// Guarda os arquivos de áudio originais.
pub trait AudioStore: Send + Sync {
    fn store(&self, speaker: &SpeakerId, sample: &SampleId, bytes: &[u8]) -> PortResult<String>;
    fn load(&self, locator: &str) -> PortResult<Vec<u8>>;
}

// ---------- Áudio e fonética ----------

pub trait AudioDecoder: Send + Sync {
    fn decode(&self, bytes: &[u8]) -> PortResult<AudioClip>;
}

pub trait AcousticAnalyzer: Send + Sync {
    fn analyze(&self, clip: &AudioClip) -> AcousticFeatures;
}

/// Converte texto em pronúncia de citação (G2P).
pub trait Phonemizer: Send + Sync {
    fn phonemize(&self, text: &str) -> PhoneticUtterance;
}

/// Reconhecimento de fala (opcional): gera o transcript a partir do áudio.
pub trait Transcriber: Send + Sync {
    fn transcribe(&self, clip: &AudioClip) -> PortResult<String>;
}

/// Reconhecimento fonético (opcional): o que a pessoa *de fato* pronunciou.
pub trait PhoneRecognizer: Send + Sync {
    fn recognize(&self, clip: &AudioClip, transcript: &str) -> PortResult<Vec<Phoneme>>;
}

// ---------- Voz ----------

/// Áudio de referência enviado ao motor de clonagem.
#[derive(Debug, Clone, PartialEq)]
pub struct ReferenceAudio {
    pub bytes: Vec<u8>,
    pub transcript: String,
}

/// Cria a voz (timbre) no motor neural.
pub trait VoiceCloningEngine: Send + Sync {
    fn enroll(&self, speaker: &Speaker, references: &[ReferenceAudio]) -> PortResult<String>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct SynthesizedAudio {
    pub wav: Vec<u8>,
    /// Marcação enviada ao motor (ex.: SSML), útil para auditoria.
    pub markup: String,
}

pub trait SpeechSynthesizer: Send + Sync {
    fn synthesize(
        &self,
        voice: &VoiceProfile,
        script: &SpeechScript,
    ) -> PortResult<SynthesizedAudio>;
}

// ---------- Mensagens ----------

pub trait MessageSource: Send + Sync {
    /// `listener` é o nome de quem ouve, para personalização.
    fn pick(
        &self,
        category: MessageCategory,
        speaker: &Speaker,
        listener: Option<&str>,
    ) -> PortResult<MotivationalMessage>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Delivery<'a> {
    pub speaker: &'a Speaker,
    pub message: &'a MotivationalMessage,
    pub script: &'a SpeechScript,
    pub audio: &'a SynthesizedAudio,
    pub at: NaiveDateTime,
}

/// Entrega a mensagem (tocar, salvar, notificar...). Devolve um recibo.
pub trait MessageDelivery: Send + Sync {
    fn deliver(&self, delivery: &Delivery<'_>) -> PortResult<String>;
}

// ---------- Sistema ----------

pub trait Clock: Send + Sync {
    fn now(&self) -> NaiveDateTime;
}

pub trait IdGenerator: Send + Sync {
    fn next_id(&self, prefix: &str) -> String;
}
