use crate::DomainError;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Tema da mensagem positiva.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MessageCategory {
    Motivation,
    Confidence,
    Gratitude,
    Calm,
    Celebration,
    GoodMorning,
}

impl MessageCategory {
    pub const ALL: [MessageCategory; 6] = [
        Self::Motivation,
        Self::Confidence,
        Self::Gratitude,
        Self::Calm,
        Self::Celebration,
        Self::GoodMorning,
    ];
    pub fn slug(self) -> &'static str {
        match self {
            Self::Motivation => "motivacao",
            Self::Confidence => "confianca",
            Self::Gratitude => "gratidao",
            Self::Calm => "calma",
            Self::Celebration => "celebracao",
            Self::GoodMorning => "bom-dia",
        }
    }
}

impl fmt::Display for MessageCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.slug())
    }
}

impl FromStr for MessageCategory {
    type Err = DomainError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_lowercase();
        Self::ALL
            .into_iter()
            .find(|c| c.slug() == s)
            .ok_or_else(|| {
                DomainError::invalid(
                    "category",
                    format!(
                        "'{s}' desconhecida; use: {}",
                        Self::ALL.map(|c| c.slug()).join(", ")
                    ),
                )
            })
    }
}

pub const MAX_MESSAGE_CHARS: usize = 600;

/// Mensagem a ser falada.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MotivationalMessage {
    text: String,
    category: Option<MessageCategory>,
}

impl MotivationalMessage {
    pub fn new(text: &str, category: Option<MessageCategory>) -> Result<Self, DomainError> {
        let text = text.trim();
        if text.is_empty() {
            return Err(DomainError::invalid("text", "mensagem vazia"));
        }
        if text.chars().count() > MAX_MESSAGE_CHARS {
            return Err(DomainError::invalid(
                "text",
                format!("máximo de {MAX_MESSAGE_CHARS} caracteres"),
            ));
        }
        Ok(Self {
            text: text.to_string(),
            category,
        })
    }
    pub fn text(&self) -> &str {
        &self.text
    }
    pub fn category(&self) -> Option<MessageCategory> {
        self.category
    }
}

/// De onde vem o conteúdo de uma mensagem.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MessageContent {
    /// Sorteia uma mensagem do tema.
    Category(MessageCategory),
    /// Texto definido pelo usuário.
    FixedText(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_categories() {
        assert_eq!(
            "bom-dia".parse::<MessageCategory>().unwrap(),
            MessageCategory::GoodMorning
        );
        assert!("xyz".parse::<MessageCategory>().is_err());
    }

    #[test]
    fn validates_text() {
        assert!(MotivationalMessage::new("  ", None).is_err());
        assert!(MotivationalMessage::new(&"a".repeat(601), None).is_err());
        assert_eq!(MotivationalMessage::new(" Oi ", None).unwrap().text(), "Oi");
    }
}
