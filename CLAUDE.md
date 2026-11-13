# CLAUDE.md — Friendly Voice

Positive, motivational messages spoken **in the voice of someone you love**,
on demand or on a schedule. Rust core (Clean Architecture), a Python
voice-cloning server, a CLI and an Android app.

Start here, then follow the links: [`README.md`](README.md) (usage),
[`docs/architecture.md`](docs/architecture.md) (layers, data flow),
[`docs/engine-api.md`](docs/engine-api.md) (HTTP contract),
[`docs/adr/`](docs/adr) (decisions), [`docs/agile/`](docs/agile) (backlog,
sprints, Definition of Done).

## Language rules (non-negotiable)

- **Interface, code, comments, docs, commits, ADRs are English** (US-21,
  US-25, US-26). The former Portuguese command/option names stay as hidden
  clap aliases (`pessoa adicionar --nome`), covered by an acceptance test.
- Themes and tones accept English or Portuguese values (`warm`/`carinhoso`).
- The **spoken** message catalog is Portuguese (`infrastructure/src/messages.rs`).
- Error messages must be clear and actionable ("split it into pieces").

## Product requirements

Vision: *hear a loved one say "bom dia" before work and "respira fundo" in the
afternoon.* Full stories with Gherkin criteria:
[`docs/agile/product-backlog.md`](docs/agile/product-backlog.md).

