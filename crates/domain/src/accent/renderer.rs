//! Aplica um perfil de sotaque a um texto novo.
use super::derivation::UtteranceDerivation;
use super::rule::AccentProfile;
use crate::phonetics::PhoneticUtterance;

/// Renderizador de sotaque.
///
/// `intensity` (0..=1) controla quão forte é o sotaque: com 1.0 todas as
/// regras do perfil são aplicadas; valores menores aplicam apenas as regras
/// que a pessoa realiza com mais regularidade (confiança > 1 - intensidade).
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
