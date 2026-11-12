//! Representação fonética em IPA (Alfabeto Fonético Internacional).
use serde::{Deserialize, Serialize};
use std::fmt;

/// Um segmento sonoro (ex.: `t`, `tʃ`, `ɾ`, `ã`). Africadas são um único fonema.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Phoneme(String);

const VOWEL_BASES: &[char] = &[
    'a', 'e', 'i', 'o', 'u', 'ɛ', 'ɔ', 'ɐ', 'ə', 'ɨ', 'ɪ', 'ʊ', 'ã', 'õ', 'ẽ', 'ĩ', 'ũ',
];
const GLIDES: &[&str] = &["j", "w", "j̃", "w̃"];
const VOICELESS: &[&str] = &["p", "t", "k", "f", "s", "ʃ", "x", "h", "tʃ", "χ", "θ"];

impl Phoneme {
    pub fn new(symbol: impl Into<String>) -> Self {
        Self(symbol.into())
    }
    pub fn symbol(&self) -> &str {
        &self.0
    }
    pub fn is_glide(&self) -> bool {
        GLIDES.contains(&self.0.as_str())
    }
    pub fn is_vowel(&self) -> bool {
        !self.is_glide()
            && self
                .0
                .chars()
                .next()
                .is_some_and(|c| VOWEL_BASES.contains(&c))
    }
    pub fn is_consonant(&self) -> bool {
        !self.is_vowel() && !self.is_glide()
    }
    pub fn is_voiced(&self) -> bool {
        !VOICELESS.contains(&self.0.as_str())
    }
    pub fn is_high_front(&self) -> bool {
        matches!(self.0.as_str(), "i" | "ĩ" | "ɪ" | "j")
    }
}

impl fmt::Display for Phoneme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Classes naturais usadas nos contextos das regras.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PhoneClass {
    Any,
    Vowel,
    Consonant,
    Glide,
    VoicedConsonant,
    VoicelessConsonant,
    HighFront,
}

/// Casa um fonema com um padrão.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PhoneMatcher {
    Exact(Phoneme),
    OneOf(Vec<Phoneme>),
    Class(PhoneClass),
}

impl PhoneMatcher {
    pub fn exact(s: &str) -> Self {
        Self::Exact(Phoneme::new(s))
    }
    pub fn one_of(symbols: &[&str]) -> Self {
        Self::OneOf(symbols.iter().map(|s| Phoneme::new(*s)).collect())
    }
    pub fn matches(&self, p: &Phoneme) -> bool {
        match self {
            Self::Exact(e) => e == p,
            Self::OneOf(set) => set.contains(p),
            Self::Class(c) => match c {
                PhoneClass::Any => true,
                PhoneClass::Vowel => p.is_vowel(),
                PhoneClass::Consonant => p.is_consonant(),
                PhoneClass::Glide => p.is_glide(),
                PhoneClass::VoicedConsonant => p.is_consonant() && p.is_voiced(),
                PhoneClass::VoicelessConsonant => p.is_consonant() && !p.is_voiced(),
                PhoneClass::HighFront => p.is_high_front(),
            },
        }
    }
}

/// Palavra transcrita. `stress` aponta o índice do fonema da vogal tônica.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhoneticWord {
    pub orthography: String,
    pub phonemes: Vec<Phoneme>,
    pub stress: Option<usize>,
    /// Há pausa (pontuação) depois da palavra: bloqueia a junção com a próxima.
    #[serde(default)]
    pub pause_after: bool,
}

impl PhoneticWord {
    pub fn new(
        orthography: impl Into<String>,
        phonemes: Vec<Phoneme>,
        stress: Option<usize>,
    ) -> Self {
        Self {
            orthography: orthography.into(),
            phonemes,
            stress,
            pause_after: false,
        }
    }
    pub fn with_pause(mut self, pause: bool) -> Self {
        self.pause_after = pause;
        self
    }
    pub fn ipa(&self) -> String {
        self.phonemes.iter().map(Phoneme::symbol).collect()
    }
    pub fn syllable_count(&self) -> usize {
        self.phonemes.iter().filter(|p| p.is_vowel()).count()
    }
}

