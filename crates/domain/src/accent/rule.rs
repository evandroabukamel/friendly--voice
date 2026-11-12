use crate::phonetics::{PhoneMatcher, Phoneme};
use serde::{Deserialize, Serialize};

/// Which aspect of the accent the rule describes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RulePhenomenon {
    /// How certain letters/sounds are pronounced.
    Segmental,
    /// Word endings.
    WordEnding,
    /// Word junctions (external sandhi).
    Juncture,
}

/// Position of the phoneme within the word.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Position {
    Anywhere,
    WordInitial,
    WordFinal,
    /// Consonant at the end of a syllable (before a consonant or the end of the word).
    Coda,
    /// Consonant at the start of a syllable (before a vowel/glide).
    Onset,
    /// Last vowel of the word (followed only by consonants/glides).
    FinalVowel,
    /// Vowel before the stressed syllable.
    Pretonic,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhoneContext {
    pub position: Position,
    pub preceding: Option<PhoneMatcher>,
    pub following: Option<PhoneMatcher>,
    pub unstressed_only: bool,
    /// Blocks the rule at the end of a word when the next word starts with a vowel
    /// (e.g. "falar" → "falá", but "falar assim" → "falaɾ assim").
    #[serde(default)]
    pub blocked_before_vowel_word: bool,
}

impl PhoneContext {
    pub fn at(position: Position) -> Self {
        Self {
            position,
            preceding: None,
            following: None,
            unstressed_only: false,
            blocked_before_vowel_word: false,
        }
    }
    pub fn blocked_before_vowel_word(mut self) -> Self {
        self.blocked_before_vowel_word = true;
        self
    }
    pub fn unstressed(mut self) -> Self {
        self.unstressed_only = true;
        self
    }
    pub fn followed_by(mut self, m: PhoneMatcher) -> Self {
        self.following = Some(m);
        self
    }
    pub fn preceded_by(mut self, m: PhoneMatcher) -> Self {
        self.preceding = Some(m);
        self
    }
}

/// Word-internal rule: `target → replacement / context`.
/// An empty `replacement` means deletion.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PronunciationRule {
    pub code: String,
    pub description: String,
    pub phenomenon: RulePhenomenon,
    pub target: PhoneMatcher,
    pub replacement: Vec<Phoneme>,
    pub context: PhoneContext,
    /// How often the person applies the rule (0..=1).
    pub confidence: f32,
}

impl PronunciationRule {
    pub fn new(
        code: &str,
        description: &str,
        phenomenon: RulePhenomenon,
        target: PhoneMatcher,
        replacement: &[&str],
        context: PhoneContext,
    ) -> Self {
        Self {
            code: code.into(),
            description: description.into(),
            phenomenon,
            target,
            replacement: replacement.iter().map(|s| Phoneme::new(*s)).collect(),
            context,
            confidence: 1.0,
        }
    }
    pub fn with_confidence(mut self, confidence: f32) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }
}

/// What happens to the end of the left word at the junction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum JunctureEffect {
    ReplaceLeft(Vec<Phoneme>),
    DeleteLeft,
}

/// Rule between the last sound of a word and the first sound of the next one.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JunctureRule {
    pub code: String,
    pub description: String,
    pub left: PhoneMatcher,
    pub right: PhoneMatcher,
    pub left_unstressed_only: bool,
    pub effect: JunctureEffect,
    pub confidence: f32,
}

impl JunctureRule {
    pub fn new(
        code: &str,
        description: &str,
        left: PhoneMatcher,
        right: PhoneMatcher,
        effect: JunctureEffect,
    ) -> Self {
        Self {
            code: code.into(),
            description: description.into(),
            left,
            right,
            left_unstressed_only: false,
            effect,
            confidence: 1.0,
        }
    }
    pub fn left_unstressed(mut self) -> Self {
        self.left_unstressed_only = true;
        self
    }
    pub fn with_confidence(mut self, confidence: f32) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }
    /// Phonemes expected in place of the end of the left word.
    pub fn expected_left(&self) -> Vec<Phoneme> {
        match &self.effect {
            JunctureEffect::ReplaceLeft(p) => p.clone(),
            JunctureEffect::DeleteLeft => Vec::new(),
        }
    }
}

/// A person's accent profile.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AccentProfile {
    /// Friendly label (e.g. "Carioca (RJ)" or "Custom").
    pub label: String,
    /// Word-internal rules, in application order.
    pub segment_rules: Vec<PronunciationRule>,
    /// Juncture rules, in application order.
    pub juncture_rules: Vec<JunctureRule>,
}

impl AccentProfile {
    /// Profile without any rule: citation pronunciation.
    pub fn neutral() -> Self {
        Self {
            label: "Neutral (no samples with phonemes)".into(),
            ..Self::default()
        }
    }
    pub fn rule_codes(&self) -> Vec<&str> {
        self.segment_rules
            .iter()
            .map(|r| r.code.as_str())
            .chain(self.juncture_rules.iter().map(|r| r.code.as_str()))
            .collect()
    }
}
