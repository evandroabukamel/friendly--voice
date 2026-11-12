//! Learning the accent from the voice samples.
use super::alignment::align;
use super::catalog::RuleCatalog;
use super::derivation::{Site, UtteranceDerivation};
use super::rule::{AccentProfile, PhoneContext, Position, PronunciationRule, RulePhenomenon};
use crate::phonetics::{PhoneMatcher, Phoneme, PhoneticUtterance};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

/// One sample: what was said (citation form) and what was heard (phonemes).
#[derive(Debug, Clone, PartialEq)]
pub struct AnalysisEvidence {
    pub canonical: PhoneticUtterance,
    pub observed: Vec<Phoneme>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AnalyzerConfig {
    /// Minimum number of opportunities to judge a rule.
    pub min_opportunities: usize,
    /// Minimum realization rate to adopt the rule into the profile.
    pub adoption_threshold: f32,
    /// Discover patterns outside the catalog.
    pub discover_new_rules: bool,
    pub min_discovery_occurrences: usize,
}

impl Default for AnalyzerConfig {
    fn default() -> Self {
        Self {
            min_opportunities: 2,
            adoption_threshold: 0.5,
            discover_new_rules: true,
            min_discovery_occurrences: 3,
        }
    }
}

/// Statistics of one phenomenon across the samples.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuleEvidence {
    pub code: String,
    pub description: String,
    pub phenomenon: RulePhenomenon,
    pub opportunities: usize,
    pub realizations: usize,
    pub adopted: bool,
}

