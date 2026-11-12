//! Estrutura de derivação: mantém, para cada fonema canônico, sua realização
//! atual ("superfície"). Permite aplicar regras em sequência sem perder o
//! alinhamento com a forma canônica (necessário para o aprendizado).
use super::rule::{JunctureRule, Position, PronunciationRule};
use crate::phonetics::{Phoneme, PhoneticUtterance, PhoneticWord};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Slot {
    pub canonical: Phoneme,
    pub surface: Vec<Phoneme>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct WordDerivation {
    pub orthography: String,
    pub slots: Vec<Slot>,
    pub stress: Option<usize>,
    pub pause_after: bool,
}

/// Endereço de um fonema canônico: (palavra, posição).
pub type Site = (usize, usize);

#[derive(Debug, Clone, PartialEq)]
pub struct UtteranceDerivation {
    pub(crate) words: Vec<WordDerivation>,
}

impl UtteranceDerivation {
    pub fn new(utterance: &PhoneticUtterance) -> Self {
        let words = utterance
            .words
            .iter()
            .map(|w| WordDerivation {
                orthography: w.orthography.clone(),
                slots: w
                    .phonemes
                    .iter()
                    .map(|p| Slot {
                        canonical: p.clone(),
                        surface: vec![p.clone()],
                    })
                    .collect(),
                stress: w.stress,
                pause_after: w.pause_after,
            })
            .collect();
        Self { words }
    }

    /// Lista os endereços canônicos na ordem (para alinhar com o observado).
    pub fn sites(&self) -> Vec<Site> {
        self.words
            .iter()
            .enumerate()
            .flat_map(|(w, word)| (0..word.slots.len()).map(move |i| (w, i)))
            .collect()
    }

    pub fn surface(&self, (w, i): Site) -> &[Phoneme] {
        &self.words[w].slots[i].surface
    }

    pub fn set_surface(&mut self, (w, i): Site, surface: Vec<Phoneme>) {
        self.words[w].slots[i].surface = surface;
    }

    fn prev_phone(&self, w: usize, i: usize) -> Option<&Phoneme> {
        self.words[w].slots[..i]
            .iter()
            .rev()
            .find_map(|s| s.surface.last())
    }

    fn following_phones(&self, w: usize, i: usize) -> impl Iterator<Item = &Phoneme> {
        self.words[w].slots[i + 1..]
            .iter()
            .flat_map(|s| s.surface.iter())
    }

    fn position_ok(&self, w: usize, i: usize, position: Position) -> bool {
        let phone = &self.words[w].slots[i].surface[0];
        let next = self.following_phones(w, i).next();
        match position {
            Position::Anywhere => true,
            Position::WordInitial => self.prev_phone(w, i).is_none(),
            Position::WordFinal => next.is_none(),
            Position::Coda => phone.is_consonant() && next.is_none_or(|n| n.is_consonant()),
            Position::Onset => next.is_some_and(|n| n.is_vowel() || n.is_glide()),
            Position::FinalVowel => {
                phone.is_vowel() && self.following_phones(w, i).all(|p| !p.is_vowel())
            }
            Position::Pretonic => phone.is_vowel() && self.words[w].stress.is_some_and(|s| i < s),
        }
    }

    /// Locais onde a regra pode ser aplicada no estado atual da derivação.
    pub fn rule_sites(&self, rule: &PronunciationRule) -> Vec<Site> {
        let mut sites = Vec::new();
        for (w, word) in self.words.iter().enumerate() {
            for (i, slot) in word.slots.iter().enumerate() {
                if slot.surface.len() != 1 || !rule.target.matches(&slot.surface[0]) {
                    continue;
                }
                let ctx = &rule.context;
                if ctx.unstressed_only && word.stress == Some(i) {
                    continue;
                }
                if !self.position_ok(w, i, ctx.position) {
                    continue;
                }
                if ctx.blocked_before_vowel_word
                    && self.last_nonempty_slot(w) == Some(i)
                    && self.next_word_first_phone(w).is_some_and(Phoneme::is_vowel)
                {
                    continue;
                }
                if let Some(m) = &ctx.preceding {
                    if !self.prev_phone(w, i).is_some_and(|p| m.matches(p)) {
                        continue;
                    }
                }
                if let Some(m) = &ctx.following {
                    if !self
                        .following_phones(w, i)
                        .next()
                        .is_some_and(|p| m.matches(p))
                    {
                        continue;
                    }
                }
                sites.push((w, i));
            }
        }
        sites
    }

    pub fn apply_rule(&mut self, rule: &PronunciationRule) {
        for site in self.rule_sites(rule) {
            self.set_surface(site, rule.replacement.clone());
        }
    }

    pub(crate) fn next_word_first_phone(&self, w: usize) -> Option<&Phoneme> {
        if self.words[w].pause_after {
            return None;
        }
        self.words
            .get(w + 1)?
            .slots
            .iter()
            .find_map(|s| s.surface.first())
    }

    pub(crate) fn is_word_final_site(&self, (w, i): Site) -> bool {
        self.last_nonempty_slot(w) == Some(i)
    }

    pub(crate) fn is_word_initial_site(&self, (w, i): Site) -> bool {
        self.prev_phone(w, i).is_none()
    }

    pub(crate) fn is_coda_site(&self, (w, i): Site) -> bool {
        self.words[w].slots[i].surface.len() == 1 && self.position_ok(w, i, Position::Coda)
    }

    pub(crate) fn last_nonempty_slot(&self, w: usize) -> Option<usize> {
        self.words[w]
            .slots
            .iter()
            .rposition(|s| !s.surface.is_empty())
    }

    /// Locais (fim da palavra esquerda) onde a regra de junção se aplica.
    pub fn juncture_sites(&self, rule: &JunctureRule) -> Vec<Site> {
        let mut sites = Vec::new();
        for w in 0..self.words.len().saturating_sub(1) {
            let Some(li) = self.last_nonempty_slot(w) else {
                continue;
            };
            let Some(right) = self.next_word_first_phone(w) else {
                continue;
            };
            let left = self.words[w].slots[li].surface.last().expect("non-empty");
            if rule.left_unstressed_only && self.words[w].stress == Some(li) {
                continue;
            }
            if rule.left.matches(left) && rule.right.matches(right) {
                sites.push((w, li));
            }
        }
        sites
    }

    pub fn apply_juncture(&mut self, rule: &JunctureRule) {
        for (w, i) in self.juncture_sites(rule) {
            let mut surface = self.words[w].slots[i].surface.clone();
            surface.pop();
            surface.extend(rule.expected_left());
            self.set_surface((w, i), surface);
        }
    }

    /// Converte o estado atual de volta em um enunciado fonético.
    pub fn to_utterance(&self) -> PhoneticUtterance {
        let words = self
            .words
            .iter()
            .map(|w| {
                let mut phonemes = Vec::new();
                let mut stress = None;
                for (i, slot) in w.slots.iter().enumerate() {
                    if w.stress == Some(i) {
                        stress = slot
                            .surface
                            .iter()
                            .position(Phoneme::is_vowel)
                            .map(|p| phonemes.len() + p);
                    }
                    phonemes.extend(slot.surface.iter().cloned());
                }
                PhoneticWord::new(w.orthography.clone(), phonemes, stress).with_pause(w.pause_after)
            })
            .collect();
        PhoneticUtterance::new(words)
    }
}
