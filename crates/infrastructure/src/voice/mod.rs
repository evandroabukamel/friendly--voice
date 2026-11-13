//! Voice engines (timbre). Neural cloning is delegated to an external
//! server (feature `http-engine`, reference server in `engine/`); locally there
//! is a prosody preview synthesizer, useful for development and tests.
#[cfg(feature = "http-engine")]
mod http;
mod sketch;

#[cfg(feature = "http-engine")]
pub use http::{HttpTranscriber, HttpVoiceEngine};
pub use sketch::{LocalVoiceBank, ProsodySketchSynthesizer};
