use crate::error::{AppError, AppResult};
use crate::ports::{
    Clock, Delivery, MessageDelivery, MessageSource, Phonemizer, SpeakerRepository,
    SpeechSynthesizer, VoiceProfileRepository,
};
use friendly_voice_domain::{
    accent::AccentRenderer,
    ids::SpeakerId,
    message::{MessageContent, MotivationalMessage},
    voice::SpeechScript,
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct SpeakRequest {
    pub speaker_id: SpeakerId,
    pub content: MessageContent,
    /// Nome de quem vai ouvir (personaliza a mensagem).
    pub listener: Option<String>,
    /// 0.0 = sem sotaque, 1.0 = sotaque completo aprendido.
    pub accent_intensity: f32,
}

#[derive(Debug, Clone)]
pub struct SpokenMessage {
    pub message: MotivationalMessage,
    pub script: SpeechScript,
    pub receipt: String,
}

/// US-05: pedir uma mensagem na hora (também usado pelos agendamentos).
pub struct SpeakMessage {
    speakers: Arc<dyn SpeakerRepository>,
    profiles: Arc<dyn VoiceProfileRepository>,
    messages: Arc<dyn MessageSource>,
    phonemizer: Arc<dyn Phonemizer>,
    synthesizer: Arc<dyn SpeechSynthesizer>,
    delivery: Arc<dyn MessageDelivery>,
    clock: Arc<dyn Clock>,
    renderer: AccentRenderer,
}

impl SpeakMessage {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        speakers: Arc<dyn SpeakerRepository>,
        profiles: Arc<dyn VoiceProfileRepository>,
        messages: Arc<dyn MessageSource>,
        phonemizer: Arc<dyn Phonemizer>,
        synthesizer: Arc<dyn SpeechSynthesizer>,
        delivery: Arc<dyn MessageDelivery>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            speakers,
            profiles,
            messages,
            phonemizer,
            synthesizer,
            delivery,
            clock,
            renderer: AccentRenderer,
        }
    }

    pub fn execute(&self, req: SpeakRequest) -> AppResult<SpokenMessage> {
        let speaker = self
            .speakers
            .find(&req.speaker_id)?
            .ok_or_else(|| AppError::not_found("pessoa", &req.speaker_id))?;
        let voice = self.profiles.find(&req.speaker_id)?.ok_or_else(|| {
            AppError::Precondition(format!(
                "a voz de {} ainda não foi gerada; envie amostras e rode a geração do perfil",
                speaker.name
            ))
        })?;

        let message = match req.content {
            MessageContent::Category(c) => {
                self.messages.pick(c, &speaker, req.listener.as_deref())?
            }
            MessageContent::FixedText(t) => MotivationalMessage::new(&t, None)?,
        };

        let citation = self.phonemizer.phonemize(message.text());
        let pronunciation = self
            .renderer
            .render(&voice.accent, &citation, req.accent_intensity);
        let script = SpeechScript {
            text: message.text().to_string(),
            pronunciation,
            prosody: voice.prosody,
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
    fn speaks_with_accent_and_delivers() {
        let f = Fakes::new();
        let spk = f.speaker();
        f.carioca_profile(&spk);
        let uc = f.speak_message();
        let out = uc
            .execute(SpeakRequest {
                speaker_id: spk.id.clone(),
                content: MessageContent::FixedText("leite quente".into()),
                listener: None,
                accent_intensity: 1.0,
            })
            .unwrap();
        assert_eq!(out.script.pronunciation.ipa(), "lejtʃi kẽtʃi");
        assert_eq!(f.delivery.delivered(), vec!["leite quente".to_string()]);

        let out = uc
            .execute(SpeakRequest {
                speaker_id: spk.id,
                content: MessageContent::Category(MessageCategory::Motivation),
                listener: Some("Ana".into()),
                accent_intensity: 0.0,
            })
            .unwrap();
        assert!(out.message.text().contains("Ana"));
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
                accent_intensity: 1.0,
            })
            .unwrap_err();
        assert!(matches!(err, AppError::Precondition(_)));
    }
}
