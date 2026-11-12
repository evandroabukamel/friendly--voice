//! Tom de voz escolhido para cada mensagem.
//!
//! O tom é um ajuste **sobre** a prosódia natural da pessoa (aprendida das
//! amostras): não substitui o jeito dela de falar, apenas o colore — mais
//! carinhoso, mais animado, mais calmo... O usuário pode ainda deslocar a
//! altura (o "tom" no sentido musical) em semitons.
use crate::message::MessageCategory;
use crate::voice::ProsodyProfile;
use crate::DomainError;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Estilos de tom disponíveis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum ToneStyle {
    /// Como a pessoa fala normalmente.
    #[default]
    Natural,
    /// Mais suave e acolhedor.
    Warm,
    /// Mais alegre, rápido e melódico.
    Cheerful,
    /// Lento, grave e estável.
    Calm,
    /// Seguro e enfático.
    Firm,
    /// Baixinho, como um segredo.
    Whisper,
}

/// Quanto o tom altera a prosódia natural.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ToneAdjustment {
    /// Deslocamento da altura média, em semitons.
    pub pitch_semitones: f32,
    /// Multiplicador da velocidade de fala.
    pub rate_factor: f32,
    /// Multiplicador da variação melódica (0 = monótono).
    pub expressiveness: f32,
    /// Ganho de volume em dB.
    pub volume_db: f32,
    /// Voz soprosa (sem vibração das pregas vocais).
    pub breathy: bool,
}

impl ToneStyle {
    pub const ALL: [ToneStyle; 6] = [
        Self::Natural,
        Self::Warm,
        Self::Cheerful,
        Self::Calm,
        Self::Firm,
        Self::Whisper,
    ];

    pub fn slug(self) -> &'static str {
        match self {
            Self::Natural => "natural",
            Self::Warm => "carinhoso",
            Self::Cheerful => "animado",
            Self::Calm => "calmo",
            Self::Firm => "firme",
            Self::Whisper => "sussurrado",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Natural => "como a pessoa fala normalmente",
            Self::Warm => "suave e acolhedor",
            Self::Cheerful => "alegre, um pouco mais rápido e melódico",
            Self::Calm => "lento, mais grave e estável",
            Self::Firm => "seguro e enfático",
            Self::Whisper => "baixinho, como quem conta um segredo",
        }
    }

    pub fn adjustment(self) -> ToneAdjustment {
        let (pitch_semitones, rate_factor, expressiveness, volume_db, breathy) = match self {
            Self::Natural => (0.0, 1.0, 1.0, 0.0, false),
            Self::Warm => (-0.5, 0.92, 1.15, -2.0, false),
            Self::Cheerful => (2.0, 1.1, 1.5, 2.0, false),
            Self::Calm => (-1.5, 0.82, 0.6, -3.0, false),
            Self::Firm => (-1.0, 0.95, 0.8, 3.0, false),
            Self::Whisper => (0.0, 0.88, 0.5, -10.0, true),
        };
        ToneAdjustment {
            pitch_semitones,
            rate_factor,
            expressiveness,
            volume_db,
            breathy,
        }
    }

    /// Tom inicial sugerido para o tema da mensagem (o usuário pode trocar).
    pub fn default_for(category: Option<MessageCategory>) -> Self {
        match category {
            None => Self::Natural,
            Some(MessageCategory::Motivation) => Self::Cheerful,
            Some(MessageCategory::Confidence) => Self::Firm,
            Some(MessageCategory::Gratitude) => Self::Warm,
            Some(MessageCategory::Calm) => Self::Calm,
            Some(MessageCategory::Celebration) => Self::Cheerful,
            Some(MessageCategory::GoodMorning) => Self::Warm,
        }
    }
}

impl fmt::Display for ToneStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.slug())
    }
}

impl FromStr for ToneStyle {
    type Err = DomainError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_lowercase();
        Self::ALL
            .into_iter()
            .find(|t| t.slug() == s)
            .ok_or_else(|| {
                DomainError::invalid(
                    "tone",
                    format!(
                        "tom '{s}' desconhecido; use: {}",
                        Self::ALL.map(|t| t.slug()).join(", ")
                    ),
                )
            })
    }
}

