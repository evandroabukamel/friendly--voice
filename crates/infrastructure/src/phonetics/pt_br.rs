//! Rule-based G2P (grapheme → phoneme) for Brazilian Portuguese.
//!
//! Produces the **citation form** — close to the spelling, without regional traits
//! (`t` stays `t`, final `e` stays `e`, coda `s` stays `s`). Accent traits are
//! the domain's responsibility (`AccentRenderer`).
use friendly_voice_application::ports::Phonemizer;
use friendly_voice_domain::phonetics::{Phoneme, PhoneticUtterance, PhoneticWord};

/// Unstressed words (clitics): no stress of their own in the sentence.
const CLITICS: &[&str] = &[
    "o", "a", "os", "as", "e", "de", "do", "da", "dos", "das", "em", "no", "na", "nos", "nas",
    "um", "uns", "que", "se", "me", "te", "lhe", "lhes", "por", "com", "pra", "pro", "para", "mas",
    "ou", "nem", "ao", "aos", "à", "às", "num", "numa",
];

#[derive(Debug, Clone, Copy, Default)]
pub struct PortugueseG2p;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Kind {
    Nucleus,
    Glide,
    Silent,
    Consonant,
}

fn is_vowel(c: char) -> bool {
    "aeiouáàâãéêíóôõúü".contains(c)
}

fn base(c: char) -> char {
    match c {
        'á' | 'à' | 'â' | 'ã' => 'a',
        'é' | 'ê' => 'e',
        'í' => 'i',
        'ó' | 'ô' | 'õ' => 'o',
        'ú' | 'ü' => 'u',
        c => c,
    }
}

fn is_front(c: Option<&char>) -> bool {
    c.is_some_and(|c| matches!(base(*c), 'e' | 'i'))
}

fn has_stress_mark(c: char) -> bool {
    "áâéêíóôú".contains(c)
}

fn nasal(v: &str) -> &'static str {
    match v {
        "a" | "ã" => "ã",
        "e" | "ɛ" => "ẽ",
        "i" => "ĩ",
        "o" | "ɔ" | "õ" => "õ",
        _ => "ũ",
    }
}

fn vowel_phone(c: char) -> &'static str {
    match c {
        'a' | 'á' | 'à' | 'â' => "a",
        'ã' => "ã",
        'e' | 'ê' => "e",
        'é' => "ɛ",
        'i' | 'í' => "i",
        'o' | 'ô' => "o",
        'ó' => "ɔ",
        'õ' => "õ",
        _ => "u",
    }
}

fn classify(w: &[char]) -> Vec<Kind> {
    let mut kinds = Vec::with_capacity(w.len());
    for (i, &c) in w.iter().enumerate() {
        let prev = i.checked_sub(1).map(|p| w[p]);
        let next = w.get(i + 1).copied();
        let after_qg = c == 'u' && matches!(prev, Some('q' | 'g'));
        // Nasal diphthongs: "ão", "ãe", "õe".
        let nasal_diphthong = matches!(prev, Some('ã' | 'õ')) && matches!(c, 'o' | 'e');
        // Falling diphthongs: "pai", "meu", "muito" (except "rainha", "ruim").
        let falling_diphthong = matches!(c, 'i' | 'u')
            && prev.is_some_and(|p| is_vowel(p) && base(p) != base(c))
            && kinds.last() == Some(&Kind::Nucleus)
            && !(next == Some('n') && w.get(i + 2) == Some(&'h'))
            && !(matches!(next, Some('m' | 'n' | 'r' | 'z' | 'l')) && i + 2 == w.len());
        let kind = if !is_vowel(c) {
            Kind::Consonant
        } else if after_qg && is_front(next.as_ref()) {
            Kind::Silent
        } else if (after_qg && next.is_some_and(is_vowel)) || nasal_diphthong || falling_diphthong {
            Kind::Glide
        } else {
            Kind::Nucleus
        };
        kinds.push(kind);
    }
    kinds
}

