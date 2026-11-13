//! Ports (interfaces) the application requires from the outside world.
//!
//! Small, cohesive interfaces (ISP): each adapter implements only what it
//! needs, and each use case depends only on what it uses.
use chrono::NaiveDateTime;
use friendly_voice_domain::{
    audio::{AcousticFeatures, AudioClip},
    ids::{SampleId, ScheduleId, SpeakerId},
    language::Language,
    message::{MessageCategory, MotivationalMessage},
    schedule::Schedule,
    speaker::Speaker,
    voice::{SpeechScript, VoiceGender, VoiceProfile, VoiceSample},
};
use std::fmt;

/// Generic adapter error.
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

// ---------- Persistence ----------

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

/// Stores the original audio files.
pub trait AudioStore: Send + Sync {
    fn store(&self, speaker: &SpeakerId, sample: &SampleId, bytes: &[u8]) -> PortResult<String>;
    fn load(&self, locator: &str) -> PortResult<Vec<u8>>;
}

// ---------- Audio ----------

pub trait AudioDecoder: Send + Sync {
    fn decode(&self, bytes: &[u8]) -> PortResult<AudioClip>;
}

pub trait AcousticAnalyzer: Send + Sync {
    fn analyze(&self, clip: &AudioClip) -> AcousticFeatures;
}

/// Speech recognition (optional): produces the transcript from the audio.
pub trait Transcriber: Send + Sync {
    fn transcribe(&self, clip: &AudioClip, language: &Language) -> PortResult<String>;
}

// ---------- Voice ----------

/// Reference audio sent to the cloning engine.
#[derive(Debug, Clone, PartialEq)]
pub struct ReferenceAudio {
    pub bytes: Vec<u8>,
    pub transcript: Option<String>,
}

/// Clones the voice in the neural engine from the person's recordings, in
/// the person's language. `gender` is the register detected from the
/// recordings, a hint for engines that pick a base voice.
pub trait VoiceCloningEngine: Send + Sync {
    fn enroll(
        &self,
        speaker: &Speaker,
        gender: Option<VoiceGender>,
        references: &[ReferenceAudio],
    ) -> PortResult<String>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct SynthesizedAudio {
    pub wav: Vec<u8>,
}

pub trait SpeechSynthesizer: Send + Sync {
    fn synthesize(
        &self,
        voice: &VoiceProfile,
        script: &SpeechScript,
    ) -> PortResult<SynthesizedAudio>;
}

// ---------- Messages ----------

pub trait MessageSource: Send + Sync {
    /// `listener` is the name of the person listening, for personalization.
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

/// Delivers the message (play, save, notify...). Returns a receipt.
pub trait MessageDelivery: Send + Sync {
    fn deliver(&self, delivery: &Delivery<'_>) -> PortResult<String>;
}

// ---------- System ----------

pub trait Clock: Send + Sync {
    fn now(&self) -> NaiveDateTime;
}

pub trait IdGenerator: Send + Sync {
    fn next_id(&self, prefix: &str) -> String;
}
