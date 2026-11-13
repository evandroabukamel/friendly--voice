//! # Friendly Voice — Domain
//!
//! Innermost layer of the Clean Architecture. It contains only pure business
//! rules: no dependency on I/O, databases, network or frameworks.
//!
//! * [`speaker`]   — the person whose voice is used (consent is mandatory).
//! * [`language`]  — the language of the recordings and of the messages.
//! * [`audio`]     — audio clip and sample quality policy.
//! * [`voice`]     — samples, voice gender, voice profile, prosody and speech script.
//! * [`message`]   — motivational messages.
//! * [`schedule`]  — user-defined scheduling rules.
//! * [`tone`]      — tone of voice chosen for each message.

pub mod audio;
pub mod error;
pub mod ids;
pub mod language;
pub mod message;
pub mod schedule;
pub mod speaker;
pub mod tone;
pub mod voice;

pub use error::DomainError;
