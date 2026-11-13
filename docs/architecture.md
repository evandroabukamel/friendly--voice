# Architecture

## Layers (Clean Architecture)

```
┌─────────────────────────────────────────────────────────────┐
│ cli / mobile  (presentation)   bootstrap (composition root) │
│  ┌───────────────────────────────────────────────────────┐  │
│  │ infrastructure  (adapters: WAV/FLAC, DSP, JSON,       │  │
│  │                  HTTP, catalog, delivery, clock)      │  │
│  │  ┌─────────────────────────────────────────────────┐  │  │
│  │  │ application  (use cases + ports/traits)         │  │  │
│  │  │  ┌───────────────────────────────────────────┐  │  │  │
│  │  │  │ domain  (entities, language, voice, tone) │  │  │  │
│  │  │  └───────────────────────────────────────────┘  │  │  │
│  │  └─────────────────────────────────────────────────┘  │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
          dependencies always point inwards →
```

The **dependency rule** is enforced by the compiler: each layer is a crate,
the `Cargo.toml` of `domain` depends on no other crate of the project, and
`application` depends only on `domain`.

| Crate | Responsibility | Depends on |
|-------|----------------|-----------|
| `friendly-voice-domain` | Entities, value objects, domain services (language, voice gender, tone, schedules) | `chrono`, `serde`, `thiserror` |
| `friendly-voice-application` | Use cases and **ports** (traits) | `domain` |
| `friendly-voice-infrastructure` | Port implementations | `domain`, `application`, `hound`, `serde_json`, (`ureq`) |
| `friendly-voice-bootstrap` | Composition root (dependency wiring) | `domain`, `application`, `infrastructure` |
| `friendly-voice` (cli) | Command line interface | `bootstrap` |
| `friendly-voice-mobile` | Android interface: JNI bridge with JSON in/out | `bootstrap`, `jni` |

The Android app (`android/`, Kotlin) is just another presentation layer:
it calls `NativeBridge` → `crates/mobile` → use cases. No business rule is
rewritten in Kotlin (see ADR 0005).

The voice engine (`engine/`, Python) is an external system behind the ports
`VoiceCloningEngine`, `SpeechSynthesizer` and `Transcriber`, reached through
the HTTP adapter (see ADR 0003 and 0007).

## Voice cloning — data flow

```
 WAV/FLAC ──► AudioFileDecoder ──► AudioClip ──► SampleQualityPolicy (domain)
                                        │
                                        └──► DspAcousticAnalyzer ──► pitch, pauses
 transcript (optional: --transcript, .txt or Transcriber in the person's language)

 all samples ──► ProsodyProfile ──► VoiceGender::from_pitch (≥ 160 Hz → female)
            └──► HttpVoiceEngine.enroll(language, gender, recordings)
                    engine: speaker embedding = mean over the recordings
                            reference = 10 s window closest to it ──► voice_id

 text + language + VoiceTone ──► HttpVoiceEngine.synthesize
                    engine: Chatterbox Multilingual, sentence by sentence ──► WAV
```

The voice — timbre, intonation, rhythm and accent — is imitated by the neural
model from the reference audio. The domain keeps what the app decides: the
person's language, consent, the detected gender, the prosody used by the
local sketch engine, the tone, and the schedules.

## Tone of voice

`VoiceTone` (domain) = style (`ToneStyle`) + pitch shift in semitones. Each
style is a `ToneAdjustment` (semitones, rate factor, melodic expressiveness,
volume, breathiness) applied **on top of** the person's natural prosody — the
voice is still theirs, only its colour changes. The message theme suggests the
initial tone (`ToneStyle::default_for`), which the user can override on every
request or schedule.

## SOLID in practice

* **S** — one use case per file (`RegisterSpeaker`, `AddVoiceSample`,
  `BuildVoiceProfile`, `SpeakMessage`, ...).
* **O** — new audio formats, engines or storage = new adapters, without
  changing the use cases.
* **L** — any `SpeechSynthesizer` implementation (local sketch, HTTP engine,
  test fake) is interchangeable.
* **I** — small ports: `Transcriber` is optional and separate; one
  repository per aggregate.
* **D** — use cases receive `Arc<dyn Port>`; only the composition root
  (`crates/bootstrap`) knows concrete types — and it is the same for CLI and Android.
