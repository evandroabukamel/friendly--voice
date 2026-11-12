use crate::error::AppResult;
use crate::ports::{Clock, IdGenerator, SpeakerRepository};
use friendly_voice_domain::{
    ids::SpeakerId,
    speaker::{Consent, Speaker},
};
use std::sync::Arc;

#[derive(Debug, Clone, Default)]
pub struct RegisterSpeakerInput {
    pub name: String,
    pub relationship: Option<String>,
    /// Who authorized the use of the voice (the person or their legal guardian).
    pub consent_granted_by: Option<String>,
}

/// US-01: register the person whose voice will be used.
pub struct RegisterSpeaker {
    speakers: Arc<dyn SpeakerRepository>,
    ids: Arc<dyn IdGenerator>,
    clock: Arc<dyn Clock>,
}

impl RegisterSpeaker {
    pub fn new(
        speakers: Arc<dyn SpeakerRepository>,
        ids: Arc<dyn IdGenerator>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            speakers,
            ids,
            clock,
        }
    }

    pub fn execute(&self, input: RegisterSpeakerInput) -> AppResult<Speaker> {
        let consent = input.consent_granted_by.map(|by| Consent {
            statement: format!(
                "{by} autorizou o uso da voz de {} para mensagens pessoais e privadas.",
                input.name.trim()
            ),
            granted_by: by,
            granted_at: self.clock.now(),
        });
        let speaker = Speaker::new(
            SpeakerId::new(self.ids.next_id("spk")),
            &input.name,
            input.relationship,
            consent,
        )?;
        self.speakers.save(&speaker)?;
        Ok(speaker)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fakes::Fakes;
    use crate::AppError;
    use friendly_voice_domain::DomainError;

    #[test]
    fn registers_with_consent_and_rejects_without() {
        let f = Fakes::new();
        let uc = RegisterSpeaker::new(f.speakers.clone(), f.ids.clone(), f.clock.clone());
        let err = uc
            .execute(RegisterSpeakerInput {
                name: "Mãe".into(),
                ..Default::default()
            })
            .unwrap_err();
        assert!(matches!(
            err,
            AppError::Domain(DomainError::ConsentRequired(_))
        ));

        let s = uc
            .execute(RegisterSpeakerInput {
                name: "Mãe".into(),
                relationship: Some("mãe".into()),
                consent_granted_by: Some("Maria".into()),
            })
            .unwrap();
        assert_eq!(f.speakers.list().unwrap(), vec![s]);
    }
}
