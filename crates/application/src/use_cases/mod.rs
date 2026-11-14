//! One use case per file (SRP). Each one receives its dependencies through
//! the constructor as `Arc<dyn Port>` (DIP) and exposes a single `execute`.
mod add_voice_sample;
mod build_voice_profile;
mod register_speaker;
mod schedules;
mod set_listener_name;
mod set_schedule_enabled;
mod speak_message;
mod update_schedule;

pub use add_voice_sample::{AddVoiceSample, AddVoiceSampleInput};
pub use build_voice_profile::BuildVoiceProfile;
pub use register_speaker::{RegisterSpeaker, RegisterSpeakerInput};
pub use schedules::{
    CreateSchedule, CreateScheduleInput, ListSchedules, RemoveSchedule, RunDueSchedules,
    ScheduleRunOutcome,
};
pub use set_listener_name::SetListenerName;
pub use set_schedule_enabled::SetScheduleEnabled;
pub use speak_message::{SpeakMessage, SpeakRequest, SpokenMessage};
pub use update_schedule::UpdateSchedule;
