//! Um caso de uso por arquivo (SRP). Cada um recebe suas dependências por
//! construtor como `Arc<dyn Porta>` (DIP) e expõe um único `execute`.
mod add_voice_sample;
mod build_voice_profile;
mod preview_pronunciation;
mod register_speaker;
mod schedules;
mod speak_message;

pub use add_voice_sample::{AddVoiceSample, AddVoiceSampleInput};
pub use build_voice_profile::{BuildVoiceProfile, BuildVoiceProfileOutput};
pub use preview_pronunciation::{PreviewPronunciation, PronunciationPreview};
pub use register_speaker::{RegisterSpeaker, RegisterSpeakerInput};
pub use schedules::{
    CreateSchedule, CreateScheduleInput, ListSchedules, RemoveSchedule, RunDueSchedules,
    ScheduleRunOutcome,
};
pub use speak_message::{SpeakMessage, SpeakRequest, SpokenMessage};
