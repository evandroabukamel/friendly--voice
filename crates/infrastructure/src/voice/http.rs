//! Adaptador HTTP para um servidor de voz neural auto-hospedado.
//!
//! O contrato (documentado em `docs/engine-api.md`) é simples o bastante
//! para ser implementado sobre XTTS, OpenVoice, F5-TTS (clonagem/síntese),
//! Whisper (transcrição) e wav2vec2-phoneme/Allosaurus (fonemas).
use super::ssml::to_ssml;
use crate::audio::encode_wav;
use base64::{engine::general_purpose::STANDARD as B64, Engine};
use friendly_voice_application::ports::{
    PhoneRecognizer, PortError, PortResult, ReferenceAudio, SpeechSynthesizer, SynthesizedAudio,
    Transcriber, VoiceCloningEngine,
};
use friendly_voice_domain::{
    audio::AudioClip,
    phonetics::{parse_ipa, Phoneme},
    speaker::Speaker,
    voice::{SpeechScript, VoiceProfile},
};
use serde::{Deserialize, Serialize};

const MAX_AUDIO_BYTES: u64 = 64 * 1024 * 1024;
const LANGUAGE: &str = "pt-BR";

fn err(e: impl std::fmt::Display) -> PortError {
    PortError::new(format!("servidor de voz: {e}"))
}

#[derive(Debug, Clone)]
struct Client {
    base_url: String,
}

impl Client {
    fn url(&self, path: &str) -> String {
        format!("{}/{}", self.base_url.trim_end_matches('/'), path)
    }
    fn post_json<Req: Serialize, Res: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        body: &Req,
    ) -> PortResult<Res> {
        ureq::post(&self.url(path))
            .send_json(body)
            .map_err(err)?
            .body_mut()
            .read_json()
            .map_err(err)
    }
}

/// Clonagem (`POST /v1/voices`) e síntese (`POST /v1/synthesize`).
#[derive(Debug, Clone)]
pub struct HttpVoiceEngine {
    client: Client,
}

impl HttpVoiceEngine {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            client: Client {
                base_url: base_url.into(),
            },
        }
    }
}

#[derive(Serialize)]
struct EnrollRequest<'a> {
    name: &'a str,
    language: &'a str,
    references: Vec<EnrollReference<'a>>,
}
#[derive(Serialize)]
struct EnrollReference<'a> {
    audio_wav_base64: String,
    transcript: &'a str,
}
#[derive(Deserialize)]
struct EnrollResponse {
    voice_id: String,
}

impl VoiceCloningEngine for HttpVoiceEngine {
    fn enroll(&self, speaker: &Speaker, references: &[ReferenceAudio]) -> PortResult<String> {
        let req = EnrollRequest {
            name: &speaker.name,
            language: LANGUAGE,
            references: references
                .iter()
                .map(|r| EnrollReference {
                    audio_wav_base64: B64.encode(&r.bytes),
                    transcript: &r.transcript,
                })
                .collect(),
        };
        let res: EnrollResponse = self.client.post_json("v1/voices", &req)?;
        Ok(res.voice_id)
    }
}

#[derive(Serialize)]
struct SynthesizeRequest<'a> {
    voice_id: &'a str,
    language: &'a str,
    text: &'a str,
    /// SSML com `<phoneme alphabet="ipa">` por palavra (sotaque aplicado).
    ssml: &'a str,
    /// Mesma pronúncia, como IPA por palavra (para motores sem SSML).
    ipa_words: Vec<String>,
    pitch_hz: f32,
    syllables_per_second: f32,
}

impl SpeechSynthesizer for HttpVoiceEngine {
    fn synthesize(
        &self,
        voice: &VoiceProfile,
        script: &SpeechScript,
    ) -> PortResult<SynthesizedAudio> {
        let ssml = to_ssml(script);
        let req = SynthesizeRequest {
            voice_id: &voice.engine_voice_id,
            language: LANGUAGE,
            text: &script.text,
            ssml: &ssml,
            ipa_words: script.pronunciation.words.iter().map(|w| w.ipa()).collect(),
            pitch_hz: script.prosody.mean_pitch_hz,
            syllables_per_second: script.prosody.syllables_per_second,
        };
        let wav = ureq::post(&self.client.url("v1/synthesize"))
            .send_json(&req)
            .map_err(err)?
            .body_mut()
            .with_config()
            .limit(MAX_AUDIO_BYTES)
            .read_to_vec()
            .map_err(err)?;
        Ok(SynthesizedAudio { wav, markup: ssml })
    }
}

#[derive(Serialize)]
struct AudioRequest<'a> {
    language: &'a str,
    audio_wav_base64: String,
    transcript: Option<&'a str>,
}

fn audio_request<'a>(
    clip: &AudioClip,
    transcript: Option<&'a str>,
) -> PortResult<AudioRequest<'a>> {
    Ok(AudioRequest {
        language: LANGUAGE,
        audio_wav_base64: B64.encode(encode_wav(clip)?),
        transcript,
    })
}

/// Reconhecimento fonético (`POST /v1/phonemes` → `{"ipa": "..."}`).
#[derive(Debug, Clone)]
pub struct HttpPhoneRecognizer {
    client: Client,
}

impl HttpPhoneRecognizer {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            client: Client {
                base_url: base_url.into(),
            },
        }
    }
}

#[derive(Deserialize)]
struct PhonemesResponse {
    ipa: String,
}

impl PhoneRecognizer for HttpPhoneRecognizer {
    fn recognize(&self, clip: &AudioClip, transcript: &str) -> PortResult<Vec<Phoneme>> {
        let res: PhonemesResponse = self
            .client
            .post_json("v1/phonemes", &audio_request(clip, Some(transcript))?)?;
        Ok(parse_ipa(&res.ipa))
    }
}

/// Transcrição (`POST /v1/transcribe` → `{"text": "..."}`).
#[derive(Debug, Clone)]
pub struct HttpTranscriber {
    client: Client,
}

impl HttpTranscriber {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            client: Client {
                base_url: base_url.into(),
            },
        }
    }
}

#[derive(Deserialize)]
struct TranscribeResponse {
    text: String,
}

impl Transcriber for HttpTranscriber {
    fn transcribe(&self, clip: &AudioClip) -> PortResult<String> {
        let res: TranscribeResponse = self
            .client
            .post_json("v1/transcribe", &audio_request(clip, None)?)?;
        Ok(res.text)
    }
}
