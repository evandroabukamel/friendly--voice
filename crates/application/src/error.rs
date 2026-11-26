use friendly_voice_domain::DomainError;
use thiserror::Error;

use crate::ports::{DecodeError, PortError};

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Domain(#[from] DomainError),
    #[error("external dependency failed: {0}")]
    Port(#[from] PortError),
    #[error("{entity} '{id}' not found")]
    NotFound { entity: &'static str, id: String },
    #[error("{0}")]
    Precondition(String),
}

/// An undecodable file is a rejected sample; only a decoder failure is a
/// port failure.
impl From<DecodeError> for AppError {
    fn from(e: DecodeError) -> Self {
        match e {
            DecodeError::Rejected(problem) => DomainError::SampleRejected(problem).into(),
            DecodeError::Port(e) => e.into(),
        }
    }
}

impl AppError {
    pub fn not_found(entity: &'static str, id: impl ToString) -> Self {
        Self::NotFound {
            entity,
            id: id.to_string(),
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;
