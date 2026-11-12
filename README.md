# Friendly Voice 🗣️💛

Positive, motivational messages **in the voice — and the accent — of someone
you love**. Ask for a message right now or schedule them with your own rules.

Written in **Rust**, following **Clean Architecture** and **SOLID**, and
developed in an **agile** way (backlog, sprints, Definition of Done — see
[`docs/agile`](docs/agile)). The app targets Brazilian Portuguese speakers, so
its user interface (CLI and Android) is in Portuguese; code and documentation
are in English.

## Highlights

* **Voice cloning from audio files** with [OpenVoice V2](https://github.com/myshell-ai/OpenVoice)
  (MIT license), behind a small self-hosted server ([`engine/`](engine)).
  Consent of the person is mandatory.
* **Accent learned from the recordings**, covering:
  * **how certain letters are pronounced** — "tia" → *tchia*, "festa" → *feshta*,
    "porta" → *pohta* / *poɻta*, "rato" → *hato*, "mulher" → *muié*;
  * **word endings** — "leite" → *leiti*, "gato" → *gatu*, "falar" → *falá*;
  * **word junctions** — "os amigos" → *uzamigus*, "casa amarela" →
    *casamarela*, "mar azul" → *maɾazul*, "de outro" → *djoutro*.
* Explainable report: each phenomenon with the frequency measured in the
  samples, the closest regional accent (Carioca, Paulistano, Mineiro,
  Nordestino, Gaúcho, Caipira, European Portuguese) and personal traits
  discovered outside the catalog.
* **Tone of voice per message**: natural, warm (*carinhoso*), cheerful
  (*animado*), calm (*calmo*), firm (*firme*) or whisper (*sussurrado*), plus a
  fine pitch adjustment (±6 semitones). Each theme suggests an initial tone
  (e.g. *calma* → calm) that the user can change.
* Messages by theme (motivation, confidence, gratitude, calm, celebration,
  good morning), personalized with your name.
* Schedules: daily, on weekdays, every N minutes inside a time window, once;
  quiet hours; a single catch-up firing if the app was off.
* Android app built from the same Rust core, with CI/CD.

## How the voice is produced

```
recordings ──► OpenVoice tone color encoder ──► speaker embedding (timbre)
recordings + transcript ──► Rust AccentAnalyzer ──► accent profile (rules)

new text ─► G2P ─► AccentRenderer ─► accented phonemes ─► Piper pt-BR ─► base speech
                                                                            │
                         speaker embedding ─► OpenVoice tone color converter ┘ ─► cloned voice ─► tone (rate, pitch, gain)
```

The accent is decided by the Rust domain and sent as explicit phonemes, so the
base speech (Piper) says exactly what the person would say; OpenVoice then
converts the timbre to the person's voice. See
[`docs/adr/0006-openvoice-for-voice-cloning.md`](docs/adr/0006-openvoice-for-voice-cloning.md)
for why OpenVoice was chosen over XTTS.

## Getting started

### 1. Voice engine (timbre cloning)

```bash
cd engine
python -m venv .venv && . .venv/bin/activate
pip install torch --index-url https://download.pytorch.org/whl/cpu
pip install -e .
pip install --no-deps -r requirements-openvoice.txt
friendly-voice-engine download        # ~200 MB of weights, SHA-256 verified
friendly-voice-engine serve --port 8020
```

or with Docker: `docker build -t friendly-voice-engine engine/ && docker run -p 8020:8020 -v fv-voices:/data friendly-voice-engine`.
Details in [`engine/README.md`](engine/README.md).

### 2. App (CLI)

```bash
cargo build --release --features http-engine
alias fv=./target/release/friendly-voice
export FRIENDLY_VOICE_ENGINE_URL=http://localhost:8020
export FRIENDLY_VOICE_ME="Evandro"       # your name in the messages
export FRIENDLY_VOICE_PLAYER="aplay"     # optional: play the audio (afplay on macOS)

# Register the person (consent is mandatory)
fv pessoa adicionar --nome "Vó Lúcia" --relacao avó --consentimento-de "Lúcia"
#   Pessoa cadastrada: Vó Lúcia (id: spk-...)

# Upload .wav recordings (≥ 2 s, ≥ 16 kHz). Optionally, next to each file:
#    recording.txt → what was said      recording.ipa → what was heard (IPA)
fv amostra adicionar --pessoa spk-... --audio recording1.wav
fv amostra adicionar --pessoa spk-... --audio recording2.wav \
   --texto "Os amigos de verdade vão falar com você" \
   --fonemas "uz amiguʃ dʒi vehdadʒi vãw fala kũ vose"

# Generate the voice (timbre + accent + prosody)
fv voz gerar --pessoa spk-...

# Check the pronunciation and ask for messages
fv pronuncia --pessoa spk-... "Você é forte, mas às vezes esquece."
#   Forma de citação: /vose ɛ foɾte mas as vezes eskese/
#   Com sotaque:      /vose ɛ fohtʃi maz aʃ veziz eʃkesi/
fv falar --pessoa spk-... --tema bom-dia
fv falar --pessoa spk-... --texto "Respira fundo, vai dar tudo certo." --intensidade 0.5
fv tons                                                     # list the tones of voice
fv falar --pessoa spk-... --tema confianca --tom carinhoso --ajuste-tom -1

# Schedule
fv agenda criar --pessoa spk-... --diario 07:30 --tema bom-dia
fv agenda criar --pessoa spk-... --semanal seg,qua,sex@12:00 --tema confianca
fv agenda criar --pessoa spk-... --a-cada 120 --entre 09:00-18:00 --tema calma --silencio 12:00-13:00
fv agenda criar --pessoa spk-... --em "2026-12-24 20:00" --texto "Feliz Natal!" --tom animado
fv agenda criar --pessoa spk-... --diario 22:30 --tema gratidao --tom sussurrado
fv agenda listar
fv agenda executar --continuo      # keep it running (or call it from cron without --continuo)
```

Generated audio files and a note with the text, pronunciation and SSML are
written to `.friendly-voice/saida/`.

Without `--features http-engine` / `FRIENDLY_VOICE_ENGINE_URL`, the app uses a
**local prosody sketch engine**: it renders audio with the person's rhythm and
melody (useful for development and tests) but **not** their timbre.

## Android app

The same Rust core runs on Android: `crates/mobile` exposes the use cases via
JNI and `android/` is the Kotlin app — register the person, upload `.wav`
samples, generate the voice, ask for a message choosing **the tone**, and
schedule messages (fired by WorkManager every 15 minutes, with a notification
and audio). Point it at the voice engine in its settings to use the cloned
timbre.

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
| `engine.yml` | every push/PR | engine unit + integration tests with the real models, CLI ⇄ engine end-to-end, Docker image build |
| `android.yml` | every push/PR | tests the mobile bridge, builds the Rust library for `arm64-v8a`, `armeabi-v7a` and `x86_64` (cargo-ndk), the CLI for Android/Termux (arm64), the debug and release APKs (Gradle) and lint; publishes everything as artifacts |
| `android.yml` → `release` | tag `v*` | creates the GitHub Release with the APKs and the arm64 binary |

To sign the release APK, add the secrets `ANDROID_KEYSTORE_BASE64` (keystore
in base64), `ANDROID_KEYSTORE_PASSWORD`, `ANDROID_KEY_ALIAS` and
`ANDROID_KEY_PASSWORD`. Without them the release APK is unsigned.

## Architecture

```
crates/
├── domain/          pure rules: Speaker, AudioClip, phonetics, accent, tone, schedules
├── application/     use cases + ports (traits)
├── infrastructure/  WAV, DSP, pt-BR G2P, JSON, voice engines, catalog, delivery
├── bootstrap/       shared composition root (CLI and Android)
├── cli/             command line interface
└── mobile/          native library for Android (JNI → use cases)
android/             Android app (Kotlin) using the native library
engine/              voice cloning server (Python: Piper + OpenVoice V2)
```

Details in [`docs/architecture.md`](docs/architecture.md) and decisions in
[`docs/adr`](docs/adr).

## Development

```bash
cargo test --workspace --all-features    # unit, use case and acceptance tests
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo fmt --all
(cd engine && pytest -q)                 # engine tests (integration ones need the weights)
scripts/engine-e2e.sh                    # CLI ⇄ running engine, end to end
```

## Responsible use

Only use voices of people who gave their consent. Data stays local, in the
`--dados` folder and in the engine's voices folder. The app is meant for
personal, private messages.

## Third-party models

| Model | Use | License |
|---|---|---|
| OpenVoice V2 tone color converter (MyShell) | timbre cloning | MIT |
| Piper `pt_BR-faber-medium` | pt-BR base speech | voice dataset CC0, but the voice was fine-tuned from Piper's *lessac* voice, whose dataset ([Blizzard 2013](https://www.cstr.ed.ac.uk/projects/blizzard/2013/lessac_blizzard2013/license.html)) has its own license — review it before any commercial distribution. The model runs with `onnxruntime` (MIT); no Piper/espeak code is used. |
| faster-whisper *(optional)* | transcription | MIT |
| wav2vec2-xlsr-53-espeak-cv-ft *(optional)* | phone recognition | Apache-2.0 |
