use crate::phonetics::{PhoneMatcher, Phoneme};
use serde::{Deserialize, Serialize};

/// Qual aspecto do sotaque a regra descreve.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RulePhenomenon {
    /// Jeito de pronunciar certas letras/sons.
    Segmental,
    /// Término de palavras.
    WordEnding,
    /// Junção de palavras (sândi externo).
    Juncture,
}

/// Posição do fonema dentro da palavra.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Position {
    Anywhere,
    WordInitial,
    WordFinal,
    /// Consoante em final de sílaba (antes de consoante ou fim de palavra).
    Coda,
    /// Consoante em início de sílaba (antes de vogal/semivogal).
    Onset,
    /// Última vogal da palavra (seguida apenas de consoantes/semivogais).
    FinalVowel,
    /// Vogal antes da sílaba tônica.
    Pretonic,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhoneContext {
    pub position: Position,
    pub preceding: Option<PhoneMatcher>,
    pub following: Option<PhoneMatcher>,
    pub unstressed_only: bool,
    /// Bloqueia a regra no fim da palavra quando a próxima começa com vogal
    /// (ex.: "falar" → "falá", mas "falar assim" → "falaɾ assim").
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

/// Regra intra-palavra: `target → replacement / context`.
/// `replacement` vazio significa apagamento.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PronunciationRule {
    pub code: String,
    pub description: String,
    pub phenomenon: RulePhenomenon,
    pub target: PhoneMatcher,
    pub replacement: Vec<Phoneme>,
    pub context: PhoneContext,
    /// Frequência com que a pessoa realiza a regra (0..=1).
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

/// O que acontece com o fim da palavra da esquerda na junção.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum JunctureEffect {
    ReplaceLeft(Vec<Phoneme>),
    DeleteLeft,
}

/// Regra entre o último som de uma palavra e o primeiro da seguinte.
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
    /// Fonemas esperados no lugar do fim da palavra esquerda.
    pub fn expected_left(&self) -> Vec<Phoneme> {
        match &self.effect {
            JunctureEffect::ReplaceLeft(p) => p.clone(),
            JunctureEffect::DeleteLeft => Vec::new(),
        }
    }
}

/// Perfil de sotaque de uma pessoa.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AccentProfile {
    /// Rótulo amigável (ex.: "Carioca (RJ)" ou "Personalizado").
    pub label: String,
    /// Regras intra-palavra, na ordem de aplicação.
    pub segment_rules: Vec<PronunciationRule>,
    /// Regras de junção, na ordem de aplicação.
    pub juncture_rules: Vec<JunctureRule>,
}

impl AccentProfile {
    /// Perfil sem nenhuma regra: pronúncia de citação.
    pub fn neutral() -> Self {
        Self {
            label: "Neutro (sem amostras com fonemas)".into(),
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