/// Index (in the word) of the stressed vowel.
fn stressed_letter(w: &[char], kinds: &[Kind], word: &str) -> Option<usize> {
    let nuclei: Vec<usize> = (0..w.len())
        .filter(|&i| kinds[i] == Kind::Nucleus)
        .collect();
    if nuclei.is_empty() {
        return None;
    }
    if let Some(&i) = nuclei.iter().find(|&&i| has_stress_mark(w[i])) {
        return Some(i);
    }
    if let Some(&i) = nuclei.iter().find(|&&i| matches!(w[i], 'ã' | 'õ')) {
        return Some(i);
    }
    if CLITICS.contains(&word) {
        return None;
    }
    if nuclei.len() == 1 {
        return Some(nuclei[0]);
    }
    let stem = word.strip_suffix('s').unwrap_or(word);
    let paroxytone = stem.ends_with(['a', 'e', 'o'])
        || word.ends_with("am")
        || word.ends_with("em")
        || word.ends_with("ens");
    Some(if paroxytone {
        nuclei[nuclei.len() - 2]
    } else {
        nuclei[nuclei.len() - 1]
    })
}

impl PortugueseG2p {
    pub fn word(&self, word: &str) -> PhoneticWord {
        let lower = word.to_lowercase();
        let w: Vec<char> = lower.chars().collect();
        let kinds = classify(&w);
        let stress_letter = stressed_letter(&w, &kinds, &lower);
        let mut out: Vec<&'static str> = Vec::new();
        let mut stress = None;
        let n = w.len();
        let at = |i: usize| w.get(i).copied();
        let mut i = 0;
        while i < n {
            let c = w[i];
            let prev = i.checked_sub(1).map(|p| w[p]);
            let next = at(i + 1);
            let prev_is_vowel = prev.is_some_and(is_vowel);
            let next_is_vowel = next.is_some_and(is_vowel);
            match kinds[i] {
                Kind::Silent => {}
                Kind::Glide => out.push(if base(c) == 'i' || base(c) == 'e' {
                    "j"
                } else {
                    "w"
                }),
                Kind::Nucleus => {
                    if Some(i) == stress_letter {
                        stress = Some(out.len());
                    }
                    let mut v = vowel_phone(c);
                    let after = at(i + 2);
                    let nasal_coda = matches!(next, Some('m' | 'n'))
                        && !(next == Some('n') && after == Some('h'))
                        && after.is_none_or(|a| !is_vowel(a) && a != 'h');
                    if nasal_coda {
                        v = nasal(v);
                        let final_m = next == Some('m') && after.is_none();
                        let final_ens = next == Some('n') && after == Some('s') && i + 3 == n;
                        out.push(v);
                        match base(c) {
                            'a' if final_m => out.push("w"),
                            'e' if final_m || final_ens => out.push("j"),
                            _ => {}
                        }
                        i += 2;
                        continue;
                    }
                    out.push(v);
                }
                Kind::Consonant => match c {
                    'c' if next == Some('h') => {
                        out.push("ʃ");
                        i += 1;
                    }
                    'c' if is_front(next.as_ref()) => out.push("s"),
                    'c' => out.push("k"),
                    'ç' => out.push("s"),
                    'g' if is_front(next.as_ref()) => out.push("ʒ"),
                    'g' => out.push("g"),
                    'h' => {}
                    'j' => out.push("ʒ"),
                    'l' if next == Some('h') => {
                        out.push("ʎ");
                        i += 1;
                    }
                    'n' if next == Some('h') => {
                        out.push("ɲ");
                        i += 1;
                    }
                    'q' => out.push("k"),
                    'r' if next == Some('r') => {
                        out.push("x");
                        i += 1;
                    }
                    'r' if prev.is_none() || matches!(prev, Some('n' | 'l' | 's')) => out.push("x"),
                    'r' => out.push("ɾ"),
                    's' if next == Some('s') => {
                        out.push("s");
                        i += 1;
                    }
                    's' if next == Some('c') && is_front(at(i + 2).as_ref()) => {
                        out.push("s");
                        i += 1;
                    }
                    's' if prev_is_vowel && next_is_vowel => out.push("z"),
                    's' => out.push("s"),
                    'x' if prev.is_some_and(|p| base(p) == 'e') && i == 1 && next_is_vowel => {
                        out.push("z")
                    }
                    'x' => out.push("ʃ"),
                    'z' if next.is_none() => out.push("s"),
                    'y' => out.push("i"),
                    'w' => out.push("w"),
                    'k' => out.push("k"),
                    'b' => out.push("b"),
                    'd' => out.push("d"),
                    'f' => out.push("f"),
                    'l' => out.push("l"),
                    'm' => out.push("m"),
                    'n' => out.push("n"),
                    'p' => out.push("p"),
                    't' => out.push("t"),
                    'v' => out.push("v"),
                    'z' => out.push("z"),
                    _ => {}
                },
            }
            i += 1;
        }
        PhoneticWord::new(lower, out.into_iter().map(Phoneme::new).collect(), stress)
    }
}

