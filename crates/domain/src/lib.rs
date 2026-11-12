//! # Friendly Voice — Domain
//!
//! Innermost layer of the Clean Architecture. It contains only pure business
//! rules: no dependency on I/O, databases, network or frameworks.
//!
//! * [`speaker`]   — the person whose voice is used (consent is mandatory).
//! * [`audio`]     — audio clip and sample quality policy.
//! * [`phonetics`] — phonemes, words and phonetic utterances (IPA).
//! * [`accent`]    — accent model: letter pronunciation, word endings and
//!   word junctions; analyzer and renderer.
//! * [`voice`]     — samples, voice profile, prosody and speech script.
//! * [`message`]   — motivational messages.
//! * [`schedule`]  — user-defined scheduling rules.
//! * [`tone`]      — tone of voice chosen for each message.

pub mod accent;
pub mod audio;
pub mod error;
pub mod ids;
pub mod message;
pub mod phonetics;
pub mod schedule;
pub mod speaker;
pub mod tone;
pub mod voice;

pub use error::DomainError;
