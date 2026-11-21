//! Messages spoken right now (theme or free text, with a tone of voice).
use super::failure::AppFailure;
use super::session::{non_empty, Session};
use friendly_voice_application::use_cases::SpeakRequest;
use friendly_voice_domain::{
    ids::SpeakerId,
    message::{MessageCategory, MessageContent},
    tone::{ToneStyle, VoiceTone},
};

#[derive(Debug, Clone, PartialEq)]
pub struct SpokenMessage {
    pub text: String,
    /// Tone actually used, e.g. `calm` or `whisper (-1 st)`.
    pub tone: String,
    pub language: String,
    /// WAV file to play.
    pub audio_path: String,
}

/// Content of a message: free text wins over the theme; default theme is
/// motivation.
pub(crate) fn content(
    theme: Option<String>,
    text: Option<String>,
) -> Result<MessageContent, AppFailure> {
    Ok(match (non_empty(text), non_empty(theme)) {
        (Some(t), _) => MessageContent::FixedText(t),
        (None, Some(c)) => MessageContent::Category(c.parse().map_err(AppFailure::new)?),
        (None, None) => MessageContent::Category(MessageCategory::Motivation),
    })
}

/// Tone chosen in the UI; no style and no shift = initial tone of the theme.
pub(crate) fn tone(
    style: Option<String>,
    pitch_shift: f32,
) -> Result<Option<VoiceTone>, AppFailure> {
    let style = non_empty(style);
    if style.is_none() && pitch_shift == 0.0 {
        return Ok(None);
    }
    let style = match style {
        Some(s) => s.parse::<ToneStyle>().map_err(AppFailure::new)?,
        None => ToneStyle::Natural,
    };
    VoiceTone::new(style, pitch_shift)
        .map(Some)
        .map_err(AppFailure::new)
}

impl Session {
    /// Speaks a message now in the person's voice and returns the WAV path.
    pub fn speak(
        &self,
        speaker_id: String,
        theme: Option<String>,
        text: Option<String>,
        tone_style: Option<String>,
        pitch_shift: f32,
        listener: Option<String>,
    ) -> Result<SpokenMessage, AppFailure> {
        let out = self
            .container
            .speak_message()
            .execute(SpeakRequest {
                speaker_id: SpeakerId::new(speaker_id),
                content: content(theme, text)?,
                listener: non_empty(listener),
                tone: tone(tone_style, pitch_shift)?,
            })
            .map_err(AppFailure::new)?;
        Ok(SpokenMessage {
            text: out.message.text().to_string(),
            tone: out.script.tone.to_string(),
            language: out.script.language.to_string(),
            audio_path: out.receipt,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::api::session::tests::local_session;
    use crate::api::session::Session;
    use crate::api::voice::tests::wav;

    fn with_voice(name: &str) -> (Session, String) {
        let s = local_session(name);
        let p = s
            .register_speaker("Vó".into(), None, Some("Vó".into()), None)
            .unwrap();
        s.add_sample(p.id.clone(), wav(6.0, 210.0), None).unwrap();
        s.add_sample(p.id.clone(), wav(6.0, 205.0), None).unwrap();
        s.build_voice(p.id.clone()).unwrap();
        (s, p.id)
    }

    #[test]
    fn speaks_a_theme_with_its_initial_tone() {
        let (s, id) = with_voice("speak-theme");
        let m = s
            .speak(
                id,
                Some("calm".into()),
                None,
                None,
                0.0,
                Some("Evandro".into()),
            )
            .unwrap();
        assert_eq!(m.tone, "calm");
        assert_eq!(m.language, "pt-BR");
        assert!(!m.text.is_empty());
        assert!(
            std::path::Path::new(&m.audio_path).exists(),
            "{}",
            m.audio_path
        );
    }

    #[test]
    fn the_users_tone_and_free_text_win() {
        let (s, id) = with_voice("speak-text");
        let m = s
            .speak(
                id.clone(),
                Some("calm".into()),
                Some("Boa noite.".into()),
                Some("whisper".into()),
                -1.0,
                None,
            )
            .unwrap();
        assert_eq!(m.text, "Boa noite.");
        assert_eq!(m.tone, "whisper (-1 st)");
        let err = s
            .speak(id, None, None, Some("angry".into()), 0.0, None)
            .unwrap_err();
        assert!(err.message.contains("angry"), "{err}");
    }

    #[test]
    fn a_voice_must_be_generated_first() {
        let s = local_session("speak-no-voice");
        let p = s
            .register_speaker("Vó".into(), None, Some("Vó".into()), None)
            .unwrap();
        let err = s.speak(p.id, None, None, None, 0.0, None).unwrap_err();
        assert!(
            err.message.contains("voice generate") || err.message.contains("not been generated"),
            "{err}"
        );
    }
}
