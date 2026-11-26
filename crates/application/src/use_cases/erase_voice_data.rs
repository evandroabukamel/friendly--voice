use crate::error::{AppError, AppResult};
use crate::ports::{
    AudioStore, Clock, SampleRepository, ScheduleRepository, SpeechCache, VoiceCloningEngine,
    VoiceProfileRepository,
};
use friendly_voice_domain::ids::SpeakerId;
use std::sync::Arc;

/// Deletes everything made from a person's voice and switches off their
/// schedules (shared by [`super::RevokeConsent`] and [`super::DeleteSpeaker`]).
/// Every step is idempotent, so a re-run after a crash finishes the job.
pub struct VoiceDataEraser {
    pub schedules: Arc<dyn ScheduleRepository>,
    pub samples: Arc<dyn SampleRepository>,
    pub store: Arc<dyn AudioStore>,
    pub profiles: Arc<dyn VoiceProfileRepository>,
    pub engine: Arc<dyn VoiceCloningEngine>,
    pub cache: Arc<dyn SpeechCache>,
    pub clock: Arc<dyn Clock>,
}

impl VoiceDataEraser {
    pub fn erase(&self, speaker_id: &SpeakerId, name: &str) -> AppResult<()> {
        let now = self.clock.now();
        for mut schedule in self.schedules.list()? {
            if &schedule.speaker_id == speaker_id && schedule.enabled {
                schedule.set_enabled(false, now)?;
                self.schedules.save(&schedule)?;
            }
        }
        self.cache.clear(speaker_id)?;
        // Files before entries: an entry left behind by a crash is harmless,
        // a file without an entry would never be found again.
        self.store.delete_all(speaker_id)?;
        self.samples.delete_by_speaker(speaker_id)?;
        // The profile goes last: it holds the engine's voice id, needed to
        // retry when the voice server is unreachable.
        if let Some(profile) = self.profiles.find(speaker_id)? {
            self.engine.forget(&profile.engine_voice_id).map_err(|e| {
                AppError::Precondition(format!(
                    "the recordings of {name} were deleted, but the voice server could not delete the voice ({e}); run it again when the server is reachable"
                ))
            })?;
            self.profiles.delete(speaker_id)?;
        }
        Ok(())
    }
}
