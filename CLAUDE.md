# CLAUDE.md — Friendly Voice

Positive, motivational messages spoken **in the voice and the accent of
someone you love**, on demand or on a schedule. Rust core (Clean
Architecture), a Python voice-cloning server, a CLI and an Android app.

Start here, then follow the links: [`README.md`](README.md) (usage),
[`docs/architecture.md`](docs/architecture.md) (layers, data flow, accent
model), [`docs/engine-api.md`](docs/engine-api.md) (HTTP contract),
[`docs/adr/`](docs/adr) (decisions), [`docs/agile/`](docs/agile) (backlog,
sprints, Definition of Done).

## Language rules (non-negotiable)

- **User-facing text is Portuguese** (pt-BR): CLI commands/flags (`pessoa`,
  `amostra adicionar`, `--texto`), error messages, themes, tones, Android UI.
- **Code, comments, docs, commits, ADRs are English** (US-21).
- Error messages must be clear and actionable ("divida em trechos", "grave
  com volume menor").

## Product requirements

Vision: *hear a loved one say "bom dia" before work and "respira fundo" in the
afternoon.* Full stories with Gherkin criteria:
[`docs/agile/product-backlog.md`](docs/agile/product-backlog.md).

| Epic | Delivered stories |
|---|---|
| E1 People & consent | US-01 register a person **only with recorded consent** (name + who authorized + timestamp) |
| E2 Cloning with accent | US-02 upload samples · US-03 letter/sound pronunciation · US-04 word endings & junctions · US-06 pronunciation preview · US-09 accent intensity · US-10 traits outside the catalog · US-11 timbre cloning (OpenVoice V2) · US-24 FLAC input |
| E3 Messages | US-05 message now (theme or free text, personalized with the listener's name) · US-18 tone per message |
| E4 Schedules | US-07 rules (daily, weekdays, every N min in a window, once) + quiet hours · US-08 fire due messages exactly once, single catch-up |
| E5 Android | US-19 APK built in CI, release on `v*` tags · US-20 full flow on the phone with notifications |
| E6 Project | US-21 English docs and comments |

Open backlog (respect when touching related code): US-15 revoking consent
must delete samples and voice (Must), US-22 audio watermark, US-17 no repeated
messages, US-16 MP3/OGG/M4A, US-12/13 rhythm and intonation, US-23 base voice.

## How it works

```
recordings ─► SampleQualityPolicy ─► DSP (pitch, pauses) ─┐
transcript (.txt | --texto | /v1/transcribe) ─► pt-BR G2P ─► citation form
heard IPA  (.ipa | --fonemas | /v1/phonemes) ─────────────► AccentAnalyzer ─► AccentProfile
recordings ─► engine /v1/voices ─► OpenVoice speaker embedding (timbre) ─► voice_id

new text ─► G2P ─► AccentRenderer (profile × intensity) ─► words (IPA, stress, pauses)
        ─► + VoiceTone ─► engine /v1/synthesize: Piper pt-BR speaks the phonemes
        ─► OpenVoice converts the timbre ─► WAV in .friendly-voice/saida/
```

Key idea (ADR 0002/0006): **the accent lives in the Rust domain** as ordered,
explainable phonological rules and reaches the engine as explicit phonemes.
The engine never re-reads text, so neural models cannot normalize the accent
away. Timbre is the only thing delegated to a neural model.

Accent learning: weighted-Levenshtein alignment of citation × heard phonemes,
catalog walked in order (realized rules feed later ones: "noite" → /nojtʃi/),
a rule is adopted with ≥ 2 opportunities and ≥ 50 % rate (rate = confidence),
recurring off-catalog substitutions become `learned:` rules, and the profile
is labelled with the nearest regional preset (Jaccard ≥ 0.5). Details:
[`docs/architecture.md#accent-model`](docs/architecture.md).

## Technical specification

### Layout and dependency rule (ADR 0001)

```
crates/domain          pure rules; depends on no project crate (chrono, serde, thiserror only)
crates/application     use cases (one per file) + ports (traits); depends only on domain
crates/infrastructure  adapters: WAV/FLAC decode, DSP, pt-BR G2P, JSON repos, HTTP engine, sketch engine, delivery
crates/bootstrap       the ONLY composition root, shared by CLI and Android
crates/cli             `friendly-voice` binary (clap), Portuguese commands
crates/mobile          JNI bridge, JSON in/out, panic-safe (ADR 0005)
android/               Kotlin app — presentation only, no business rules
engine/                Python FastAPI server: Piper pt-BR + OpenVoice V2
```

- New business rules go in `domain`; new I/O goes behind a port in
  `application/src/ports.rs` and is wired in `bootstrap`. Never import
  infrastructure from application or domain.
- Ports: `VoiceCloningEngine`, `SpeechSynthesizer`, `Transcriber` (optional),
  `PhoneRecognizer` (optional), one repository per aggregate. Use cases take
  `Arc<dyn Port>`; tests use the in-memory fakes in `application/src/fakes.rs`.
- Audio format is detected by **magic bytes** (`RIFF…WAVE` / `fLaC`), never by
  extension (`infrastructure/src/audio/format.rs`). FLAC is converted to WAV
  before going to the engine, so the API contract stays WAV-only.
- New accents = new catalog data in `domain/src/accent/catalog.rs`, not
  analyzer changes (OCP).

### Voice engines

| Build / config | Engine | Result |
|---|---|---|
| default build, no URL | local prosody **sketch** | person's rhythm and melody, **not** the timbre — for dev/tests |
| `--features http-engine` + `--motor-url` / `FRIENDLY_VOICE_ENGINE_URL` | `HttpVoiceEngine` → `engine/` | cloned timbre + accent + tone |

Engine endpoints (contract in [`docs/engine-api.md`](docs/engine-api.md)):
`POST /v1/voices`, `POST /v1/synthesize`, optional `POST /v1/transcribe`
(faster-whisper `small`, extra `[transcribe]`) and `POST /v1/phonemes`
(wav2vec2-xlsr-53-espeak, extra `[phonemes]`), `GET /health` (reports which
optional endpoints are installed). Optional endpoints answer **501** when not
installed; the app then stores the sample **without accent evidence** instead
of failing. Engine env vars: `FV_MODELS_DIR`, `FV_VOICES_DIR`,
`FV_ENGINE_HOME`, `FV_DEVICE`, `FV_TAU` (see `engine/friendly_voice_engine/config.py`).

Engine gotchas:
- `/v1/phonemes` needs the `phonemizer` Python package **and** the
  `espeak-ng` system library besides `transformers`.
- transformers checks optional libraries at import time: after installing
  one into a running server, **restart** the process/container.
- First calls download models (Whisper, wav2vec2) and are slow.

### App configuration

| Env var | Flag | Meaning |
|---|---|---|
| `FRIENDLY_VOICE_DATA` | `--dados` | data folder, default `.friendly-voice` **relative to the cwd** (git-ignored) |
| `FRIENDLY_VOICE_ENGINE_URL` | `--motor-url` | engine URL (http-engine builds only) |
| `FRIENDLY_VOICE_ME` | `--meu-nome` | listener's name used in messages |
| `FRIENDLY_VOICE_PLAYER` | `--player` | command that plays generated audio (file path appended); `windows` = built-in PowerShell `SoundPlayer` |

Data folder: `speakers.json`, `samples.json`, `audio/<speaker-id>/<sample-id>.{wav,flac}`,
voices, schedules, `saida/` (WAV + note with text, IPA and SSML). A sample can
carry `<file>.txt` (transcript) and `<file>.ipa` (heard phonemes) next to it.

## Limits

| Limit | Value | Where |
|---|---|---|
| Sample duration | 2 s – **180 s** (longer: split into pieces) | `domain/src/audio.rs` `SampleQualityPolicy` |
| Sample rate | ≥ 16 kHz | same |
| Clipping | ≤ 1 % of samples at full scale | same |
| Loudness | RMS ≥ 0.005 (not silent) | same |
| Input formats | WAV (PCM/float), FLAC (any bit depth, mixed to mono) | `infrastructure/src/audio/` |
| Transcript | required — `.txt`, `--texto`, or a configured transcriber | `AddVoiceSample` |
| Message text | ≤ 600 chars | `domain/src/message.rs` |
| Pitch shift | ±6 semitones | `domain/src/tone.rs` |
| Tones | natural, carinhoso, animado, calmo, firme, sussurrado | `domain/src/tone.rs` |
| Accent intensity | 0 – 1 | `pronuncia`, `falar --intensidade` |
| Schedule interval | ≥ 1 min; rules that never fire are refused | `domain/src/schedule.rs` |
| Schedule look-ahead | 8 days | same |
| Engine payload | ≤ 64 MiB per audio | `infrastructure/src/voice/http.rs` |
| Language | pt-BR only (G2P, catalog, models) | — |

Ethical limits (ADR 0004): no person without consent; personal, private use
only; data stays local (data folder + engine voices folder). Piper's
`pt_BR-faber-medium` inherits the Blizzard 2013 license — review before any
commercial distribution. Do not swap in non-commercial models (why XTTS was
rejected: ADR 0006).

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
| 6 | FLAC input |

## Commands

```bash
cargo build --release --features http-engine    # binary: target/release/friendly-voice(.exe); README aliases it as `fv`
cargo test --workspace --all-features           # unit, use-case and acceptance tests (real binary)
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo fmt --all
(cd engine && pytest -q)                        # integration tests need downloaded weights
scripts/engine-e2e.sh                           # CLI ⇄ running engine
docker build -t friendly-voice-engine engine/ && docker run -p 8020:8020 -v fv-voices:/data friendly-voice-engine
scripts/build-android.sh [--release]            # needs cargo-ndk + ANDROID_NDK_HOME
```

CI (`.github/workflows`): `ci.yml` (fmt, clippy, tests), `engine.yml` (engine
tests with real models, e2e, Docker build), `android.yml` (3 ABIs, Termux CLI,
APKs, lint; GitHub Release on `v*`).

## Definition of Done (summary)

Acceptance criteria covered by automated tests · fmt/clippy/tests/pytest green
· dependency rule holds · Portuguese UI text, English code/docs · README/ADR/
backlog updated when behaviour changes · voice stories checked by listening to
real recordings · reviewed in a PR. Full text:
[`docs/agile/definition-of-done.md`](docs/agile/definition-of-done.md).
