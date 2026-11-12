//! Catalog of known Portuguese phenomena and regional presets.
//!
//! The catalog works as a set of **hypotheses**: the analyzer measures, in the
//! person's samples, how many times each phenomenon had the opportunity to
//! occur and how many times it actually did. Order matters (rules can feed
//! each other).
use super::rule::{
    JunctureEffect, JunctureRule, PhoneContext, Position, PronunciationRule, RulePhenomenon,
};
use crate::phonetics::{PhoneClass, PhoneMatcher, Phoneme};

#[derive(Debug, Clone, PartialEq)]
pub struct RuleCatalog {
    pub segment_rules: Vec<PronunciationRule>,
    pub juncture_rules: Vec<JunctureRule>,
}

/// Set of phenomena typical of a region, used to label the profile.
#[derive(Debug, Clone, PartialEq)]
pub struct AccentPreset {
    pub name: &'static str,
    pub codes: &'static [&'static str],
}

impl RuleCatalog {
    /// Portuguese phenomena (Brazilian and European varieties).
    pub fn portuguese() -> Self {
        use Position::*;
        use RulePhenomenon::*;
        let ex = PhoneMatcher::exact;
        let vowel = || PhoneMatcher::Class(PhoneClass::Vowel);
        let r = PronunciationRule::new;
        let segment_rules = vec![
            // ---- Word endings ----
            r(
                "final-e-raising",
                "unstressed final 'e' sounds like 'i' (leite → leiti)",
                WordEnding,
                ex("e"),
                &["i"],
                PhoneContext::at(FinalVowel).unstressed(),
            ),
            r(
                "final-e-centralization",
                "unstressed final 'e' reduced to 'ɨ' (European Portuguese)",
                WordEnding,
                ex("e"),
                &["ɨ"],
                PhoneContext::at(FinalVowel).unstressed(),
            ),
            r(
                "final-o-raising",
                "unstressed final 'o' sounds like 'u' (gato → gatu)",
                WordEnding,
                ex("o"),
                &["u"],
                PhoneContext::at(FinalVowel).unstressed(),
            ),
            r(
                "final-a-reduction",
                "unstressed final 'a' reduced to 'ɐ'",
                WordEnding,
                ex("a"),
                &["ɐ"],
                PhoneContext::at(FinalVowel).unstressed(),
            ),
            r(
                "word-final-r-deletion",
                "final 'r' is dropped (falar → falá)",
                WordEnding,
                ex("ɾ"),
                &[],
                PhoneContext::at(WordFinal).blocked_before_vowel_word(),
            ),
            // ---- Pronunciation of letters/sounds ----
            r(
                "t-palatalization",
                "'t' before 'i' becomes 'tʃ' (tia → tchia)",
                Segmental,
                ex("t"),
                &["tʃ"],
                PhoneContext::at(Anywhere).followed_by(PhoneMatcher::Class(PhoneClass::HighFront)),
            ),
            r(
                "d-palatalization",
                "'d' before 'i' becomes 'dʒ' (dia → djia)",
                Segmental,
                ex("d"),
                &["dʒ"],
                PhoneContext::at(Anywhere).followed_by(PhoneMatcher::Class(PhoneClass::HighFront)),
            ),
            r(
                "coda-l-vocalization",
                "syllable-final 'l' becomes 'w' (Brasil → Brasiu)",
                Segmental,
                ex("l"),
                &["w"],
                PhoneContext::at(Coda),
            ),
            r(
                "coda-s-voiced-palatalization",
                "'s' before a voiced consonant becomes 'ʒ' (mesmo → meʒmo)",
                Segmental,
                ex("s"),
                &["ʒ"],
                PhoneContext::at(Coda)
                    .followed_by(PhoneMatcher::Class(PhoneClass::VoicedConsonant)),
            ),
            r(
                "coda-s-palatalization",
                "syllable-final 's' hushed as 'ʃ' (festa → feʃta)",
                Segmental,
                ex("s"),
                &["ʃ"],
                PhoneContext::at(Coda),
            ),
            r(
                "coda-r-aspiration",
                "syllable-final 'r' aspirated as 'h' (porta → pohta)",
                Segmental,
                ex("ɾ"),
                &["h"],
                PhoneContext::at(Coda),
            ),
            r(
                "coda-r-retroflex",
                "syllable-final 'r' retroflex 'ɻ' (porta → poɻta, caipira)",
                Segmental,
                ex("ɾ"),
                &["ɻ"],
                PhoneContext::at(Coda),
            ),
            r(
                "strong-r-glottal",
                "strong/initial 'r' aspirated as 'h' (rato → hato)",
                Segmental,
                ex("x"),
                &["h"],
                PhoneContext::at(Anywhere),
            ),
            r(
                "strong-r-trill",
                "strong 'r' trilled 'r' (rato → rrato)",
                Segmental,
                ex("x"),
                &["r"],
                PhoneContext::at(Anywhere),
            ),
            r(
                "lh-yeism",
                "'lh' becomes 'i' (mulher → muié)",
                Segmental,
                ex("ʎ"),
                &["j"],
                PhoneContext::at(Anywhere),
            ),
            r(
                "pretonic-e-opening",
                "open pretonic 'e' as 'ɛ' (relógio → rélógio)",
                Segmental,
                ex("e"),
                &["ɛ"],
                PhoneContext::at(Pretonic),
            ),
            r(
                "pretonic-o-opening",
                "open pretonic 'o' as 'ɔ' (morena → mórena)",
                Segmental,
                ex("o"),
                &["ɔ"],
                PhoneContext::at(Pretonic),
            ),
        ];

        let j = JunctureRule::new;
        let juncture_rules = vec![
            j(
                "sibilant-voicing",
                "final 's' + vowel becomes 'z' (os amigos → uzamigus)",
                PhoneMatcher::one_of(&["s", "ʃ"]),
                vowel(),
                JunctureEffect::ReplaceLeft(vec![Phoneme::new("z")]),
            ),
            j(
                "rhotic-linking",
                "final 'r' + vowel becomes a tap (mar azul → maɾazul)",
                PhoneMatcher::one_of(&["h", "ɻ", "x"]),
                vowel(),
                JunctureEffect::ReplaceLeft(vec![Phoneme::new("ɾ")]),
            ),
            j(
                "vowel-elision",
                "unstressed final 'a' is elided before a vowel (casa amarela → casamarela)",
                PhoneMatcher::one_of(&["a", "ɐ"]),
                vowel(),
                JunctureEffect::DeleteLeft,
            )
            .left_unstressed(),
            j(
                "high-vowel-gliding",
                "unstressed final 'i' becomes a glide (de outro → djoutro)",
                PhoneMatcher::exact("i"),
                vowel(),
                JunctureEffect::ReplaceLeft(vec![Phoneme::new("j")]),
            )
            .left_unstressed(),
            j(
                "sibilant-degemination",
                "'s' + 's' merge (mas sempre → masempre)",
                PhoneMatcher::one_of(&["s", "ʃ", "z"]),
                PhoneMatcher::exact("s"),
                JunctureEffect::DeleteLeft,
            ),
        ];
        Self {
            segment_rules,
            juncture_rules,
        }
    }