impl RuleEvidence {
    pub fn rate(&self) -> f32 {
        if self.opportunities == 0 {
            0.0
        } else {
            self.realizations as f32 / self.opportunities as f32
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AccentAnalysis {
    pub profile: AccentProfile,
    pub evidence: Vec<RuleEvidence>,
    pub closest_preset: Option<(String, f32)>,
}

struct Item {
    derivation: UtteranceDerivation,
    realized: HashMap<Site, Vec<Phoneme>>,
}

/// Domain service that infers a person's [`AccentProfile`].
#[derive(Debug, Clone)]
pub struct AccentAnalyzer {
    catalog: RuleCatalog,
    config: AnalyzerConfig,
}

impl Default for AccentAnalyzer {
    fn default() -> Self {
        Self::new(RuleCatalog::portuguese(), AnalyzerConfig::default())
    }
}

impl AccentAnalyzer {
    pub fn new(catalog: RuleCatalog, config: AnalyzerConfig) -> Self {
        Self { catalog, config }
    }

    pub fn analyze(&self, evidence: &[AnalysisEvidence]) -> AccentAnalysis {
        let mut items: Vec<Item> = evidence.iter().map(Self::prepare).collect();
        let mut profile = AccentProfile::default();
        let mut stats = Vec::new();

        for rule in &self.catalog.segment_rules {
            let (mut opp, mut real) = (0, 0);
            for item in &mut items {
                for site in item.derivation.rule_sites(rule) {
                    let realized = &item.realized[&site];
                    if *realized == rule.replacement {
                        opp += 1;
                        real += 1;
                        item.derivation.set_surface(site, rule.replacement.clone());
                    } else if self.explained_by_juncture(item, site) {
                        // The juncture (e.g. "os amigos" → z) masks the rule:
                        // it does not count, but the underlying form is assumed.
                        if !rule.replacement.is_empty() {
                            item.derivation.set_surface(site, rule.replacement.clone());
                        }
                    } else {
                        opp += 1;
                    }
                }
            }
            let ev = self.evidence(&rule.code, &rule.description, rule.phenomenon, opp, real);
            if ev.adopted {
                profile
                    .segment_rules
                    .push(rule.clone().with_confidence(ev.rate()));
            }
            stats.push(ev);
        }

        for rule in &self.catalog.juncture_rules {
            let (mut opp, mut real) = (0, 0);
            for item in &mut items {
                for site in item.derivation.juncture_sites(rule) {
                    opp += 1;
                    let mut expected = item.derivation.surface(site).to_vec();
                    expected.pop();
                    expected.extend(rule.expected_left());
                    if item.realized[&site] == expected {
                        real += 1;
                        item.derivation.set_surface(site, expected);
                    }
                }
            }
            let ev = self.evidence(
                &rule.code,
                &rule.description,
                RulePhenomenon::Juncture,
                opp,
                real,
            );
            if ev.adopted {
                profile
                    .juncture_rules
                    .push(rule.clone().with_confidence(ev.rate()));
            }
            stats.push(ev);
        }

        let codes = profile.rule_codes();
        let closest = RuleCatalog::closest_preset(&codes).map(|(n, s)| (n.to_string(), s));
        profile.label = match &closest {
            Some((name, score)) if *score >= 0.5 => name.clone(),
            _ if codes.is_empty() => "Neutral".into(),
            _ => "Custom".into(),
        };

        if self.config.discover_new_rules {
            for (rule, ev) in self.discover(&items) {
                profile.segment_rules.push(rule);
                stats.push(ev);
            }
        }

        AccentAnalysis {
            profile,
            evidence: stats,
            closest_preset: closest,
        }
    }

    fn prepare(e: &AnalysisEvidence) -> Item {
        let derivation = UtteranceDerivation::new(&e.canonical);
        let sites = derivation.sites();
        let aligned = align(&e.canonical.phonemes(), &e.observed);
        let realized = sites.into_iter().zip(aligned).collect();
        Item {
            derivation,
            realized,
        }
    }

    fn explained_by_juncture(&self, item: &Item, site: Site) -> bool {
        if !item.derivation.is_word_final_site(site) {
            return false;
        }
        let Some(next) = item.derivation.next_word_first_phone(site.0) else {
            return false;
        };
        let realized = &item.realized[&site];
        self.catalog
            .juncture_rules
            .iter()
            .any(|j| j.right.matches(next) && *realized == j.expected_left())
    }

    fn evidence(
        &self,
        code: &str,
        description: &str,
        phenomenon: RulePhenomenon,
        opportunities: usize,
        realizations: usize,
    ) -> RuleEvidence {
        let mut ev = RuleEvidence {
            code: code.into(),
            description: description.into(),
            phenomenon,
            opportunities,
            realizations,
            adopted: false,
        };
        ev.adopted = opportunities >= self.config.min_opportunities
            && ev.rate() >= self.config.adoption_threshold;
        ev
    }

    /// Discovers recurring substitutions the catalog does not explain.
    fn discover(&self, items: &[Item]) -> Vec<(PronunciationRule, RuleEvidence)> {
        type Key = (String, PositionKey);
        let mut opportunities: BTreeMap<Key, usize> = BTreeMap::new();
        let mut changes: BTreeMap<(Key, Vec<Phoneme>), usize> = BTreeMap::new();
        for item in items {
            for site in item.derivation.sites() {
                let surface = item.derivation.surface(site);
                if surface.len() != 1 || self.explained_by_juncture(item, site) {
                    continue;
                }
                let key = (
                    surface[0].symbol().to_string(),
                    Self::position_key(item, site),
                );
                *opportunities.entry(key.clone()).or_default() += 1;
                let realized = &item.realized[&site];
                if realized.as_slice() != surface {
                    *changes.entry((key, realized.clone())).or_default() += 1;
                }
            }
        }
        changes
            .into_iter()
            .filter_map(|(((from, pos), to), count)| {
                let opp = opportunities[&(from.clone(), pos)];
                let rate = count as f32 / opp as f32;
                if count < self.config.min_discovery_occurrences
                    || rate < self.config.adoption_threshold
                {
                    return None;
                }
                let to_str: String = to.iter().map(Phoneme::symbol).collect();
                let shown = if to_str.is_empty() {
                    "∅".to_string()
                } else {
                    to_str
                };
                let position = pos.position();
                let code = format!("learned:{from}>{shown}@{position:?}");
                let description = format!("learned pattern: '{from}' → '{shown}' ({position:?})");
                let rule = PronunciationRule {
                    code: code.clone(),
                    description: description.clone(),
                    phenomenon: if matches!(position, Position::WordFinal) {
                        RulePhenomenon::WordEnding
                    } else {
                        RulePhenomenon::Segmental
                    },
                    target: PhoneMatcher::Exact(Phoneme::new(from)),
                    replacement: to,
                    context: PhoneContext::at(position),
                    confidence: rate,
                };
                let ev = RuleEvidence {
                    code,
                    description,
                    phenomenon: rule.phenomenon,
                    opportunities: opp,
                    realizations: count,
                    adopted: true,
                };
                Some((rule, ev))
            })
            .collect()
    }

    fn position_key(item: &Item, site: Site) -> PositionKey {
        let d = &item.derivation;
        if d.is_word_final_site(site) {
            PositionKey::Final
        } else if d.is_word_initial_site(site) {
            PositionKey::Initial
        } else if d.is_coda_site(site) {
            PositionKey::Coda
        } else {
            PositionKey::Medial
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum PositionKey {
    Initial,
    Medial,
    Coda,
    Final,
}

impl PositionKey {
    fn position(self) -> Position {
        match self {
            Self::Initial => Position::WordInitial,
            Self::Medial => Position::Anywhere,
            Self::Coda => Position::Coda,
            Self::Final => Position::WordFinal,
        }
    }
}
