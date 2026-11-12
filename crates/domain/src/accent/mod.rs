//! # Accent model
//!
//! The accent is modelled as an **ordered** set of phonological rules that
//! turn the citation form (the "dictionary" pronunciation) into the form
//! the person actually produces. Three phenomena are handled explicitly:
//!
//! 1. **Pronunciation of letters/sounds** ([`RulePhenomenon::Segmental`]) — e.g. `t`
//!    before `i` becomes `tʃ` ("tia" → "tʃia"); coda `s` becomes `ʃ` (Rio de Janeiro);
//!    coda `r` becomes `h`, `ɻ` (caipira)...
//! 2. **Word endings** ([`RulePhenomenon::WordEnding`]) — e.g. unstressed final
//!    `e`/`o` become `i`/`u` ("leite" → "leiti"), final `r` is dropped
//!    ("falar" → "falá").
//! 3. **Word junctions** ([`JunctureRule`]) — external sandhi: "os amigos"
//!    → "uz amigus", "casa amarela" → "casamarela", "mar azul" → "maɾazul".
//!
//! Rules are applied in order, so one can feed another
//! ("noite": `e → i` creates the context for `t → tʃ`).
//!
//! * [`AccentAnalyzer`] learns the profile from pairs
//!   (citation pronunciation, phonemes actually heard in the samples).
//! * [`AccentRenderer`] applies the learned profile to any new text.

mod alignment;
mod analyzer;
mod catalog;
mod derivation;
mod renderer;
mod rule;

pub use alignment::align;
pub use analyzer::{
    AccentAnalysis, AccentAnalyzer, AnalysisEvidence, AnalyzerConfig, RuleEvidence,
};
pub use catalog::{AccentPreset, RuleCatalog};
pub use derivation::UtteranceDerivation;
pub use renderer::AccentRenderer;
pub use rule::{
    AccentProfile, JunctureEffect, JunctureRule, PhoneContext, Position, PronunciationRule,
    RulePhenomenon,
};

#[cfg(test)]
mod tests;
