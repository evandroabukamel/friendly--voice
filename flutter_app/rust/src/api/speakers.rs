//! People whose voice is used (registration requires recorded consent).
use super::failure::AppFailure;
use super::session::{non_empty, Session};
use flutter_rust_bridge::frb;
use friendly_voice_application::use_cases::RegisterSpeakerInput;
use friendly_voice_domain::{speaker::Speaker, voice::VoiceGender};

/// Voice register detected from the recordings.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Gender {
    Female,
    Male,
}

impl From<VoiceGender> for Gender {
    fn from(g: VoiceGender) -> Self {
        match g {
            VoiceGender::Female => Self::Female,
            VoiceGender::Male => Self::Male,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpeakerSummary {
    pub id: String,
    pub name: String,
    pub relationship: Option<String>,
    /// Language tag of the recordings and the messages (e.g. `pt-BR`).
    pub language: String,
    pub voice_ready: bool,
    pub gender: Option<Gender>,
}

impl Session {
    fn summary(&self, s: Speaker) -> Result<SpeakerSummary, AppFailure> {
        let voice = self
            .container
            .profiles
            .find(&s.id)
            .map_err(AppFailure::new)?;
        Ok(SpeakerSummary {
            id: s.id.to_string(),
            name: s.name,
            relationship: s.relationship,
            language: s.language.to_string(),
            voice_ready: voice.is_some(),
            gender: voice.and_then(|v| v.gender).map(Gender::from),
        })
    }

    /// Registers a person; `consent_by` (who authorized the use of the
    /// voice) is mandatory. Default language: pt-BR.
    #[frb(sync)]
    pub fn register_speaker(
        &self,
        name: String,
        relationship: Option<String>,
        consent_by: Option<String>,
        language: Option<String>,
    ) -> Result<SpeakerSummary, AppFailure> {
        let language = non_empty(language)
            .map(|l| l.parse().map_err(AppFailure::new))
            .transpose()?;
        let speaker = self
            .container
            .register_speaker()
            .execute(RegisterSpeakerInput {
                name,
                relationship: non_empty(relationship),
                consent_granted_by: non_empty(consent_by),
                language,
                listener_name: None,
            })
            .map_err(AppFailure::new)?;
        self.summary(speaker)
    }

    #[frb(sync)]
    pub fn list_speakers(&self) -> Result<Vec<SpeakerSummary>, AppFailure> {
        let speakers = self.container.speakers.list().map_err(AppFailure::new)?;
        speakers.into_iter().map(|s| self.summary(s)).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::session::tests::local_session;

    #[test]
    fn registration_requires_consent() {
        let s = local_session("no-consent");
        let err = s
            .register_speaker("Vó".into(), None, None, None)
            .unwrap_err();
        assert!(err.message.contains("consent"), "{err}");
        assert!(s.list_speakers().unwrap().is_empty());
    }

    #[test]
    fn registered_people_are_listed_with_language_and_voice_state() {
        let s = local_session("register");
        let vo = s
            .register_speaker("Vó".into(), Some("grandma".into()), Some("Vó".into()), None)
            .unwrap();
        assert_eq!(vo.name, "Vó");
        assert_eq!(vo.language, "pt-BR");
        s.register_speaker(
            "Ana".into(),
            Some(" ".into()),
            Some("Ana".into()),
            Some("es".into()),
        )
        .unwrap();
        let all = s.list_speakers().unwrap();
        assert_eq!(all.len(), 2);
        let ana = all.iter().find(|p| p.name == "Ana").unwrap();
        assert_eq!(ana.language, "es");
        assert_eq!(ana.relationship, None);
        assert!(!ana.voice_ready);
        assert_eq!(ana.gender, None);
    }

    #[test]
    fn unknown_language_is_refused() {
        let s = local_session("bad-language");
        let err = s
            .register_speaker("Vó".into(), None, Some("Vó".into()), Some("xx".into()))
            .unwrap_err();
        assert!(!err.message.is_empty());
    }
}
