use crate::error::{AppError, AppResult};
use crate::ports::SpeakerRepository;
use friendly_voice_domain::{ids::SpeakerId, speaker::Speaker};
use std::sync::Arc;

/// Per-voice listener name: how this person calls the listener in the
/// messages. `None` or blank clears it (the app's/CLI's name is used again).
pub struct SetListenerName {
    speakers: Arc<dyn SpeakerRepository>,
}

impl SetListenerName {
    pub fn new(speakers: Arc<dyn SpeakerRepository>) -> Self {
        Self { speakers }
    }

    pub fn execute(&self, id: &SpeakerId, listener_name: Option<String>) -> AppResult<Speaker> {
        let speaker = self
            .speakers
            .find(id)?
            .ok_or_else(|| AppError::not_found("person", id))?;
        let updated = speaker.clone().with_listener_name(listener_name);
        if updated != speaker {
            self.speakers.save(&updated)?;
        }
        Ok(updated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fakes::Fakes;

    #[test]
    fn sets_and_clears_idempotently() {
        let f = Fakes::new();
        let spk = f.speaker();
        let uc = SetListenerName::new(f.speakers.clone());
        let named = uc.execute(&spk.id, Some(" Filhote ".into())).unwrap();
        assert_eq!(named.listener_name.as_deref(), Some("Filhote"));
        assert_eq!(uc.execute(&spk.id, Some("Filhote".into())).unwrap(), named);
        assert_eq!(f.speakers.find(&spk.id).unwrap(), Some(named));
        let cleared = uc.execute(&spk.id, Some("  ".into())).unwrap();
        assert_eq!(cleared.listener_name, None);
        assert_eq!(cleared, spk);
    }

    #[test]
    fn unknown_person() {
        let f = Fakes::new();
        let err = SetListenerName::new(f.speakers.clone())
            .execute(&SpeakerId::new("ghost"), None)
            .unwrap_err();
        assert!(matches!(err, AppError::NotFound { .. }));
    }
}
