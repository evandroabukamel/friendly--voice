use crate::{ids::SpeakerId, language::Language, DomainError};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// Consent record. Cloning someone's voice without authorization is not
/// allowed by the domain — it is an invariant, not a UI detail.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Consent {
    pub granted_by: String,
    pub granted_at: NaiveDateTime,
    pub statement: String,
}

/// Person chosen to "speak" the messages (e.g. mom, grandma, a friend).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Speaker {
    pub id: SpeakerId,
    pub name: String,
    pub relationship: Option<String>,
    pub consent: Consent,
    /// Language spoken in the recordings and in the messages.
    #[serde(default)]
    pub language: Language,
    /// How this person calls the listener ("filho", "Evinho"...). When set,
    /// it wins over the name given by the app or the CLI.
    #[serde(default)]
    pub listener_name: Option<String>,
}

impl Speaker {
    pub fn new(
        id: SpeakerId,
        name: &str,
        relationship: Option<String>,
        consent: Option<Consent>,
    ) -> Result<Self, DomainError> {
        let name = name.trim();
        if name.is_empty() {
            return Err(DomainError::invalid("name", "the name cannot be empty"));
        }
        let consent = consent.ok_or_else(|| DomainError::ConsentRequired(name.to_string()))?;
        if consent.granted_by.trim().is_empty() {
            return Err(DomainError::ConsentRequired(name.to_string()));
        }
        Ok(Self {
            id,
            name: name.to_string(),
            relationship: relationship.filter(|r| !r.trim().is_empty()),
            consent,
            language: Language::default(),
            listener_name: None,
        })
    }

    /// Sets how this person calls the listener; blank clears it.
    pub fn with_listener_name(mut self, listener_name: Option<String>) -> Self {
        self.listener_name = listener_name
            .map(|n| n.trim().to_string())
            .filter(|n| !n.is_empty());
        self
    }

    pub fn with_language(mut self, language: Language) -> Self {
        self.language = language;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn consent(by: &str) -> Consent {
        Consent {
            granted_by: by.into(),
            granted_at: NaiveDateTime::default(),
            statement: "autorizo".into(),
        }
    }

    #[test]
    fn requires_consent() {
        let err = Speaker::new(SpeakerId::new("s1"), "Mãe", None, None).unwrap_err();
        assert_eq!(err, DomainError::ConsentRequired("Mãe".into()));
        assert!(Speaker::new(SpeakerId::new("s1"), "Mãe", None, Some(consent(" "))).is_err());
    }

    #[test]
    fn requires_name() {
        assert!(Speaker::new(SpeakerId::new("s1"), "  ", None, Some(consent("Ana"))).is_err());
    }

    #[test]
    fn creates_speaker() {
        let s = Speaker::new(
            SpeakerId::new("s1"),
            " Vó ",
            Some("avó".into()),
            Some(consent("Vó")),
        )
        .unwrap();
        assert_eq!(s.name, "Vó");
    }

    #[test]
    fn listener_name_is_trimmed_and_blank_means_none() {
        let s = Speaker::new(SpeakerId::new("s1"), "Mãe", None, Some(consent("Mãe"))).unwrap();
        assert_eq!(s.listener_name, None);
        let s = s.with_listener_name(Some("  Filhote ".into()));
        assert_eq!(s.listener_name.as_deref(), Some("Filhote"));
        assert_eq!(s.with_listener_name(Some("   ".into())).listener_name, None);
    }

    #[test]
    fn reads_speakers_saved_before_listener_names() {
        let old = r#"{"id":"spk-1","name":"Mãe","relationship":null,
            "consent":{"granted_by":"Mãe","granted_at":"2026-11-02T10:00:00","statement":"ok"},
            "language":"pt-BR"}"#;
        let s: Speaker = serde_json::from_str(old).unwrap();
        assert_eq!(s.listener_name, None);
        assert_eq!(s.language.tag(), "pt-BR");
    }
}
