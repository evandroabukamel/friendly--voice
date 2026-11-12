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
                "'e' átono final soa como 'i' (leite → leiti)",
                WordEnding,
                ex("e"),
                &["i"],
                PhoneContext::at(FinalVowel).unstressed(),
            ),
            r(
                "final-e-centralization",
                "'e' átono final reduzido a 'ɨ' (português europeu)",
                WordEnding,
                ex("e"),
                &["ɨ"],
                PhoneContext::at(FinalVowel).unstressed(),
            ),
            r(
                "final-o-raising",
                "'o' átono final soa como 'u' (gato → gatu)",
                WordEnding,
                ex("o"),
                &["u"],
                PhoneContext::at(FinalVowel).unstressed(),
            ),
            r(
                "final-a-reduction",
                "'a' átono final reduzido a 'ɐ'",
                WordEnding,
                ex("a"),
                &["ɐ"],
                PhoneContext::at(FinalVowel).unstressed(),
            ),
            r(
                "word-final-r-deletion",
                "'r' final cai (falar → falá)",
                WordEnding,
                ex("ɾ"),
                &[],
                PhoneContext::at(WordFinal).blocked_before_vowel_word(),
            ),
            // ---- Pronunciation of letters/sounds ----
            r(
                "t-palatalization",
                "'t' antes de 'i' vira 'tʃ' (tia → tchia)",
                Segmental,
                ex("t"),
                &["tʃ"],
                PhoneContext::at(Anywhere).followed_by(PhoneMatcher::Class(PhoneClass::HighFront)),
            ),
            r(
                "d-palatalization",
                "'d' antes de 'i' vira 'dʒ' (dia → djia)",
                Segmental,
                ex("d"),
                &["dʒ"],
                PhoneContext::at(Anywhere).followed_by(PhoneMatcher::Class(PhoneClass::HighFront)),
            ),
            r(
                "coda-l-vocalization",
                "'l' em final de sílaba vira 'w' (Brasil → Brasiu)",
                Segmental,
                ex("l"),
                &["w"],
                PhoneContext::at(Coda),
            ),
            r(
                "coda-s-voiced-palatalization",
                "'s' antes de consoante sonora vira 'ʒ' (mesmo → meʒmo)",
                Segmental,
                ex("s"),
                &["ʒ"],
                PhoneContext::at(Coda)
                    .followed_by(PhoneMatcher::Class(PhoneClass::VoicedConsonant)),
            ),
            r(
                "coda-s-palatalization",
                "'s' em final de sílaba chiado 'ʃ' (festa → feʃta)",
                Segmental,
                ex("s"),
                &["ʃ"],
                PhoneContext::at(Coda),
            ),
            r(
                "coda-r-aspiration",
                "'r' em final de sílaba aspirado 'h' (porta → pohta)",
                Segmental,
                ex("ɾ"),
                &["h"],
                PhoneContext::at(Coda),
            ),
            r(
                "coda-r-retroflex",
                "'r' em final de sílaba retroflexo 'ɻ' (porta → poɻta, caipira)",
                Segmental,
                ex("ɾ"),
                &["ɻ"],
                PhoneContext::at(Coda),
            ),
            r(
                "strong-r-glottal",
                "'r' forte/inicial aspirado 'h' (rato → hato)",
                Segmental,
                ex("x"),
                &["h"],
                PhoneContext::at(Anywhere),
            ),
            r(
                "strong-r-trill",
                "'r' forte vibrante 'r' (rato → rrato)",
                Segmental,
                ex("x"),
                &["r"],
                PhoneContext::at(Anywhere),
            ),
            r(
                "lh-yeism",
                "'lh' vira 'i' (mulher → muié)",
                Segmental,
                ex("ʎ"),
                &["j"],
                PhoneContext::at(Anywhere),
            ),
            r(
                "pretonic-e-opening",
                "'e' pretônico aberto 'ɛ' (relógio → rélógio)",
                Segmental,
                ex("e"),
                &["ɛ"],
                PhoneContext::at(Pretonic),
            ),
            r(
                "pretonic-o-opening",
                "'o' pretônico aberto 'ɔ' (morena → mórena)",
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
                "'s' final + vogal vira 'z' (os amigos → uzamigus)",
                PhoneMatcher::one_of(&["s", "ʃ"]),
                vowel(),
                JunctureEffect::ReplaceLeft(vec![Phoneme::new("z")]),
            ),
            j(
                "rhotic-linking",
                "'r' final + vogal vira tepe (mar azul → maɾazul)",
                PhoneMatcher::one_of(&["h", "ɻ", "x"]),
                vowel(),
                JunctureEffect::ReplaceLeft(vec![Phoneme::new("ɾ")]),
            ),
            j(
                "vowel-elision",
                "vogal 'a' átona final some antes de vogal (casa amarela → casamarela)",
                PhoneMatcher::one_of(&["a", "ɐ"]),
                vowel(),
                JunctureEffect::DeleteLeft,
            )
            .left_unstressed(),
            j(
                "high-vowel-gliding",
                "'i' átono final vira semivogal (de outro → djoutro)",
                PhoneMatcher::exact("i"),
                vowel(),
                JunctureEffect::ReplaceLeft(vec![Phoneme::new("j")]),
            )
            .left_unstressed(),
            j(
                "sibilant-degemination",
                "'s' + 's' se fundem (mas sempre → masempre)",
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
                name: "Caipira / interior",
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
                name: "Português europeu (PT)",
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
