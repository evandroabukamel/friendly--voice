//! # Friendly Voice — Aplicação
//!
//! Orquestra o domínio através de **casos de uso**. Depende apenas de
//! abstrações ([`ports`]) — as implementações concretas (WAV, JSON, HTTP...)
//! vivem na infraestrutura e são injetadas pela composition root (CLI).
//! Isso aplica o Princípio da Inversão de Dependência (DIP).

pub mod error;
pub mod ports;
pub mod use_cases;

pub use error::AppError;

#[cfg(test)]
pub(crate) mod fakes;
