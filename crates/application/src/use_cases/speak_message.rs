use crate::error::{AppError, AppResult};
use crate::ports::{
    Clock, Delivery, MessageDelivery, MessageSource, SpeakerRepository, SpeechSynthesizer,
    VoiceProfileRepository,
};
use friendly_voice_domain::{
    ids::SpeakerId,
    message::{MessageContent, MotivationalMessage},
    tone::{ToneStyle, VoiceTone},
    voice::SpeechScript,
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct SpeakRequest {
    pub speaker_id: SpeakerId,
    pub content: MessageContent,
    /// Name of the listener (personalizes the message).
    pub listener: Option<String>,
    /// Tone of voice of this message. `None` = initial tone suggested by the theme
    /// (e.g. "calma" → calmo, "celebracao" → animado; free text → natural).
    pub tone: Option<VoiceTone>,
}

#[derive(Debug, Clone)]
pub struct SpokenMessage {
    pub message: MotivationalMessage,
    pub script: SpeechScript,
    pub receipt: String,
}

/// US-05: ask for a message right now (also used by the schedules).
pub struct SpeakMessage {
    speakers: Arc<dyn SpeakerRepository>,
    profiles: Arc<dyn VoiceProfileRepository>,
    messages: Arc<dyn MessageSource>,
    synthesizer: Arc<dyn SpeechSynthesizer>,
    delivery: Arc<dyn MessageDelivery>,
    clock: Arc<dyn Clock>,
}

impl SpeakMessage {
    pub fn new(
        speakers: Arc<dyn SpeakerRepository>,
        profiles: Arc<dyn VoiceProfileRepository>,
        messages: Arc<dyn MessageSource>,
        synthesizer: Arc<dyn SpeechSynthesizer>,
        delivery: Arc<dyn MessageDelivery>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            speakers,
            profiles,
            messages,
            synthesizer,
            delivery,
            clock,
        }
    }

    pub fn execute(&self, req: SpeakRequest) -> AppResult<SpokenMessage> {
        let speaker = self
            .speakers
            .find(&req.speaker_id)?
            .ok_or_else(|| AppError::not_found("person", &req.speaker_id))?;
        let voice = self.profiles.find(&req.speaker_id)?.ok_or_else(|| {
            AppError::Precondition(format!(
                "the voice of {} has not been generated yet; add samples and run `voice generate`",
                speaker.name
            ))
        })?;

        let message = match req.content {
            // The theme catalog is written in Portuguese.
            MessageContent::Category(_) if !voice.language.is_portuguese() => {
                return Err(AppError::Precondition(format!(
                    "themes are only available in Portuguese and {} speaks {}; pass the text to say",
                    speaker.name,
                    voice.language.name()
                )))
            }
            MessageContent::Category(c) => {
                self.messages.pick(c, &speaker, req.listener.as_deref())?
            }
            MessageContent::FixedText(t) => MotivationalMessage::new(&t, None)?,
        };

        let tone = req
            .tone
            .unwrap_or_else(|| VoiceTone::of(ToneStyle::default_for(message.category())));

        let script = SpeechScript {
            text: message.text().to_string(),
            language: voice.language.clone(),
            prosody: tone.apply(voice.prosody),
            tone,
        };
        let audio = self.synthesizer.synthesize(&voice, &script)?;
        let receipt = self.delivery.deliver(&Delivery {
            speaker: &speaker,
            message: &message,
            script: &script,
            audio: &audio,
            at: self.clock.now(),
        })?;
        Ok(SpokenMessage {
            message,
            script,
            receipt,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fakes::Fakes;
    use friendly_voice_domain::message::MessageCategory;

    #[test]
    fn speaks_in_the_voice_language_and_delivers() {
        let f = Fakes::new();
        let spk = f.speaker();
        f.voice_profile(&spk);
        let uc = f.speak_message();
        let out = uc
            .execute(SpeakRequest {
                speaker_id: spk.id.clone(),
                content: MessageContent::FixedText("leite quente".into()),
                listener: None,
                tone: None,
            })
            .unwrap();
        assert_eq!(out.script.text, "leite quente");
        assert_eq!(out.script.language.tag(), "pt-BR");
        assert_eq!(f.engine.spoken()[0].text, "leite quente");
        assert_eq!(out.script.tone, VoiceTone::of(ToneStyle::Natural));
        assert_eq!(f.delivery.delivered(), vec!["leite quente".to_string()]);

        let out = uc
            .execute(SpeakRequest {
                speaker_id: spk.id,
                content: MessageContent::Category(MessageCategory::Motivation),
                listener: Some("Ana".into()),
                tone: None,
            })
            .unwrap();
        assert!(out.message.text().contains("Ana"));
        // initial tone suggested by the theme
        assert_eq!(out.script.tone.style, ToneStyle::Cheerful);
    }

    #[test]
    fn chosen_tone_colors_the_natural_prosody() {
        let f = Fakes::new();
        let spk = f.speaker();
        f.voice_profile(&spk);
        let natural = f.profiles.find(&spk.id).unwrap().unwrap().prosody;
        let tone = VoiceTone::new(ToneStyle::Calm, -1.0).unwrap();
        let out = f
            .speak_message()
            .execute(SpeakRequest {
                speaker_id: spk.id,
                content: MessageContent::Category(MessageCategory::Celebration),
                listener: None,
                tone: Some(tone),
            })
            .unwrap();
        assert_eq!(
            out.script.tone, tone,
            "the user's choice wins over the theme's tone"
        );
        assert_eq!(out.script.prosody, tone.apply(natural));
        assert!(out.script.prosody.mean_pitch_hz < natural.mean_pitch_hz);
    }

    #[test]
    fn requires_generated_voice() {
        let f = Fakes::new();
        let spk = f.speaker();
        let err = f
            .speak_message()
            .execute(SpeakRequest {
                speaker_id: spk.id,
                content: MessageContent::FixedText("oi".into()),
                listener: None,
                tone: None,
            })
            .unwrap_err();
        assert!(matches!(err, AppError::Precondition(_)));
    }

    #[test]
    fn other_languages_speak_free_text_only() {
        let f = Fakes::new();
        let spk = f.speaker_speaking("en".parse().unwrap());
        f.voice_profile(&spk);
        let request = |content| SpeakRequest {
            speaker_id: spk.id.clone(),
            content,
            listener: None,
            tone: None,
        };
        let err = f
            .speak_message()
            .execute(request(MessageContent::Category(MessageCategory::Calm)))
            .unwrap_err();
        assert!(matches!(err, AppError::Precondition(_)), "{err}");
        let out = f
            .speak_message()
            .execute(request(MessageContent::FixedText("You can do it!".into())))
            .unwrap();
        assert_eq!(out.script.language.code(), "en");
    }
}