| Epic | Delivered stories |
|---|---|
| E1 People & consent | US-01 register a person **only with recorded consent** (name + who authorized + timestamp) · US-27 language per person |
| E2 Voice cloning | US-02 upload samples (transcript optional) · US-11 zero-shot cloning from all recordings + female/male detection · US-24 FLAC input |
| E3 Messages | US-05 message now (theme or free text, personalized with the listener's name) · US-18 tone per message |
| E4 Schedules | US-07 rules (daily, weekdays, every N min in a window, once) + quiet hours · US-08 fire due messages exactly once, single catch-up |
| E5 Android | US-19 APK built in CI, release on `v*` tags · US-20 full flow on the phone with notifications |
| E6 Project | US-21 English docs and comments · US-25/26 English CLI and output |

Withdrawn (ADR 0007): the accent rules, G2P, phone recognizer, pronunciation
preview and accent intensity (US-03/04/06/09/10). Do not reintroduce them.
Open backlog (respect when touching related code): US-15 revoking consent
must delete samples and voice (Must), US-17 no repeated messages, US-16
MP3/OGG/M4A, US-28 themes in other languages, US-29 gendered first-person words.

## How it works

```
recordings ─► SampleQualityPolicy ─► DSP (mean pitch, spread, pauses)
transcript (optional: --transcript | .txt | /v1/transcribe in the person's language)
all samples ─► ProsodyProfile ─► VoiceGender::from_pitch (≥ 160 Hz → female)
            ─► engine /v1/voices (language, gender, all recordings)
               speaker embedding = mean over all recordings
               reference = 10 s window whose embedding is closest ─► voice_id

text + language + VoiceTone ─► engine /v1/synthesize ─► Chatterbox Multilingual,
    sentence by sentence, cut-off takes retried ─► PSOLA pitch shift, gain
    ─► WAV + note in .friendly-voice/outbox/
```

Key idea (ADR 0007): the neural model reads the text and imitates timbre,
intonation, rhythm and accent from the person's own recordings. The Rust
domain owns what the app decides: consent, language, detected gender,
prosody (for the local sketch engine), tone and schedules. The earlier
Piper + OpenVoice + accent-rules pipeline (ADR 0002/0006) sounded foreign and
robotic in listening tests and was replaced.

## Technical specification

### Layout and dependency rule (ADR 0001)

```
crates/domain          pure rules; depends on no project crate (chrono, serde, thiserror only)
crates/application     use cases (one per file) + ports (traits); depends only on domain
crates/infrastructure  adapters: WAV/FLAC decode, DSP, JSON repos, HTTP engine, sketch engine, delivery
crates/bootstrap       the ONLY composition root, shared by CLI and Android
crates/cli             `friendly-voice` binary (clap)
crates/mobile          JNI bridge, JSON in/out, panic-safe (ADR 0005)
android/               Kotlin app — presentation only, no business rules
engine/                Python FastAPI server: Chatterbox Multilingual
```

- New business rules go in `domain`; new I/O goes behind a port in
  `application/src/ports.rs` and is wired in `bootstrap`. Never import
  infrastructure from application or domain.
- Ports: `VoiceCloningEngine`, `SpeechSynthesizer`, `Transcriber` (optional),
  one repository per aggregate. Use cases take `Arc<dyn Port>`; tests use the
  in-memory fakes in `application/src/fakes.rs`.
- Audio format is detected by **magic bytes** (`RIFF…WAVE` / `fLaC`), never by
  extension (`infrastructure/src/audio/format.rs`). FLAC is converted to WAV
  before going to the engine, so the API contract stays WAV-only.
- Supported languages live in `domain/src/language.rs` (`SUPPORTED`) and must
  match `engine/friendly_voice_engine/languages.py`.
- Stored JSON must stay readable: new fields get `#[serde(default)]`, old
  fields are ignored (see the compatibility test in `domain/src/voice.rs`).

### Voice engines

| Build / config | Engine | Result |
|---|---|---|
| default build, no URL | local prosody **sketch** | person's rhythm and melody, **not** the voice — for dev/tests |
| `--features http-engine` + `--engine-url` / `FRIENDLY_VOICE_ENGINE_URL` | `HttpVoiceEngine` → `engine/` | cloned voice + language + tone |

Engine endpoints (contract in [`docs/engine-api.md`](docs/engine-api.md)):
`POST /v1/voices`, `POST /v1/synthesize`, optional `POST /v1/transcribe`
(faster-whisper `small` on the CPU, extra `[transcribe]`; 501 when missing →
sample stored without transcript), `GET /health`. Env vars: `FV_VOICES_DIR`,
`FV_ENGINE_HOME`, `FV_DEVICE` (`auto`), `HF_HOME` (weights).

Engine gotchas:
- `chatterbox-tts==0.1.7` pins torch 2.6 / transformers; install torch from
  the CUDA or CPU index first (see `engine/Dockerfile`, `TORCH_INDEX`).
- GPU: `docker run --gpus all`; ~4 s per sentence on a 6 GB laptop GPU.
  Whisper runs on the CPU so both models fit in 6 GB.
- The model object holds the current voice (`model.conds`): synthesis and
  enrollment are serialized with a lock in `engine.py`.
- Voices from engine 0.1 (OpenVoice) are not compatible: regenerate them.
- Style → (`exaggeration`, `cfg_weight`) table in `engine.py`; there is no
  direct speaking-rate control.

### App configuration

| Env var | Flag | Meaning |
|---|---|---|
| `FRIENDLY_VOICE_DATA` | `--data-dir` | data folder, default `.friendly-voice` **relative to the cwd** (git-ignored) |
| `FRIENDLY_VOICE_ENGINE_URL` | `--engine-url` | engine URL (http-engine builds only) |
| `FRIENDLY_VOICE_ME` | `--my-name` | listener's name used in messages |
| `FRIENDLY_VOICE_PLAYER` | `--player` | command that plays generated audio (file path appended); `windows` = built-in PowerShell `SoundPlayer` |

Data folder: `speakers.json`, `samples.json`, `audio/<speaker-id>/<sample-id>.{wav,flac}`,
`voices.json`, `schedules.json`, `outbox/` (WAV + note with text, language
and tone). A sample can carry `<file>.txt` (transcript) next to it.

## Limits

| Limit | Value | Where |
|---|---|---|
| Sample duration | 2 s – **180 s** (longer: split into pieces) | `domain/src/audio.rs` `SampleQualityPolicy` |
| Sample rate | ≥ 16 kHz | same |
| Clipping | ≤ 1 % of samples at full scale | same |
| Loudness | RMS ≥ 0.005 (not silent) | same |
| Voice material | ≥ 1 sample and ≥ 10 s in total | `domain/src/voice.rs` `EnrollmentPolicy` |
| Input formats | WAV (PCM/float), FLAC (any bit depth, mixed to mono) | `infrastructure/src/audio/` |
| Message text | ≤ 600 chars; engine chunks ≤ 220 chars | `domain/src/message.rs`, `engine/.../text.py` |
| Pitch shift | ±6 semitones | `domain/src/tone.rs` |
| Tones | natural, warm, cheerful, calm, firm, whisper | `domain/src/tone.rs` |
| Languages | 23 (`fv languages`); themes only in Portuguese | `domain/src/language.rs` |
| Gender boundary | mean pitch 160 Hz | `domain/src/voice.rs` `VoiceGender` |
| Schedule interval | ≥ 1 min; rules that never fire are refused | `domain/src/schedule.rs` |
| Schedule look-ahead | 8 days | same |
| Engine payload | ≤ 64 MiB per audio | `infrastructure/src/voice/http.rs` |

Ethical limits (ADR 0004): no person without consent; personal, private use
only; data stays local (data folder + engine voices folder). Chatterbox is
MIT and watermarks its output (Perth). Do not swap in non-commercial models
(why XTTS and F5-TTS-style CC-BY-NC weights are out: ADR 0006/0007).
`praat-parselmouth` is GPL-3.0.

## How the project was built

Agile, one-week sprints, MoSCoW + story points
([`docs/agile/sprints.md`](docs/agile/sprints.md)):

| Sprint | Increment |
|---:|---|
| 0 | Layered Cargo workspace, CI, backlog, Definition of Done |
| 1 | Pure domain: consent, IPA phonetics, accent rules and derivation |
| 2 | Samples, DSP, pt-BR G2P, `AccentAnalyzer`, HTTP engine port |
| 3 | Messages, pronunciation preview, intensity, schedules end to end in the CLI |
| 4 | Tone of voice, `bootstrap` crate, JNI bridge, Android app + CI/CD |
| 5 | Real timbre cloning with OpenVoice V2, engine server, English docs |
| 6 | FLAC input, English CLI and output |
| 7 | Zero-shot cloning with Chatterbox; language per person; gender detection; accent rules removed |

## Commands

```bash
cargo build --release --features http-engine    # binary: target/release/friendly-voice(.exe); README aliases it as `fv`
cargo test --workspace --all-features           # unit, use-case and acceptance tests (real binary)
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo fmt --all
(cd engine && pytest -q)                        # integration tests need chatterbox-tts
scripts/engine-e2e.sh                           # CLI ⇄ running engine
docker build -t friendly-voice-engine engine/ && docker run --gpus all -p 8020:8020 -v fv-voices:/data friendly-voice-engine
scripts/build-android.sh [--release]            # needs cargo-ndk + ANDROID_NDK_HOME
```

CI (`.github/workflows`): `ci.yml` (fmt, clippy, tests), `engine.yml` (engine
tests with the real model on CPU, e2e, Docker build), `android.yml` (3 ABIs,
Termux CLI, APKs, lint; GitHub Release on `v*`).

## Definition of Done (summary)

Acceptance criteria covered by automated tests · fmt/clippy/tests/pytest green
· dependency rule holds · English interface/code/docs · README/ADR/backlog
updated when behaviour changes · voice stories checked by listening to real
recordings · reviewed in a PR. Full text:
[`docs/agile/definition-of-done.md`](docs/agile/definition-of-done.md).

<!-- claude-reflect: learned rules (saved from the band; edit freely) -->
## Learned rules
- When lengthening vowels across word boundaries, lengthen only the stressed (tônica) vowel, not unstressed ones.
<!-- end claude-reflect -->
