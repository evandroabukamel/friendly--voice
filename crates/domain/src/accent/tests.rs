use super::*;
use crate::phonetics::{parse_ipa, PhoneticUtterance, PhoneticWord};

/// Builds a word from IPA; `stress` = index of the stressed phoneme.
fn w(orth: &str, ipa: &str, stress: Option<usize>) -> PhoneticWord {
    PhoneticWord::new(orth, parse_ipa(ipa), stress)
}

fn utt(words: Vec<PhoneticWord>) -> PhoneticUtterance {
    PhoneticUtterance::new(words)
}

fn ev(canonical: PhoneticUtterance, observed: &str) -> AnalysisEvidence {
    AnalysisEvidence {
        canonical,
        observed: parse_ipa(observed),
    }
}

fn adopted(a: &AccentAnalysis, code: &str) -> bool {
    a.profile.rule_codes().contains(&code)
}

/// Samples from a speaker from Rio de Janeiro (carioca accent).
fn carioca_evidence() -> Vec<AnalysisEvidence> {
    vec![
        // "leite quente" → lejtʃi kẽtʃi
        ev(
            utt(vec![
                w("leite", "lejte", Some(1)),
                w("quente", "kẽte", Some(1)),
            ]),
            "lejtʃi kẽtʃi",
        ),
        // "festa mesmo" → fɛʃta meʒmu
        ev(
            utt(vec![
                w("festa", "fɛsta", Some(1)),
                w("mesmo", "mesmo", Some(1)),
            ]),
            "fɛʃta meʒmu",
        ),
        // "os amigos de verdade" → uz amigus dʒi vehdadʒi
        ev(
            utt(vec![
                w("os", "os", None),
                w("amigos", "amigos", Some(2)),
                w("de", "de", None),
                w("verdade", "veɾdade", Some(4)),
            ]),
            "uz amiguʃ dʒi vehdadʒi",
        ),
        // "porta do mar" → pɔhta du ma
        ev(
            utt(vec![
                w("porta", "pɔɾta", Some(1)),
                w("do", "do", None),
                w("mar", "maɾ", Some(1)),
            ]),
            "pɔhta du ma",
        ),
        // "dia lindo" → dʒia lĩdu
        ev(
            utt(vec![w("dia", "dia", Some(1)), w("lindo", "lĩdo", Some(1))]),
            "dʒia lĩdu",
        ),
        // "tudo certo" → tudu sɛhtu
        ev(
            utt(vec![
                w("tudo", "tudo", Some(1)),
                w("certo", "sɛɾto", Some(1)),
            ]),
            "tudu sɛhtu",
        ),
        // "mais alegria" → majz alegɾia
        ev(
            utt(vec![
                w("mais", "majs", Some(1)),
                w("alegria", "alegɾia", Some(5)),
            ]),
            "majz alegɾia",
        ),
        // "vou falar" → vow fala
        ev(
            utt(vec![w("vou", "vow", Some(1)), w("falar", "falaɾ", Some(3))]),
            "vow fala",
        ),
    ]
}

#[test]
fn learns_word_endings_letters_and_junctures() {
    let a = AccentAnalyzer::default().analyze(&carioca_evidence());
    // word endings
    assert!(adopted(&a, "final-e-raising"), "{:#?}", a.evidence);
    assert!(adopted(&a, "final-o-raising"));
    assert!(adopted(&a, "word-final-r-deletion"));
    // pronunciation of letters
    assert!(adopted(&a, "t-palatalization"));
    assert!(adopted(&a, "d-palatalization"));
    assert!(adopted(&a, "coda-s-palatalization"));
    assert!(adopted(&a, "coda-r-aspiration"));
    // word junctions
    assert!(adopted(&a, "sibilant-voicing"));
    // does not adopt what the person does not do
    assert!(!adopted(&a, "coda-r-retroflex"));
    assert!(!adopted(&a, "final-e-centralization"));
    assert_eq!(a.profile.label, "Carioca (RJ)");
}

#[test]
fn rules_feed_each_other_in_order() {
    // "noite" only palatalizes the t because the final e became i first.
    let a = AccentAnalyzer::default().analyze(&carioca_evidence());
    let out = AccentRenderer.render(&a.profile, &utt(vec![w("noite", "nojte", Some(1))]), 1.0);
    assert_eq!(out.ipa(), "nojtʃi");
}

