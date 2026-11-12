//! # Modelo de sotaque
//!
//! O sotaque é modelado como um conjunto **ordenado** de regras fonológicas que
//! transformam a forma de citação (pronúncia "de dicionário") na forma
//! realizada pela pessoa. Três fenômenos são tratados explicitamente:
//!
//! 1. **Pronúncia de letras/sons** ([`RulePhenomenon::Segmental`]) — ex.: `t`
//!    antes de `i` vira `tʃ` ("tia" → "tʃia"); `s` em coda vira `ʃ` (carioca);
//!    `r` em coda vira `h`, `ɻ` (caipira)...
//! 2. **Término de palavras** ([`RulePhenomenon::WordEnding`]) — ex.: `e`/`o`
//!    átonos finais viram `i`/`u` ("leite" → "leiti"), `r` final cai
//!    ("falar" → "falá").
//! 3. **Junção de palavras** ([`JunctureRule`]) — sândi externo: "os amigos"
//!    → "uz amigus", "casa amarela" → "casamarela", "mar azul" → "maɾazul".
//!
//! As regras são aplicadas em ordem, permitindo que uma alimente a outra
//! ("noite": `e → i` cria o contexto para `t → tʃ`).
//!
//! * [`AccentAnalyzer`] aprende o perfil a partir de pares
//!   (pronúncia canônica, fonemas efetivamente ouvidos nas amostras).
//! * [`AccentRenderer`] aplica o perfil aprendido a qualquer texto novo.

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