    pub fn presets() -> &'static [AccentPreset] {
        &[
            AccentPreset {
                name: "Carioca (RJ)",
                codes: &[
                    "final-e-raising",
                    "final-o-raising",
                    "t-palatalization",
                    "d-palatalization",
                    "coda-l-vocalization",
                    "coda-s-voiced-palatalization",
                    "coda-s-palatalization",
                    "word-final-r-deletion",
                    "coda-r-aspiration",
                    "strong-r-glottal",
                    "sibilant-voicing",
                    "rhotic-linking",
                    "high-vowel-gliding",
                ],
            },
            AccentPreset {
                name: "Paulistano (SP)",
                codes: &[
                    "final-e-raising",
                    "final-o-raising",
                    "t-palatalization",
                    "d-palatalization",
                    "coda-l-vocalization",
                    "word-final-r-deletion",
                    "sibilant-voicing",
                    "vowel-elision",
                    "high-vowel-gliding",
                ],
            },
            AccentPreset {
                name: "Caipira (countryside)",
                codes: &[
                    "final-e-raising",
                    "final-o-raising",
                    "t-palatalization",
                    "d-palatalization",
                    "coda-l-vocalization",
                    "coda-r-retroflex",
                    "lh-yeism",
                    "word-final-r-deletion",
                    "sibilant-voicing",
                ],
            },
            AccentPreset {
                name: "Mineiro (MG)",
                codes: &[
                    "final-e-raising",
                    "final-o-raising",
                    "t-palatalization",
                    "d-palatalization",
                    "coda-l-vocalization",
                    "coda-r-aspiration",
                    "strong-r-glottal",
                    "word-final-r-deletion",
                    "vowel-elision",
                    "sibilant-voicing",
                    "high-vowel-gliding",
                    "sibilant-degemination",
                ],
            },
            AccentPreset {
                name: "Nordestino (NE)",
                codes: &[
                    "final-e-raising",
                    "final-o-raising",
                    "pretonic-e-opening",
                    "pretonic-o-opening",
                    "coda-l-vocalization",
                    "coda-r-aspiration",
                    "strong-r-glottal",
                    "word-final-r-deletion",
                    "sibilant-voicing",
                ],
            },
            AccentPreset {
                name: "Gaúcho (RS)",
                codes: &[
                    "t-palatalization",
                    "d-palatalization",
                    "final-o-raising",
                    "coda-l-vocalization",
                    "strong-r-trill",
                    "sibilant-voicing",
                ],
            },
            AccentPreset {
                name: "European Portuguese (PT)",
                codes: &[
                    "final-e-centralization",
                    "final-o-raising",
                    "final-a-reduction",
                    "coda-s-voiced-palatalization",
                    "coda-s-palatalization",
                    "strong-r-trill",
                    "sibilant-voicing",
                ],
            },
        ]
    }

    /// Preset closest to the set of adopted rules (Jaccard similarity).
    pub fn closest_preset(codes: &[&str]) -> Option<(&'static str, f32)> {
        if codes.is_empty() {
            return None;
        }
        Self::presets()
            .iter()
            .map(|p| {
                let inter = p.codes.iter().filter(|c| codes.contains(c)).count() as f32;
                let union = (p.codes.len() + codes.len()) as f32 - inter;
                (p.name, inter / union)
            })
            .max_by(|a, b| a.1.total_cmp(&b.1))
    }

    pub fn preset(name: &str) -> Option<&'static AccentPreset> {
        let needle = name.to_lowercase();
        Self::presets()
            .iter()
            .find(|p| p.name.to_lowercase().contains(&needle))
    }
}
