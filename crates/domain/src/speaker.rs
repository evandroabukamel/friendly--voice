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
        })
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
}
