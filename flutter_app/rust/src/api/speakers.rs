//! People whose voice is used (registration requires recorded consent).
use super::failure::AppFailure;
use super::session::{non_empty, Session};
use flutter_rust_bridge::frb;
use friendly_voice_application::use_cases::{RegisterSpeakerInput, SpeakerOverview};
use friendly_voice_domain::{ids::SpeakerId, speaker::Speaker, voice::VoiceGender};

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
    /// Who authorized the use of the voice.
    pub consent_by: String,
    /// How this person calls the listener; wins over the app's name.
    pub listener_name: Option<String>,
    pub sample_count: u32,
    pub total_sample_secs: f32,
}

impl From<SpeakerOverview> for SpeakerSummary {
    fn from(o: SpeakerOverview) -> Self {
        let s = o.speaker;
        Self {
            id: s.id.to_string(),
            name: s.name,
            relationship: s.relationship,
            language: s.language.to_string(),
            voice_ready: o.voice.is_some(),
            gender: o.voice.and_then(|v| v.gender).map(Gender::from),
            consent_by: s.consent.granted_by,
            listener_name: s.listener_name,
            sample_count: u32::try_from(o.sample_count).unwrap_or(u32::MAX),
            total_sample_secs: o.total_sample_secs,
        }
    }
}

impl Session {
    fn summary(&self, s: Speaker) -> Result<SpeakerSummary, AppFailure> {
        let overview = self
            .container
            .list_speakers()
            .one(s)
            .map_err(AppFailure::new)?;
        Ok(overview.into())
    }

    /// Registers a person; `consent_by` (who authorized the use of the
    /// voice) is mandatory. Default language: pt-BR. `listener_name` is how
    /// this person calls the listener (optional).
    #[frb(sync)]
    pub fn register_speaker(
        &self,
        name: String,
        relationship: Option<String>,
        consent_by: Option<String>,
        language: Option<String>,
        listener_name: Option<String>,
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
                listener_name: non_empty(listener_name),
            })
            .map_err(AppFailure::new)?;
        self.summary(speaker)
    }

    /// People whose consent stands, with their sample totals and voice.
    #[frb(sync)]
    pub fn list_speakers(&self) -> Result<Vec<SpeakerSummary>, AppFailure> {
        let all = self
            .container
            .list_speakers()
            .execute()
            .map_err(AppFailure::new)?;
        Ok(all.into_iter().map(SpeakerSummary::from).collect())
    }

    /// Sets how this person calls the listener; blank clears it.
    #[frb(sync)]
    pub fn set_listener_name(
        &self,
        speaker_id: String,
        listener_name: Option<String>,
    ) -> Result<SpeakerSummary, AppFailure> {
        let speaker = self
            .container
            .set_listener_name()
            .execute(&SpeakerId::new(speaker_id), non_empty(listener_name))
            .map_err(AppFailure::new)?;
        self.summary(speaker)
    }

    /// Deletes a person and everything made from their voice (also on the
    /// voice server); their schedules are switched off and kept.
    /// Idempotent: an unknown id is fine.
    pub fn delete_speaker(&self, speaker_id: String) -> Result<(), AppFailure> {
        self.container
            .delete_speaker()
            .execute(&SpeakerId::new(speaker_id))
            .map_err(AppFailure::new)
    }
}

#[cfg(test)]
mod tests {
    use crate::api::session::tests::local_session;
    use crate::api::voice::tests::wav;

    #[test]
    fn registration_requires_consent() {
        let s = local_session("no-consent");
        let err = s
            .register_speaker("Vó".into(), None, None, None, None)
            .unwrap_err();
        assert!(err.message.contains("consent"), "{err}");
        assert!(s.list_speakers().unwrap().is_empty());
    }

    #[test]
    fn registered_people_are_listed_with_language_and_voice_state() {
        let s = local_session("register");
        let vo = s
            .register_speaker(
                "Vó".into(),
                Some("grandma".into()),
                Some("Vó".into()),
                None,
                None,
            )
            .unwrap();
        assert_eq!(vo.name, "Vó");
        assert_eq!(vo.language, "pt-BR");
        s.register_speaker(
            "Ana".into(),
            Some(" ".into()),
            Some("Ana".into()),
            Some("es".into()),
            None,
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
    fn a_person_carries_consent_listener_name_and_sample_totals() {
        let s = local_session("details");
        let vo = s
            .register_speaker(
                "Vó".into(),
                None,
                Some("Ana".into()),
                None,
                Some(" Filhote ".into()),
            )
            .unwrap();
        assert_eq!(vo.consent_by, "Ana");
        assert_eq!(vo.listener_name.as_deref(), Some("Filhote"));
        assert_eq!(vo.sample_count, 0);
        s.add_sample(vo.id.clone(), wav(6.0, 210.0), None).unwrap();
        let listed = &s.list_speakers().unwrap()[0];
        assert_eq!(listed.sample_count, 1);
        assert!((listed.total_sample_secs - 6.0).abs() < 0.01);
    }

    #[test]
    fn the_listener_name_is_changed_and_cleared_when_blank() {
        let s = local_session("listener");
        let vo = s
            .register_speaker("Vó".into(), None, Some("Vó".into()), None, None)
            .unwrap();
        let named = s
            .set_listener_name(vo.id.clone(), Some("Campeão".into()))
            .unwrap();
        assert_eq!(named.listener_name.as_deref(), Some("Campeão"));
        let cleared = s.set_listener_name(vo.id, Some("  ".into())).unwrap();
        assert_eq!(cleared.listener_name, None);
    }

    #[test]
    fn a_deleted_person_is_no_longer_listed_and_deleting_twice_is_fine() {
        let s = local_session("delete");
        let vo = s
            .register_speaker("Vó".into(), None, Some("Vó".into()), None, None)
            .unwrap();
        s.add_sample(vo.id.clone(), wav(6.0, 210.0), None).unwrap();
        s.delete_speaker(vo.id.clone()).unwrap();
        assert!(s.list_speakers().unwrap().is_empty());
        s.delete_speaker(vo.id).unwrap();
    }

    #[test]
    fn unknown_language_is_refused() {
        let s = local_session("bad-language");
        let err = s
            .register_speaker(
                "Vó".into(),
                None,
                Some("Vó".into()),
                Some("xx".into()),
                None,
            )
            .unwrap_err();
        assert!(!err.message.is_empty());
    }
}
