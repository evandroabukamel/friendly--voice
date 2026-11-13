//! HTTP adapter for a self-hosted neural voice server.
//!
//! The contract is documented in `docs/engine-api.md`. The reference
//! implementation lives in `engine/` (zero-shot cloning with Chatterbox
//! Multilingual, MIT licensed); any server that implements the same
//! endpoints can be plugged in.
use crate::audio::{encode_wav, to_wav};
use base64::{engine::general_purpose::STANDARD as B64, Engine};
use friendly_voice_application::ports::{
    PortError, PortResult, ReferenceAudio, SpeechSynthesizer, SynthesizedAudio, Transcriber,
    VoiceCloningEngine,
};
use friendly_voice_domain::{
    audio::AudioClip,
    language::Language,
    speaker::Speaker,
    voice::{SpeechScript, VoiceGender, VoiceProfile},
};
use serde::{Deserialize, Serialize};

const MAX_AUDIO_BYTES: u64 = 64 * 1024 * 1024;
/// HTTP status used by the server for optional features it does not provide.
const NOT_IMPLEMENTED: u16 = 501;

fn err(e: impl std::fmt::Display) -> PortError {
    PortError::new(format!("voice server: {e}"))
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
            CallError::NotImplemented => err("feature not available on this server"),
            CallError::Other(e) => e,
        }
    }
}

#[derive(Debug, Clone)]
struct Client {
    base_url: String,
}