#[test]
fn renders_new_text_with_learned_accent() {
    let a = AccentAnalyzer::default().analyze(&carioca_evidence());
    let text = utt(vec![
        w("você", "vose", Some(3)),
        w("é", "ɛ", Some(0)),
        w("forte", "fɔɾte", Some(1)),
        w("mas", "mas", Some(1)),
        w("às", "as", None),
        w("vezes", "vezes", Some(1)),
        w("esquece", "eskɛse", Some(3)),
    ]);
    let out = AccentRenderer.render(&a.profile, &text, 1.0);
    assert_eq!(out.ipa(), "vose ɛ fɔhtʃi maz aʃ veziz eʃkɛsi");
}

#[test]
fn juncture_rhotic_linking_blocks_r_deletion() {
    let mut profile = AccentProfile::default();
    let catalog = RuleCatalog::portuguese();
    let pick = |c: &str| {
        catalog
            .segment_rules
            .iter()
            .find(|r| r.code == c)
            .unwrap()
            .clone()
    };
    profile.segment_rules = vec![pick("word-final-r-deletion"), pick("coda-r-aspiration")];
    profile.juncture_rules = catalog
        .juncture_rules
        .iter()
        .filter(|r| r.code == "rhotic-linking")
        .cloned()
        .collect();
    let text = utt(vec![
        w("amar", "amaɾ", Some(2)),
        w("a", "a", None),
        w("vida", "vida", Some(1)),
    ]);
    // "amar a vida": the r is not dropped before a vowel and links as a tap
    assert_eq!(
        AccentRenderer.render(&profile, &text, 1.0).ipa(),
        "amaɾ a vida"
    );
    let text = utt(vec![w("vou", "vow", Some(1)), w("amar", "amaɾ", Some(2))]);
    assert_eq!(AccentRenderer.render(&profile, &text, 1.0).ipa(), "vow ama");
}

#[test]
fn vowel_elision_and_gliding_between_words() {
    let catalog = RuleCatalog::portuguese();
    let profile = AccentProfile {
        label: "teste".into(),
        segment_rules: vec![catalog.segment_rules[0].clone()], // final-e-raising
        juncture_rules: catalog.juncture_rules.clone(),
    };
    let text = utt(vec![
        w("casa", "kaza", Some(1)),
        w("amarela", "amaɾɛla", Some(4)),
        w("de", "de", None),
        w("outro", "owtɾo", Some(1)),
    ]);
    assert_eq!(
        AccentRenderer.render(&profile, &text, 1.0).ipa(),
        "kaz amaɾɛla dj owtɾo"
    );
}

#[test]
fn pause_blocks_juncture() {
    let catalog = RuleCatalog::portuguese();
    let profile = AccentProfile {
        label: "t".into(),
        segment_rules: vec![],
        juncture_rules: catalog.juncture_rules.clone(),
    };
    let text = utt(vec![
        w("mas", "mas", Some(1)).with_pause(true),
        w("agora", "agɔɾa", Some(2)),
    ]);
    assert_eq!(
        AccentRenderer.render(&profile, &text, 1.0).ipa(),
        "mas agɔɾa"
    );
}

#[test]
fn intensity_zero_keeps_citation_form() {
    let a = AccentAnalyzer::default().analyze(&carioca_evidence());
    let text = utt(vec![w("noite", "nojte", Some(1))]);
    assert_eq!(AccentRenderer.render(&a.profile, &text, 0.0).ipa(), "nojte");
}

#[test]
fn discovers_patterns_outside_catalog() {
    // Speaker who replaces word-initial 'v' with 'b' (bida, bento, boz, bez).
    let samples = vec![
        ev(utt(vec![w("vida", "vida", Some(1))]), "bida"),
        ev(utt(vec![w("vento", "vẽto", Some(1))]), "bẽto"),
        ev(utt(vec![w("voz", "vɔs", Some(1))]), "bɔs"),
        ev(utt(vec![w("vez", "ves", Some(1))]), "bes"),
    ];
    let a = AccentAnalyzer::default().analyze(&samples);
    assert!(
        adopted(&a, "learned:v>b@WordInitial"),
        "{:?}",
        a.profile.rule_codes()
    );
    let out = AccentRenderer.render(&a.profile, &utt(vec![w("viva", "viva", Some(1))]), 1.0);
    assert_eq!(out.ipa(), "biva");
}

#[test]
fn no_evidence_yields_neutral_profile() {
    let a = AccentAnalyzer::default().analyze(&[]);
    assert!(a.profile.segment_rules.is_empty());
    assert_eq!(a.profile.label, "Neutral");
}
