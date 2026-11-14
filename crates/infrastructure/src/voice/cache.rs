//! Synthesized-speech cache (ADR 0008): a decorator around any
//! [`SpeechSynthesizer`] that keeps every generated WAV on disk, so the same
//! talk is never asked from the engine twice.
use friendly_voice_application::ports::{
    PortError, PortResult, SpeechSynthesizer, SynthesizedAudio,
};
use friendly_voice_domain::voice::{SpeechScript, VoiceProfile};
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// Bumped when the key layout changes, so old entries are simply ignored.
const KEY_VERSION: u32 = 1;

#[derive(Serialize)]
struct CacheKey<'a> {
    version: u32,
    engine: &'a str,
    voice: &'a VoiceProfile,
    script: &'a SpeechScript,
}

/// FNV-1a, 128 bits: stable across Rust versions and platforms (unlike
/// `DefaultHasher`), so cached files survive upgrades.
fn fnv1a_128(bytes: &[u8]) -> u128 {
    const OFFSET: u128 = 0x6c62272e07bb014262b821756295c58d;
    const PRIME: u128 = 0x0000000001000000000000000000013B;
    bytes
        .iter()
        .fold(OFFSET, |h, b| (h ^ u128::from(*b)).wrapping_mul(PRIME))
}

/// Keeps only characters that are safe in a folder name.
fn folder(id: &str) -> String {
    let safe: String = id
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();
    if safe.is_empty() {
        "_".into()
    } else {
        safe
    }
}

static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Writes to a temporary file in the same folder and renames it over the
/// target, so a crash never leaves a half-written `.wav` that is served later.
fn write_atomically(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    let dir = path.parent().unwrap_or(Path::new("."));
    fs::create_dir_all(dir)?;
    let tmp = path.with_extension(format!(
        "{}-{}.tmp",
        std::process::id(),
        TEMP_COUNTER.fetch_add(1, Ordering::Relaxed)
    ));
    let result = (|| {
        use std::io::Write;
        let mut f = fs::File::create(&tmp)?;
        f.write_all(bytes)?;
        f.sync_all()?;
        drop(f);
        fs::rename(&tmp, path)
    })();
    if result.is_err() {
        let _ = fs::remove_file(&tmp);
    }
    result
}

/// Stores audio at `<root>/<speaker-id>/<key>.wav`. The key covers
/// everything that changes the audio: the engine, the whole voice profile
/// (engine voice id, language, prosody and `built_at`, so rebuilding the
/// voice invalidates it) and the whole script (final text, language, tone
/// style, pitch shift and the tone-coloured prosody).
pub struct CachedSpeechSynthesizer {
    inner: Arc<dyn SpeechSynthesizer>,
    root: PathBuf,
    engine: String,
}

impl CachedSpeechSynthesizer {
    /// `engine` names the synthesizer behind the cache (e.g. "sketch",
    /// "neural"), so audio from one engine is never served for another.
    pub fn new(
        inner: Arc<dyn SpeechSynthesizer>,
        root: impl Into<PathBuf>,
        engine: impl Into<String>,
    ) -> Self {
        Self {
            inner,
            root: root.into(),
            engine: engine.into(),
        }
    }

    fn path(&self, voice: &VoiceProfile, script: &SpeechScript) -> PortResult<PathBuf> {
        let key = serde_json::to_vec(&CacheKey {
            version: KEY_VERSION,
            engine: &self.engine,
            voice,
            script,
        })
        .map_err(PortError::new)?;
        Ok(self
            .root
            .join(folder(voice.speaker_id.as_str()))
            .join(format!("{:032x}.wav", fnv1a_128(&key))))
    }
}

