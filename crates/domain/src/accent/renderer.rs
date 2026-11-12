//! Applies an accent profile to new text.
use super::derivation::UtteranceDerivation;
use super::rule::AccentProfile;
use crate::phonetics::PhoneticUtterance;

/// Accent renderer.
///
/// `intensity` (0..=1) controls how strong the accent is: at 1.0 every rule
/// of the profile is applied; lower values apply only the rules the person
/// produces most consistently (confidence > 1 - intensity).
#[derive(Debug, Clone, Copy, Default)]
pub struct AccentRenderer;

impl AccentRenderer {
    pub fn render(
        &self,
        profile: &AccentProfile,
        utterance: &PhoneticUtterance,
        intensity: f32,
    ) -> PhoneticUtterance {
        let threshold = 1.0 - intensity.clamp(0.0, 1.0);
        let mut d = UtteranceDerivation::new(utterance);
        for rule in profile
            .segment_rules
            .iter()
            .filter(|r| r.confidence > threshold)
        {
            d.apply_rule(rule);
        }
        for rule in profile
            .juncture_rules
            .iter()
            .filter(|r| r.confidence > threshold)
        {
            d.apply_juncture(rule);
        }
        d.to_utterance()
    }
}