impl Phonemizer for PortugueseG2p {
    fn phonemize(&self, text: &str) -> PhoneticUtterance {
        let mut words: Vec<PhoneticWord> = Vec::new();
        let mut current = String::new();
        let flush = |current: &mut String, words: &mut Vec<PhoneticWord>| {
            if !current.is_empty() {
                let w = self.word(current);
                if !w.phonemes.is_empty() {
                    words.push(w);
                }
                current.clear();
            }
        };
        for ch in text.chars() {
            if ch.is_alphabetic() {
                current.push(ch);
            } else {
                flush(&mut current, &mut words);
                if matches!(
                    ch,
                    '.' | ',' | ';' | ':' | '!' | '?' | '…' | '—' | '(' | ')'
                ) {
                    if let Some(last) = words.last_mut() {
                        last.pause_after = true;
                    }
                }
            }
        }
        flush(&mut current, &mut words);
        PhoneticUtterance::new(words)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn g(word: &str) -> (String, Option<usize>) {
        let w = PortugueseG2p.word(word);
        (w.ipa(), w.stress)
    }

    #[test]
    fn transcribes_citation_forms() {
        let cases = [
            ("leite", "lejte", Some(1)),
            ("quente", "kẽte", Some(1)),
            ("noite", "nojte", Some(1)),
            ("você", "vose", Some(3)),
            ("forte", "foɾte", Some(1)),
            ("amigos", "amigos", Some(2)),
            ("casa", "kaza", Some(1)),
            ("mesmo", "mesmo", Some(1)),
            ("rato", "xato", Some(1)),
            ("carro", "kaxo", Some(1)),
            ("caro", "kaɾo", Some(1)),
            ("honra", "õxa", Some(0)),
            ("brasil", "bɾazil", Some(4)),
            ("coração", "koɾasãw", Some(5)),
            ("mulher", "muʎeɾ", Some(3)),
            ("também", "tãbẽj", Some(3)),
            ("bom", "bõ", Some(1)),
            ("muito", "mujto", Some(1)),
            ("dia", "dia", Some(1)),
            ("queijo", "kejʒo", Some(1)),
            ("guerra", "gexa", Some(1)),
            ("água", "agwa", Some(0)),
            ("exame", "ezame", Some(2)),
            ("nascer", "naseɾ", Some(3)),
            ("hoje", "oʒe", Some(0)),
            ("pão", "pãw", Some(1)),
            ("mãe", "mãj", Some(1)),
            ("lindo", "lĩdo", Some(1)),
            ("falam", "falãw", Some(1)),
            ("chuva", "ʃuva", Some(1)),
            ("filho", "fiʎo", Some(1)),
            ("sonho", "soɲo", Some(1)),
            ("verdade", "veɾdade", Some(4)),
            ("rainha", "xaiɲa", Some(2)),
        ];
        for (word, ipa, stress) in cases {
            assert_eq!(g(word), (ipa.to_string(), stress), "{word}");
        }
    }

    #[test]
    fn clitics_are_unstressed() {
        assert_eq!(g("de").1, None);
        assert_eq!(g("os").1, None);
    }

    #[test]
    fn phonemizes_sentences_with_pauses() {
        let u = PortugueseG2p.phonemize("Bom dia, você é forte!");
        assert_eq!(u.ipa(), "bõ dia vose ɛ foɾte");
        assert!(u.words[1].pause_after);
        assert!(!u.words[0].pause_after);
    }
}