/// Frase/enunciado: sequência de palavras faladas em um mesmo grupo prosódico.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PhoneticUtterance {
    pub words: Vec<PhoneticWord>,
}

impl PhoneticUtterance {
    pub fn new(words: Vec<PhoneticWord>) -> Self {
        Self { words }
    }
    pub fn ipa(&self) -> String {
        self.words
            .iter()
            .map(PhoneticWord::ipa)
            .collect::<Vec<_>>()
            .join(" ")
    }
    pub fn syllable_count(&self) -> usize {
        self.words.iter().map(PhoneticWord::syllable_count).sum()
    }
    pub fn phonemes(&self) -> Vec<Phoneme> {
        self.words
            .iter()
            .flat_map(|w| w.phonemes.iter().cloned())
            .collect()
    }
}

/// Converte uma string IPA em fonemas. Ignora espaços, marcas de acento
/// (ˈ ˌ) e fronteiras silábicas; agrupa africadas (tʃ, dʒ, t͡ʃ) e diacríticos.
pub fn parse_ipa(input: &str) -> Vec<Phoneme> {
    let mut out: Vec<String> = Vec::new();
    let mut join_next = false;
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        i += 1;
        if c.is_whitespace() || matches!(c, 'ˈ' | 'ˌ' | '.' | '|' | '/' | '[' | ']') {
            continue;
        }
        if c == '\u{361}' || c == '\u{35C}' {
            join_next = true;
            continue;
        }
        let is_modifier =
            ('\u{300}'..='\u{36F}').contains(&c) || matches!(c, 'ː' | 'ʰ' | 'ʲ' | 'ʷ');
        if is_modifier || join_next {
            if let Some(last) = out.last_mut() {
                if c == '\u{303}' {
                    if let Some(composed) = compose_tilde(last) {
                        *last = composed;
                        continue;
                    }
                }
                last.push(c);
                if !is_modifier {
                    join_next = false;
                }
                continue;
            }
        }
        let affricate = matches!((c, chars.get(i)), ('t', Some('ʃ')) | ('d', Some('ʒ')));
        if affricate {
            out.push(format!("{c}{}", chars[i]));
            i += 1;
        } else {
            out.push(c.to_string());
        }
    }
    out.into_iter().map(Phoneme::new).collect()
}

/// Normaliza vogal + til combinante para a forma pré-composta (a + ◌̃ → ã).
fn compose_tilde(base: &str) -> Option<String> {
    let composed = match base {
        "a" => "ã",
        "e" => "ẽ",
        "i" => "ĩ",
        "o" => "õ",
        "u" => "ũ",
        _ => return None,
    };
    Some(composed.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn syms(v: &[Phoneme]) -> Vec<&str> {
        v.iter().map(Phoneme::symbol).collect()
    }

    #[test]
    fn parses_affricates_and_diacritics() {
        assert_eq!(syms(&parse_ipa("ˈnoj.tʃi")), ["n", "o", "j", "tʃ", "i"]);
        assert_eq!(syms(&parse_ipa("t͡ʃia")), ["tʃ", "i", "a"]);
        assert_eq!(syms(&parse_ipa("ma\u{303}j")), ["m", "ã", "j"]);
        assert_eq!(syms(&parse_ipa("[dʒa]")), ["dʒ", "a"]);
    }

    #[test]
    fn classifies() {
        assert!(Phoneme::new("ã").is_vowel());
        assert!(Phoneme::new("j").is_glide());
        assert!(Phoneme::new("tʃ").is_consonant());
        assert!(!Phoneme::new("tʃ").is_voiced());
        assert!(PhoneMatcher::Class(PhoneClass::VoicedConsonant).matches(&Phoneme::new("m")));
        assert!(PhoneMatcher::Class(PhoneClass::HighFront).matches(&Phoneme::new("i")));
    }
}
