//! The single error type crossing the bridge: Dart receives it as an
//! exception carrying a clear, actionable message for the user.
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct AppFailure {
    pub message: String,
}

impl AppFailure {
    pub(crate) fn new(message: impl fmt::Display) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for AppFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}
