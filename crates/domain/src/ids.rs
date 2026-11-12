//! Identificadores fortemente tipados (evitam trocar um id por outro).
use serde::{Deserialize, Serialize};
use std::fmt;

macro_rules! id_type {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }
    };
}

id_type!(
    /// Identifica a pessoa cuja voz é usada.
    SpeakerId
);
id_type!(
    /// Identifica uma amostra de áudio enviada.
    SampleId
);
id_type!(
    /// Identifica um agendamento.
    ScheduleId
);
