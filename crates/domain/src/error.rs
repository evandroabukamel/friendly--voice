use thiserror::Error;

/// Business rule violations.
#[derive(Debug, Clone, Error, PartialEq)]
pub enum DomainError {
    #[error("valor inválido para '{field}': {reason}")]
    InvalidValue { field: &'static str, reason: String },
    #[error("é obrigatório registrar o consentimento de '{0}' para clonar a voz")]
    ConsentRequired(String),
    #[error("amostra de áudio rejeitada: {0}")]
    SampleRejected(String),
    #[error("material de voz insuficiente: {0}")]
    InsufficientMaterial(String),
    #[error("agendamento inválido: {0}")]
    InvalidSchedule(String),
}

impl DomainError {
    pub fn invalid(field: &'static str, reason: impl Into<String>) -> Self {
        Self::InvalidValue {
            field,
            reason: reason.into(),
        }
    }
}
