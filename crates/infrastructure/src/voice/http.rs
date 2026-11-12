//! HTTP adapter for a self-hosted neural voice server.
//!
//! The contract is documented in `docs/engine-api.md`. The reference
//! implementation lives in `engine/` (Piper pt-BR base speech + OpenVoice V2
//! tone color conversion, both permissively licensed); any server that
//! implements the same endpoints can be plugged in.
use super::ssml::to_ssml;
use crate::audio::{encode_wav, to_wav};
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
/// HTTP status used by the server for optional features it does not provide.
const NOT_IMPLEMENTED: u16 = 501;

fn err(e: impl std::fmt::Display) -> PortError {
    PortError::new(format!("servidor de voz: {e}"))
}

/// Failure modes the adapters need to tell apart.
enum CallError {
    /// The server does not offer this optional endpoint.
    NotImplemented,
    Other(PortError),
}

impl From<CallError> for PortError {
    fn from(e: CallError) -> Self {
        match e {
            CallError::NotImplemented => err("recurso não disponível neste servidor"),
            CallError::Other(e) => e,
        }
    }
}

#[derive(Debug, Clone)]
struct Client {
    base_url: String,
}

impl Client {
    fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}/{}", self.base_url.trim_end_matches('/'), path)
    }

    /// POSTs JSON and returns the raw body, turning non-2xx answers into
    /// errors that carry the server's own explanation (`detail`).
    fn post(&self, path: &str, body: &impl Serialize) -> Result<Vec<u8>, CallError> {
        let mut response = ureq::post(&self.url(path))
            .config()
            .http_status_as_error(false)
            .build()
            .send_json(body)
            .map_err(|e| CallError::Other(err(e)))?;
        let status = response.status().as_u16();
        let bytes = response
            .body_mut()
            .with_config()
            .limit(MAX_AUDIO_BYTES)
            .read_to_vec()
            .map_err(|e| CallError::Other(err(e)))?;
        match status {
            200..=299 => Ok(bytes),
            NOT_IMPLEMENTED => Err(CallError::NotImplemented),
            _ => {
                #[derive(Deserialize)]
                struct ErrorBody {
                    detail: serde_json::Value,
                }
                let detail = serde_json::from_slice::<ErrorBody>(&bytes)
                    .map(|b| match b.detail {
                        serde_json::Value::String(s) => s,
                        other => other.to_string(),
                    })
                    .unwrap_or_else(|_| String::from_utf8_lossy(&bytes).into_owned());
                Err(CallError::Other(err(format!("HTTP {status}: {detail}"))))
            }
        }
    }

    fn post_json<Res: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        body: &impl Serialize,
    ) -> Result<Res, CallError> {
        let bytes = self.post(path, body)?;
        serde_json::from_slice(&bytes).map_err(|e| CallError::Other(err(e)))
    }
}

/// Voice cloning (`POST /v1/voices`) and synthesis (`POST /v1/synthesize`).
#[derive(Debug, Clone)]
pub struct HttpVoiceEngine {
    client: Client,
}

impl HttpVoiceEngine {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            client: Client::new(base_url),
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
            // The contract carries WAV; other formats (FLAC) are converted here,
            // so engines do not need to support every input format.
            references: references
                .iter()
                .map(|r| {
                    Ok(EnrollReference {
                        audio_wav_base64: B64.encode(to_wav(&r.bytes)?),
                        transcript: &r.transcript,
                    })
                })
                .collect::<PortResult<_>>()?,
        };
        let res: EnrollResponse = self.client.post_json("v1/voices", &req)?;
        Ok(res.voice_id)
    }
}

/// One word as it must be pronounced (the learned accent is already applied).
#[derive(Serialize)]
struct WordPayload<'a> {
    phonemes: Vec<&'a str>,
    stress: Option<usize>,
    pause_after: bool,
}

#[derive(Serialize)]
struct SynthesizeRequest<'a> {
    voice_id: &'a str,
    language: &'a str,
    text: &'a str,
    /// SSML with one `<phoneme alphabet="ipa">` per word.
    ssml: &'a str,
    /// Accented pronunciation per word, with stress and pauses (preferred).
    words: Vec<WordPayload<'a>>,
    /// Same pronunciation as plain IPA per word (for engines without `words`).
    ipa_words: Vec<String>,
    pitch_hz: f32,
    syllables_per_second: f32,
    /// Chosen tone of voice (natural, carinhoso, animado, calmo, firme, sussurrado).
    style: &'a str,
    /// Tone adjustments relative to the person's natural prosody.
    pitch_shift_semitones: f32,
    volume_db: f32,
}

fn synthesize_request<'a>(
    voice: &'a VoiceProfile,
    script: &'a SpeechScript,
    ssml: &'a str,
) -> SynthesizeRequest<'a> {
    let tone = script.tone.adjustment();
    SynthesizeRequest {
        voice_id: &voice.engine_voice_id,
        language: LANGUAGE,
        text: &script.text,
        ssml,
        words: script
            .pronunciation
            .words
            .iter()
            .map(|w| WordPayload {
                phonemes: w.phonemes.iter().map(Phoneme::symbol).collect(),
                stress: w.stress,
                pause_after: w.pause_after,
            })
            .collect(),
        ipa_words: script.pronunciation.words.iter().map(|w| w.ipa()).collect(),
        pitch_hz: script.prosody.mean_pitch_hz,
        syllables_per_second: script.prosody.syllables_per_second,
        style: script.tone.style.slug(),
        pitch_shift_semitones: tone.pitch_semitones,
        volume_db: tone.volume_db,
    }
}

