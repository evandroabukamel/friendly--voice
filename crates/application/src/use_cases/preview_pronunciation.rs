use crate::error::{AppError, AppResult};
use crate::ports::{Phonemizer, VoiceProfileRepository};
use friendly_voice_domain::{accent::AccentRenderer, ids::SpeakerId};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct PronunciationPreview {
    pub accent_label: String,
    pub citation_ipa: String,
    pub accented_ipa: String,
}

/// US-06: see how a text will be pronounced with the person's accent.
pub struct PreviewPronunciation {
    profiles: Arc<dyn VoiceProfileRepository>,
    phonemizer: Arc<dyn Phonemizer>,
}

impl PreviewPronunciation {
    pub fn new(profiles: Arc<dyn VoiceProfileRepository>, phonemizer: Arc<dyn Phonemizer>) -> Self {
        Self {
            profiles,
            phonemizer,
        }
    }

    pub fn execute(
        &self,
        speaker: &SpeakerId,
        text: &str,
        intensity: f32,
    ) -> AppResult<PronunciationPreview> {
        let profile = self
            .profiles
            .find(speaker)?
            .ok_or_else(|| AppError::not_found("voice profile", speaker))?;
        let citation = self.phonemizer.phonemize(text);
        let accented = AccentRenderer.render(&profile.accent, &citation, intensity);
        Ok(PronunciationPreview {
            accent_label: profile.accent.label,
            citation_ipa: citation.ipa(),
            accented_ipa: accented.ipa(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fakes::Fakes;

    #[test]
    fn previews() {
        let f = Fakes::new();
        let spk = f.speaker();
        f.carioca_profile(&spk);
        let p = PreviewPronunciation::new(f.profiles.clone(), f.phonemizer.clone())
            .execute(&spk.id, "noite", 1.0)
            .unwrap();
        assert_eq!(p.citation_ipa, "nojte");
        assert_eq!(p.accented_ipa, "nojtʃi");
    }
}