impl Client {
    /// `localhost` is pinned to IPv4: on Windows it resolves to `::1` first,
    /// and Docker Desktop accepts that connection but stalls for ~20 s
    /// before the client falls back to `127.0.0.1`.
    fn new(base_url: impl Into<String>) -> Self {
        let base_url: String = base_url.into();
        Self {
            base_url: base_url.replacen("://localhost", "://127.0.0.1", 1),
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
    gender: Option<&'a str>,
    references: Vec<EnrollReference<'a>>,
}

#[derive(Serialize)]
struct EnrollReference<'a> {
    audio_wav_base64: String,
    transcript: Option<&'a str>,
}

#[derive(Deserialize)]
struct EnrollResponse {
    voice_id: String,
}

impl VoiceCloningEngine for HttpVoiceEngine {
    fn enroll(
        &self,
        speaker: &Speaker,
        gender: Option<VoiceGender>,
        references: &[ReferenceAudio],
    ) -> PortResult<String> {
        let req = EnrollRequest {
            name: &speaker.name,
            language: speaker.language.tag(),
            gender: gender.map(VoiceGender::slug),
            // The contract carries WAV; other formats (FLAC) are converted here,
            // so engines do not need to support every input format.
            references: references
                .iter()
                .map(|r| {
                    Ok(EnrollReference {
                        audio_wav_base64: B64.encode(to_wav(&r.bytes)?),
                        transcript: r.transcript.as_deref(),
                    })
                })
                .collect::<PortResult<_>>()?,
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
    /// Chosen tone of voice (natural, warm, cheerful, calm, firm, whisper).
    style: &'a str,
    /// Extra pitch shift chosen by the user, in semitones.
    pitch_shift_semitones: f32,
    /// Volume change of the tone, in dB.
    volume_db: f32,
}

fn synthesize_request<'a>(
    voice: &'a VoiceProfile,
    script: &'a SpeechScript,
) -> SynthesizeRequest<'a> {
    SynthesizeRequest {
        voice_id: &voice.engine_voice_id,
        language: script.language.tag(),
        text: &script.text,
        style: script.tone.style.slug(),
        pitch_shift_semitones: script.tone.pitch_shift_semitones,
        volume_db: script.tone.style.adjustment().volume_db,
    }
}

impl SpeechSynthesizer for HttpVoiceEngine {
    fn synthesize(
        &self,
        voice: &VoiceProfile,
        script: &SpeechScript,
    ) -> PortResult<SynthesizedAudio> {
        let wav = self
            .client
            .post("v1/synthesize", &synthesize_request(voice, script))?;
        Ok(SynthesizedAudio { wav })
    }
}

#[derive(Serialize)]
struct AudioRequest<'a> {
    language: &'a str,
    audio_wav_base64: String,
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
    fn transcribe(&self, clip: &AudioClip, language: &Language) -> PortResult<String> {
        let req = AudioRequest {
            language: language.tag(),
            audio_wav_base64: B64.encode(encode_wav(clip)?),
        };
        match self.client.post_json::<TranscribeResponse>("v1/transcribe", &req) {
            Ok(res) => Ok(res.text),
            Err(CallError::NotImplemented) => Err(err(
                "the server does not transcribe; provide the transcript (--transcript or a .txt file) or leave it out",
            )),
            Err(e) => Err(e.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use friendly_voice_domain::ids::SpeakerId;
    use friendly_voice_domain::speaker::Consent;
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
    fn localhost_is_reached_over_ipv4() {
        assert_eq!(
            Client::new("http://localhost:8020/").url("v1/voices"),
            "http://127.0.0.1:8020/v1/voices"
        );
        assert_eq!(
            Client::new("http://engine.lan:8020").url("health"),
            "http://engine.lan:8020/health"
        );
    }

    #[test]
    fn server_error_detail_is_reported() {
        let (url, _) = serve_once(422, r#"{"detail":"no reference audio longer than 1s"}"#);
        let e = HttpTranscriber::new(url)
            .transcribe(&clip(), &Language::default())
            .unwrap_err();
        assert!(
            e.0.contains("HTTP 422") && e.0.contains("longer than 1s"),
            "{e}"
        );
    }

    #[test]
    fn synthesis_sends_text_language_and_tone() {
        let (url, rx) = serve_once(200, "RIFF");
        let voice = VoiceProfile {
            speaker_id: SpeakerId::new("s"),
            engine_voice_id: "abc".into(),
            language: Language::default(),
            gender: Some(VoiceGender::Female),
            prosody: ProsodyProfile::default(),
            sample_count: 1,
            total_audio_secs: 10.0,
        };
        let script = SpeechScript {
            text: "Boa noite, meu bem.".into(),
            language: "es".parse().unwrap(),
            prosody: ProsodyProfile::default(),
            tone: VoiceTone::new(ToneStyle::Calm, 1.5).unwrap(),
        };
        let audio = HttpVoiceEngine::new(url)
            .synthesize(&voice, &script)
            .unwrap();
        assert_eq!(audio.wav, b"RIFF");
        let sent: serde_json::Value = serde_json::from_str(&rx.recv().unwrap()).unwrap();
        assert_eq!(sent["voice_id"], "abc");
        assert_eq!(sent["text"], "Boa noite, meu bem.");
        assert_eq!(sent["language"], "es");
        assert_eq!(sent["style"], "calm");
        assert_eq!(sent["pitch_shift_semitones"], 1.5);
        assert_eq!(sent["volume_db"], -3.0);
    }

    #[test]
    fn enrollment_sends_language_and_gender() {
        let (url, rx) = serve_once(200, r#"{"voice_id":"v1"}"#);
        let speaker = Speaker::new(
            SpeakerId::new("s"),
            "Vó",
            None,
            Some(Consent {
                granted_by: "Vó".into(),
                granted_at: Default::default(),
                statement: "ok".into(),
            }),
        )
        .unwrap()
        .with_language("it".parse().unwrap());
        let wav = encode_wav(&clip()).unwrap();
        let id = HttpVoiceEngine::new(url)
            .enroll(
                &speaker,
                Some(VoiceGender::Male),
                &[ReferenceAudio {
                    bytes: wav,
                    transcript: None,
                }],
            )
            .unwrap();
        assert_eq!(id, "v1");
        let sent: serde_json::Value = serde_json::from_str(&rx.recv().unwrap()).unwrap();
        assert_eq!(sent["language"], "it");
        assert_eq!(sent["gender"], "male");
        assert!(sent["references"][0]["transcript"].is_null());
    }
}