impl SpeechSynthesizer for CachedSpeechSynthesizer {
    fn synthesize(
        &self,
        voice: &VoiceProfile,
        script: &SpeechScript,
    ) -> PortResult<SynthesizedAudio> {
        let path = self.path(voice, script)?;
        if let Ok(wav) = fs::read(&path) {
            if !wav.is_empty() {
                return Ok(SynthesizedAudio { wav });
            }
        }
        let audio = self.inner.synthesize(voice, script)?;
        // A cache that cannot be written only costs a future request.
        let _ = write_atomically(&path, &audio.wav);
        Ok(audio)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use friendly_voice_application::ports::PortError;
    use friendly_voice_domain::{
        ids::SpeakerId,
        language::Language,
        tone::{ToneStyle, VoiceTone},
        voice::ProsodyProfile,
    };
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// Counts the calls; answers with the text as "audio".
    #[derive(Default)]
    struct Counting(AtomicUsize);
    impl SpeechSynthesizer for Counting {
        fn synthesize(&self, _: &VoiceProfile, s: &SpeechScript) -> PortResult<SynthesizedAudio> {
            self.0.fetch_add(1, Ordering::SeqCst);
            if s.text == "fail" {
                return Err(PortError::new("engine down"));
            }
            Ok(SynthesizedAudio {
                wav: s.text.as_bytes().to_vec(),
            })
        }
    }
    impl Counting {
        fn calls(&self) -> usize {
            self.0.load(Ordering::SeqCst)
        }
    }

    fn dir(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("fv-cache-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        d
    }

    fn voice() -> VoiceProfile {
        VoiceProfile {
            speaker_id: SpeakerId::new("spk-1"),
            engine_voice_id: "v1".into(),
            language: Language::default(),
            gender: None,
            prosody: ProsodyProfile::default(),
            sample_count: 1,
            total_audio_secs: 10.0,
            built_at: None,
        }
    }

    fn script(text: &str) -> SpeechScript {
        SpeechScript {
            text: text.into(),
            language: Language::default(),
            prosody: ProsodyProfile::default(),
            tone: VoiceTone::of(ToneStyle::Warm),
        }
    }

    fn files(root: &std::path::Path) -> Vec<String> {
        let mut out = Vec::new();
        for spk in std::fs::read_dir(root).unwrap() {
            for f in std::fs::read_dir(spk.unwrap().path()).unwrap() {
                out.push(f.unwrap().file_name().to_string_lossy().into_owned());
            }
        }
        out
    }

    #[test]
    fn the_same_talk_is_synthesized_once() {
        let root = dir("same");
        let inner = Arc::new(Counting::default());
        let cache = CachedSpeechSynthesizer::new(inner.clone(), &root, "neural");
        let first = cache.synthesize(&voice(), &script("bom dia")).unwrap();
        let second = cache.synthesize(&voice(), &script("bom dia")).unwrap();
        assert_eq!(inner.calls(), 1);
        assert_eq!(first, second);
        // A new process (new decorator) still finds it on disk.
        let again = CachedSpeechSynthesizer::new(inner.clone(), &root, "neural");
        again.synthesize(&voice(), &script("bom dia")).unwrap();
        assert_eq!(inner.calls(), 1);
        let names = files(&root);
        assert_eq!(names.len(), 1, "no temp files left behind: {names:?}");
        assert!(names[0].ends_with(".wav"));
        assert!(root.join("spk-1").is_dir());
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn anything_that_changes_the_audio_misses() {
        let root = dir("miss");
        let inner = Arc::new(Counting::default());
        let cache = CachedSpeechSynthesizer::new(inner.clone(), &root, "neural");
        let base = script("bom dia");
        cache.synthesize(&voice(), &base).unwrap();

        let mut other_text = base.clone();
        other_text.text = "boa noite".into();
        let mut other_tone = base.clone();
        other_tone.tone = VoiceTone::of(ToneStyle::Whisper);
        let mut other_pitch = base.clone();
        other_pitch.tone = VoiceTone::new(ToneStyle::Warm, 2.0).unwrap();
        let mut other_language = base.clone();
        other_language.language = "es".parse().unwrap();
        for s in [&other_text, &other_tone, &other_pitch, &other_language] {
            cache.synthesize(&voice(), s).unwrap();
        }
        assert_eq!(inner.calls(), 5);

        let mut rebuilt = voice();
        rebuilt.built_at = Some(chrono::NaiveDateTime::default());
        cache.synthesize(&rebuilt, &base).unwrap();
        assert_eq!(inner.calls(), 6, "a rebuilt voice is a new voice");

        let sketch = CachedSpeechSynthesizer::new(inner.clone(), &root, "sketch");
        sketch.synthesize(&voice(), &base).unwrap();
        assert_eq!(inner.calls(), 7, "another engine is not served this audio");
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn failures_and_broken_files_are_not_served() {
        let root = dir("broken");
        let inner = Arc::new(Counting::default());
        let cache = CachedSpeechSynthesizer::new(inner.clone(), &root, "neural");
        assert!(cache.synthesize(&voice(), &script("fail")).is_err());
        assert!(cache.synthesize(&voice(), &script("fail")).is_err());
        assert_eq!(inner.calls(), 2, "errors are not cached");

        cache.synthesize(&voice(), &script("oi")).unwrap();
        // Simulate a file truncated by something outside the cache.
        let spk = root.join("spk-1");
        for f in std::fs::read_dir(&spk).unwrap() {
            std::fs::write(f.unwrap().path(), b"").unwrap();
        }
        assert_eq!(
            cache.synthesize(&voice(), &script("oi")).unwrap().wav,
            b"oi"
        );
        assert_eq!(inner.calls(), 4, "an empty file is a miss");
        std::fs::remove_dir_all(root).unwrap();
    }
}
