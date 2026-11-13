//! Language the person speaks in the recordings, and in which the messages
//! are spoken.
use crate::DomainError;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Languages the voice engine can speak: primary subtag and English name.
pub const SUPPORTED: [(&str, &str); 23] = [
    ("ar", "Arabic"),
    ("da", "Danish"),
    ("de", "German"),
    ("el", "Greek"),
    ("en", "English"),
    ("es", "Spanish"),
    ("fi", "Finnish"),
    ("fr", "French"),
    ("he", "Hebrew"),
    ("hi", "Hindi"),
    ("it", "Italian"),
    ("ja", "Japanese"),
    ("ko", "Korean"),
    ("ms", "Malay"),
    ("nl", "Dutch"),
    ("no", "Norwegian"),
    ("pl", "Polish"),
    ("pt", "Portuguese"),
    ("ru", "Russian"),
    ("sv", "Swedish"),
    ("sw", "Swahili"),
    ("tr", "Turkish"),
    ("zh", "Chinese"),
];

/// BCP 47 language tag such as `pt-BR` or `en`, restricted to [`SUPPORTED`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Language(String);

impl Language {
    /// Primary subtag: `pt-BR` → `pt`.
    pub fn code(&self) -> &str {
        self.0.split('-').next().unwrap_or(&self.0)
    }

    pub fn tag(&self) -> &str {
        &self.0
    }

    pub fn name(&self) -> &'static str {
        SUPPORTED
            .iter()
            .find(|(code, _)| *code == self.code())
            .map(|(_, name)| *name)
            .unwrap_or("?")
    }

    pub fn is_portuguese(&self) -> bool {
        self.code() == "pt"
    }
}

impl Default for Language {
    fn default() -> Self {
        Self("pt-BR".into())
    }
}

impl FromStr for Language {
    type Err = DomainError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().replace('_', "-");
        let mut parts = s.split('-');
        let code = parts.next().unwrap_or_default().to_lowercase();
        if !SUPPORTED.iter().any(|(c, _)| *c == code) {
            return Err(DomainError::invalid(
                "language",
                format!(
                    "unsupported language '{s}'; use one of: {}",
                    SUPPORTED.map(|(c, _)| c).join(", ")
                ),
            ));
        }
        let region: Vec<String> = parts.map(str::to_uppercase).collect();
        Ok(Self(
            std::iter::once(code)
                .chain(region)
                .collect::<Vec<_>>()
                .join("-"),
        ))
    }
}

impl TryFrom<String> for Language {
    type Error = DomainError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl From<Language> for String {
    fn from(l: Language) -> Self {
        l.0
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_normalizes_tags() {
        let pt: Language = "pt_br".parse().unwrap();
        assert_eq!(pt.tag(), "pt-BR");
        assert_eq!(pt.code(), "pt");
        assert!(pt.is_portuguese());
        assert_eq!("EN".parse::<Language>().unwrap().name(), "English");
        assert!("tlh".parse::<Language>().is_err());
        assert!("".parse::<Language>().is_err());
    }

    #[test]
    fn defaults_to_brazilian_portuguese_and_round_trips() {
        let l = Language::default();
        assert_eq!(l.tag(), "pt-BR");
        let json = serde_json::to_string(&l).unwrap();
        assert_eq!(json, "\"pt-BR\"");
        assert_eq!(serde_json::from_str::<Language>(&json).unwrap(), l);
        assert!(serde_json::from_str::<Language>("\"xx\"").is_err());
    }
}
