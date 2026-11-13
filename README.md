# Friendly Voice 🗣️💛

Positive, motivational messages **in the voice of someone you love**. Ask for
a message right now or schedule them with your own rules.

Written in **Rust**, following **Clean Architecture** and **SOLID**, and
developed in an **agile** way (backlog, sprints, Definition of Done — see
[`docs/agile`](docs/agile)). The interface (CLI and Android), code and
documentation are in English. Each person has a **language** (Brazilian
Portuguese by default, or any of 23 languages): the recordings are in that
language and the messages are spoken in it.

## Highlights

* **Voice cloning from audio files** (WAV or FLAC, any bit depth, mono or
  stereo) with [Chatterbox Multilingual](https://github.com/resemble-ai/chatterbox)
  (MIT license), zero-shot — no training — behind a small self-hosted server
  ([`engine/`](engine)). Consent of the person is mandatory.
* **The voice comes from all the recordings**: the timbre is averaged over
  every sample, and the 10 s of speech closest to it are what the model
  imitates for intonation, rhythm and accent.
* **Female or male voice detected** from the recordings (mean pitch).
* **Language per person** (`--language pt-BR`, `en`, `es`, `fr`, `it`, `de`,
  `ja`... — `fv languages` lists them).
* **Tone of voice per message**: natural, warm, cheerful, calm, firm or
  whisper, plus a fine pitch adjustment (±6 semitones). Each theme suggests an
  initial tone (e.g. calm → calm, celebration → cheerful) that the user can change.
* Messages by theme (motivation, confidence, gratitude, calm, celebration,
  good morning; written in Portuguese), personalized with your name, or any
  free text in the person's language.
* Schedules: daily, on weekdays, every N minutes inside a time window, once;
  quiet hours; a single catch-up firing if the app was off.
* Android app built from the same Rust core, with CI/CD.

## How the voice is produced

```
recordings ─► Rust DSP ─► mean pitch ─► female / male, prosody
recordings ─► engine /v1/voices ─► speaker embedding (mean of all recordings)
                                 + reference window closest to it ─► voice_id

text + language + tone ─► engine /v1/synthesize ─► Chatterbox Multilingual,
    sentence by sentence (a cut-off take is retried) ─► pitch shift, gain ─► WAV
```

The model reads the text itself, in the person's language, and imitates the
person's way of speaking from the reference audio. See
[`docs/adr/0007-zero-shot-cloning-with-chatterbox.md`](docs/adr/0007-zero-shot-cloning-with-chatterbox.md).

## Getting started

### 1. Voice engine

With Docker (an NVIDIA GPU makes it ~10× faster, but the CPU works):

```bash
docker build -t friendly-voice-engine engine/
docker run --gpus all -p 8020:8020 -v fv-voices:/data friendly-voice-engine
```

Without Docker, see [`engine/README.md`](engine/README.md).

### 2. App (CLI)

```bash
cargo build --release --features http-engine
alias fv=./target/release/friendly-voice
export FRIENDLY_VOICE_ENGINE_URL=http://localhost:8020
export FRIENDLY_VOICE_ME="Evandro"       # your name in the messages
export FRIENDLY_VOICE_PLAYER="aplay"     # optional: play the audio (afplay on macOS, windows on Windows)

# Register the person (consent is mandatory; the language defaults to pt-BR)
fv person add --name "Vó Lúcia" --relationship avó --consent-by "Lúcia"
#   Person registered: Vó Lúcia (id: spk-..., speaks Portuguese)
fv person add --name "Grandpa Joe" --consent-by "Joe" --language en-US

# Upload .wav or .flac recordings (2 s – 180 s each, ≥ 16 kHz).
# A transcript is optional (--transcript, or recording.txt next to the file).
fv sample add --person spk-... --audio recording1.flac
fv sample add --person spk-... --audio recording2.wav

# Generate the voice: cloned from all the samples
fv voice generate --person spk-...
#   Language: Portuguese (pt-BR)  ·  female voice (mean pitch 246 Hz)

fv speak --person spk-... --theme good-morning
fv speak --person spk-... --text "Respira fundo, vai dar tudo certo."
fv speak --person spk-... --theme confidence --tone warm --pitch-shift -1
fv tones                                                   # list the tones of voice
fv themes                                                  # list the message themes
fv languages                                               # list the languages

# Schedule
fv schedule create --person spk-... --daily 07:30 --theme good-morning
fv schedule create --person spk-... --weekly mon,wed,fri@12:00 --theme confidence
fv schedule create --person spk-... --every 120 --between 09:00-18:00 --theme calm --quiet 12:00-13:00
fv schedule create --person spk-... --at "2026-12-24 20:00" --text "Feliz Natal!" --tone cheerful
fv schedule list
fv schedule run --watch      # keep it running (or call it from cron without --watch)
```

Themes, tones and weekdays accept English or Portuguese values (`good-morning`
or `bom-dia`, `warm` or `carinhoso`, `mon` or `seg`). The former Portuguese
command and option names (`pessoa adicionar --nome`, `falar --tema`, ...) are
still accepted as hidden aliases, so existing scripts keep working.

Generated audio files and a note with the text, language and tone are written
to `.friendly-voice/outbox/`.

Without `--features http-engine` / `FRIENDLY_VOICE_ENGINE_URL`, the app uses a
**local prosody sketch engine**: it renders audio with the person's rhythm and
melody (useful for development and tests) but **not** their voice.

## Android app

The same Rust core runs on Android: `crates/mobile` exposes the use cases via
JNI and `android/` is the Kotlin app — register the person (with the
language), upload `.wav`/`.flac` samples, generate the voice, ask for a
message choosing **the tone**, and schedule messages (fired by WorkManager
every 15 minutes, with a notification and audio). Point it at the voice engine
in its settings to use the cloned voice.

```bash
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android
cargo install cargo-ndk
export ANDROID_NDK_HOME=/path/to/ndk
scripts/build-android.sh            # debug APK in android/app/build/outputs/apk/
scripts/build-android.sh --release  # release APK (signed if ANDROID_KEYSTORE_* are set)
```

## CI/CD

| Workflow | When | What it does |
|---|---|---|
| `ci.yml` | every push/PR | `fmt`, `clippy -D warnings`, all Rust tests |
| `engine.yml` | every push/PR | engine unit + integration tests with the real model, CLI ⇄ engine end-to-end, Docker image build |
| `android.yml` | every push/PR | tests the mobile bridge, builds the Rust library for `arm64-v8a`, `armeabi-v7a` and `x86_64` (cargo-ndk), the CLI for Android/Termux (arm64), the debug and release APKs (Gradle) and lint; publishes everything as artifacts |
| `android.yml` → `release` | tag `v*` | creates the GitHub Release with the APKs and the arm64 binary |

To sign the release APK, add the secrets `ANDROID_KEYSTORE_BASE64` (keystore
in base64), `ANDROID_KEYSTORE_PASSWORD`, `ANDROID_KEY_ALIAS` and
`ANDROID_KEY_PASSWORD`. Without them the release APK is unsigned.

## Architecture

```
crates/
├── domain/          pure rules: Speaker, Language, AudioClip, voice (gender, prosody), tone, schedules
├── application/     use cases + ports (traits)
├── infrastructure/  WAV/FLAC, DSP, JSON, voice engines, catalog, delivery
├── bootstrap/       shared composition root (CLI and Android)
├── cli/             command line interface
└── mobile/          native library for Android (JNI → use cases)
android/             Android app (Kotlin) using the native library
engine/              voice cloning server (Python: Chatterbox Multilingual)
```

Details in [`docs/architecture.md`](docs/architecture.md) and decisions in
[`docs/adr`](docs/adr).

## Development

```bash
cargo test --workspace --all-features    # unit, use case and acceptance tests
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo fmt --all
(cd engine && pytest -q)                 # engine tests (integration ones need chatterbox-tts)
scripts/engine-e2e.sh                    # CLI ⇄ running engine, end to end
```

## Responsible use

Only use voices of people who gave their consent. Data stays local, in the
`--data-dir` folder and in the engine's voices folder. The app is meant for
personal, private messages. Every generated audio carries Resemble AI's
imperceptible Perth watermark, so it can be identified as synthetic.

## Third-party models

| Model | Use | License |
|---|---|---|
| Chatterbox Multilingual (Resemble AI) | voice cloning and speech, 23 languages | MIT (code and weights) |
| faster-whisper `small` *(optional)* | transcription | MIT |
| Praat, via `praat-parselmouth` | pitch shift (PSOLA) | GPL-3.0 |
