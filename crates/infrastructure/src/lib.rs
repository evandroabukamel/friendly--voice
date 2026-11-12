//! # Friendly Voice — Infrastructure
//!
//! Adapters that implement the application layer's ports. No business rule
//! lives here: only translation between the outside world (files, audio,
//! HTTP, system) and the domain model.
pub mod audio;
pub mod delivery;
pub mod messages;
pub mod persistence;
pub mod phonetics;
pub mod system;
pub mod voice;
