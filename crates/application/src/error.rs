use friendly_voice_domain::DomainError;
use thiserror::Error;

use crate::ports::PortError;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Domain(#[from] DomainError),
    #[error("falha em dependência externa: {0}")]
    Port(#[from] PortError),
    #[error("{entity} '{id}' não encontrado(a)")]
    NotFound { entity: &'static str, id: String },
    #[error("{0}")]
    Precondition(String),
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