/// Maior deslocamento de altura aceito (em semitons) — além disso a voz
/// deixa de soar como a pessoa.
pub const MAX_PITCH_SHIFT_SEMITONES: f32 = 6.0;

/// Tom escolhido para uma mensagem: estilo + ajuste fino de altura.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct VoiceTone {
    pub style: ToneStyle,
    /// Ajuste extra de altura escolhido pelo usuário (semitons).
    #[serde(default)]
    pub pitch_shift_semitones: f32,
}

impl VoiceTone {
    pub fn new(style: ToneStyle, pitch_shift_semitones: f32) -> Result<Self, DomainError> {
        if !pitch_shift_semitones.is_finite()
            || pitch_shift_semitones.abs() > MAX_PITCH_SHIFT_SEMITONES
        {
            return Err(DomainError::invalid(
                "pitch_shift",
                format!("o ajuste de tom deve ficar entre -{MAX_PITCH_SHIFT_SEMITONES} e +{MAX_PITCH_SHIFT_SEMITONES} semitons"),
            ));
        }
        Ok(Self {
            style,
            pitch_shift_semitones,
        })
    }

    pub fn of(style: ToneStyle) -> Self {
        Self {
            style,
            pitch_shift_semitones: 0.0,
        }
    }

    /// Ajuste total (estilo + deslocamento do usuário).
    pub fn adjustment(&self) -> ToneAdjustment {
        let mut a = self.style.adjustment();
        a.pitch_semitones += self.pitch_shift_semitones;
        a
    }

    /// Aplica o tom à prosódia natural da pessoa.
    pub fn apply(&self, natural: ProsodyProfile) -> ProsodyProfile {
        let a = self.adjustment();
        let pitch_factor = 2f32.powf(a.pitch_semitones / 12.0);
        ProsodyProfile {
            mean_pitch_hz: natural.mean_pitch_hz * pitch_factor,
            pitch_spread_hz: natural.pitch_spread_hz * pitch_factor * a.expressiveness,
            syllables_per_second: natural.syllables_per_second * a.rate_factor,
            pause_ratio: natural.pause_ratio,
        }
    }
}

impl fmt::Display for VoiceTone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.style)?;
        if self.pitch_shift_semitones != 0.0 {
            write!(f, " ({:+} st)", self.pitch_shift_semitones)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn natural() -> ProsodyProfile {
        ProsodyProfile {
            mean_pitch_hz: 200.0,
            pitch_spread_hz: 40.0,
            syllables_per_second: 5.0,
            pause_ratio: 0.2,
        }
    }

    #[test]
    fn natural_tone_keeps_prosody() {
        assert_eq!(VoiceTone::default().apply(natural()), natural());
    }

    #[test]
    fn tones_color_the_natural_prosody() {
        let calm = VoiceTone::of(ToneStyle::Calm).apply(natural());
        let cheerful = VoiceTone::of(ToneStyle::Cheerful).apply(natural());
        assert!(calm.mean_pitch_hz < 200.0 && cheerful.mean_pitch_hz > 200.0);
        assert!(calm.syllables_per_second < 5.0 && cheerful.syllables_per_second > 5.0);
        assert!(calm.pitch_spread_hz < cheerful.pitch_spread_hz);
    }

    #[test]
    fn pitch_shift_is_in_semitones_and_bounded() {
        let up_octave_half = VoiceTone::new(ToneStyle::Natural, 6.0)
            .unwrap()
            .apply(natural());
        assert!((up_octave_half.mean_pitch_hz - 200.0 * 2f32.sqrt()).abs() < 0.01);
        assert!(VoiceTone::new(ToneStyle::Natural, 6.5).is_err());
        assert!(VoiceTone::new(ToneStyle::Natural, f32::NAN).is_err());
    }

    #[test]
    fn parses_and_defaults_by_category() {
        assert_eq!("Carinhoso".parse::<ToneStyle>().unwrap(), ToneStyle::Warm);
        assert!("bravo".parse::<ToneStyle>().is_err());
        assert_eq!(
            ToneStyle::default_for(Some(MessageCategory::Calm)),
            ToneStyle::Calm
        );
        assert_eq!(ToneStyle::default_for(None), ToneStyle::Natural);
    }
}
