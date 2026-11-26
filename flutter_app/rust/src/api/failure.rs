//! The single error type crossing the bridge: Dart receives it as an
//! exception carrying a clear, actionable message for the user, and the
//! kind of failure so the app can explain it in the user's language.
use friendly_voice_application::AppError;
use friendly_voice_domain::{audio::SampleProblem, DomainError};
use std::fmt;

/// What went wrong, for the failures the app explains in its own words.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FailureKind {
    /// Anything else: show `message`.
    Other,
    /// The file is not audio the app reads: unknown format, unsupported
    /// codec, or damaged.
    AudioFormat,
    /// The file holds no audio track.
    NoAudioTrack,
    /// The recording is longer than the maximum.
    AudioTooLong,
    /// The recording is shorter than the minimum.
    AudioTooShort,
}

impl From<SampleProblem> for FailureKind {
    fn from(problem: SampleProblem) -> Self {
        match problem {
            SampleProblem::UnsupportedFormat
            | SampleProblem::UnsupportedCodec
            | SampleProblem::Unreadable => Self::AudioFormat,
            SampleProblem::NoAudioTrack => Self::NoAudioTrack,
            SampleProblem::TooLong { .. } => Self::AudioTooLong,
            SampleProblem::TooShort { .. } => Self::AudioTooShort,
            SampleProblem::LowSampleRate { .. }
            | SampleProblem::Clipped
            | SampleProblem::NearlySilent => Self::Other,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AppFailure {
    pub kind: FailureKind,
    /// English text, shown when the app has no words of its own for `kind`.
    pub message: String,
}

impl AppFailure {
    /// A failure of kind [`FailureKind::Other`].
    pub(crate) fn new(message: impl fmt::Display) -> Self {
        Self {
            kind: FailureKind::Other,
            message: message.to_string(),
        }
    }
}

impl From<AppError> for AppFailure {
    fn from(e: AppError) -> Self {
        let kind = match &e {
            AppError::Domain(DomainError::SampleRejected(problem)) => (*problem).into(),
            _ => FailureKind::Other,
        };
        Self {
            kind,
            message: e.to_string(),
        }
    }
}

impl fmt::Display for AppFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}
