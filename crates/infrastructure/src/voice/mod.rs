//! Motores de voz (timbre). A clonagem neural é delegada a um servidor
//! externo (feature `http-engine`); localmente há um sintetizador de
//! pré-visualização de prosódia, útil para desenvolvimento e testes.
#[cfg(feature = "http-engine")]
mod http;
mod sketch;
mod ssml;

#[cfg(feature = "http-engine")]
pub use http::{HttpPhoneRecognizer, HttpTranscriber, HttpVoiceEngine};
pub use sketch::{LocalVoiceBank, ProsodySketchSynthesizer};
pub use ssml::to_ssml;