impl SpeechSynthesizer for HttpVoiceEngine {
    fn synthesize(
        &self,
        voice: &VoiceProfile,
        script: &SpeechScript,
    ) -> PortResult<SynthesizedAudio> {
        let ssml = to_ssml(script);
        let wav = self
            .client
            .post("v1/synthesize", &synthesize_request(voice, script, &ssml))?;
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

/// Phone recognition (`POST /v1/phonemes` → `{"ipa": "..."}`).
///
/// Optional on the server: when it answers 501, no phonemes are returned
/// and the sample is stored without accent evidence instead of failing.
#[derive(Debug, Clone)]
pub struct HttpPhoneRecognizer {
    client: Client,
}

impl HttpPhoneRecognizer {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            client: Client::new(base_url),
        }
    }
}

#[derive(Deserialize)]
struct PhonemesResponse {
    ipa: String,
}

impl PhoneRecognizer for HttpPhoneRecognizer {
    fn recognize(&self, clip: &AudioClip, transcript: &str) -> PortResult<Vec<Phoneme>> {
        let req = audio_request(clip, Some(transcript))?;
        match self
            .client
            .post_json::<PhonemesResponse>("v1/phonemes", &req)
        {
            Ok(res) => Ok(parse_ipa(&res.ipa)),
            Err(CallError::NotImplemented) => Ok(Vec::new()),
            Err(e) => Err(e.into()),
        }
    }
}

/// Transcription (`POST /v1/transcribe` → `{"text": "..."}`).
#[derive(Debug, Clone)]
pub struct HttpTranscriber {
    client: Client,
}

impl HttpTranscriber {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            client: Client::new(base_url),
        }
    }
}

#[derive(Deserialize)]
struct TranscribeResponse {
    text: String,
}

impl Transcriber for HttpTranscriber {
    fn transcribe(&self, clip: &AudioClip) -> PortResult<String> {
        let req = audio_request(clip, None)?;
        match self.client.post_json::<TranscribeResponse>("v1/transcribe", &req) {
            Ok(res) => Ok(res.text),
            Err(CallError::NotImplemented) => Err(err(
                "o servidor não faz transcrição; informe o texto do áudio (--transcript ou arquivo .txt)",
            )),
            Err(e) => Err(e.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use friendly_voice_domain::accent::AccentProfile;
    use friendly_voice_domain::ids::SpeakerId;
    use friendly_voice_domain::phonetics::{PhoneticUtterance, PhoneticWord};
    use friendly_voice_domain::tone::{ToneStyle, VoiceTone};
    use friendly_voice_domain::voice::ProsodyProfile;
    use std::io::{BufRead, BufReader, Read, Write};
    use std::net::TcpListener;
    use std::sync::mpsc;

    /// Minimal one-shot HTTP server: answers with `status`/`body` and hands
    /// the request body back to the test.
    fn serve_once(status: u16, body: &'static str) -> (String, mpsc::Receiver<String>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut length = 0;
            loop {
                let mut line = String::new();
                reader.read_line(&mut line).unwrap();
                if line == "\r\n" {
                    break;
                }
                if let Some(v) = line.to_lowercase().strip_prefix("content-length:") {
                    length = v.trim().parse().unwrap();
                }
            }
            let mut request = vec![0; length];
            reader.read_exact(&mut request).unwrap();
            let _ = tx.send(String::from_utf8(request).unwrap());
            let mut stream = stream;
            write!(
                stream,
                "HTTP/1.1 {status} X\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
                body.len()
            )
            .unwrap();
        });
        (url, rx)
    }

    fn clip() -> AudioClip {
        AudioClip::new(16_000, vec![0.1; 16_000]).unwrap()
    }

    #[test]
    fn missing_phone_recognizer_is_not_an_error() {
        let (url, _) = serve_once(501, r#"{"detail":"not installed"}"#);
        assert!(HttpPhoneRecognizer::new(url)
            .recognize(&clip(), "oi")
            .unwrap()
            .is_empty());
    }

    #[test]
    fn server_error_detail_is_reported() {
        let (url, _) = serve_once(422, r#"{"detail":"no reference audio longer than 1s"}"#);
        let e = HttpTranscriber::new(url).transcribe(&clip()).unwrap_err();
        assert!(
            e.0.contains("HTTP 422") && e.0.contains("longer than 1s"),
            "{e}"
        );
    }

    #[test]
    fn synthesis_sends_accented_words_with_stress_and_tone() {
        let (url, rx) = serve_once(200, "RIFF");
        let voice = VoiceProfile {
            speaker_id: SpeakerId::new("s"),
            engine_voice_id: "abc".into(),
            accent: AccentProfile::default(),
            prosody: ProsodyProfile::default(),
            sample_count: 1,
            total_audio_secs: 10.0,
        };
        let script = SpeechScript {
            text: "noite".into(),
            pronunciation: PhoneticUtterance::new(vec![PhoneticWord::new(
                "noite",
                parse_ipa("nojtʃi"),
                Some(1),
            )
            .with_pause(true)]),
            prosody: ProsodyProfile::default(),
            tone: VoiceTone::of(ToneStyle::Calm),
        };
        let audio = HttpVoiceEngine::new(url)
            .synthesize(&voice, &script)
            .unwrap();
        assert_eq!(audio.wav, b"RIFF");
        let sent: serde_json::Value = serde_json::from_str(&rx.recv().unwrap()).unwrap();
        assert_eq!(sent["voice_id"], "abc");
        assert_eq!(
            sent["words"][0]["phonemes"],
            serde_json::json!(["n", "o", "j", "tʃ", "i"])
        );
        assert_eq!(sent["words"][0]["stress"], 1);
        assert_eq!(sent["words"][0]["pause_after"], true);
        assert_eq!(sent["style"], "calmo");
        assert_eq!(sent["volume_db"], -3.0);
    }
}
