use thiserror::Error;

/// Business rule violations.
#[derive(Debug, Clone, Error, PartialEq)]
pub enum DomainError {
    #[error("invalid value for '{field}': {reason}")]
    InvalidValue { field: &'static str, reason: String },
    #[error("the consent of '{0}' must be recorded before cloning the voice")]
    ConsentRequired(String),
    #[error("audio sample rejected: {0}")]
    SampleRejected(String),
    #[error("not enough voice material: {0}")]
    InsufficientMaterial(String),
    #[error("invalid schedule: {0}")]
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
