//! # Friendly Voice — Application
//!
//! Orchestrates the domain through **use cases**. It depends only on
//! abstractions ([`ports`]) — the concrete implementations (WAV, JSON, HTTP...)
//! live in the infrastructure layer and are injected by the composition root.
//! This applies the Dependency Inversion Principle (DIP).

pub mod error;
pub mod ports;
pub mod use_cases;

pub use error::AppError;

#[cfg(test)]
pub(crate) mod fakes;
