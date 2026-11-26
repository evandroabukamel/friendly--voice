# Friendly Voice 🗣️💛

Positive, motivational messages **in the voice of someone you love**. Ask for
a message right now or schedule them with your own rules.

Written in **Rust**, following **Clean Architecture** and **SOLID**, and
developed in an **agile** way (backlog, sprints, Definition of Done — see
[`docs/agile`](docs/agile)). The interface (CLI and mobile app), code and
documentation are in English. Each person has a **language** (Brazilian
Portuguese by default, or any of 23 languages): the recordings are in that
language and the messages are spoken in it.

## Highlights

* **Voice cloning from audio files** (WAV, FLAC, MP3, M4A or OGG; mono or
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
* Mobile app for Android and iOS (Flutter) on the same Rust core, with CI/CD.

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
# How this person calls you (wins over FRIENDLY_VOICE_ME / --my-name)
fv person add --name "Mãe" --consent-by "Mãe" --listener-name "Filhote"

# Upload .wav, .flac, .mp3, .m4a or .ogg recordings (2 s – 180 s each, ≥ 16 kHz;
# the format is read from the content, not the extension).
# A transcript is optional (--transcript, or recording.txt next to the file).
fv sample add --person spk-... --audio recording1.flac
fv sample add --person spk-... --audio recording2.wav

# Generate the voice: cloned from all the samples
fv voice generate --person spk-...
#   Language: Portuguese (pt-BR)  ·  female voice (mean pitch 246 Hz)

fv speak --person spk-... --theme good-morning
fv speak --person spk-... --text "Respira fundo, vai dar tudo certo."
fv speak --person spk-... --theme confidence --tone warm --pitch-shift -1
fv speak --person spk-... --theme celebration --listener-name "Campeão"   # name for this message only
fv tones                                                   # list the tones of voice
fv themes                                                  # list the message themes
fv languages                                               # list the languages

# How this person calls you (omit --listener-name to clear it)
fv person set-listener-name --person spk-... --listener-name "Filhote"
# Revoke consent: deletes recordings, voice (also on the server) and cached
# speech; the person's schedules are switched off. Re-adding needs new consent.
fv person revoke --person spk-... --by "Lúcia"
fv person delete --person spk-...                          # also removes the person

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
to `.friendly-voice/outbox/`. Every generated talk is also cached in
`.friendly-voice/cache/speech/<person>/`, so the same text, tone and voice is
never asked from the voice server again; regenerating the voice invalidates
it, and deleting the folder is always safe (ADR 0008).

Without `--features http-engine` / `FRIENDLY_VOICE_ENGINE_URL`, the app uses a
**local prosody sketch engine**: it renders audio with the person's rhythm and
melody (useful for development and tests) but **not** their voice.

## Mobile app (Android and iOS)

`flutter_app/` is the Flutter app (Flutter 3.50.1) for Android and iOS. It
runs the same Rust core through flutter_rust_bridge (`flutter_app/rust/`).
Three tabs in a bottom navigation bar, in a warm design whose background
follows the time of day (dawn, day, dusk, night) with large, light type:
**Reminders** (each with its time, switch, message, repeat and voice; tap to
edit the voice, theme or own words, **tone**, pitch, repeat and quiet hours,
or play it now), **Voices** (register a person with consent and language,
add recordings, record from the microphone, create the voice, delete it) and
**Settings** (your name and the app language: the Kotlin app's 21 languages,
or the phone's). Recordings may be WAV, FLAC, MP3, M4A or OGG, several at
once. There is no Save button: every edit saves itself. Schedules wake the phone with **exact
notifications** (Android 14+ asks for the exact-alarm permission; without it
they may be a few minutes late); the app speaks what is due when it opens,
when an alarm is tapped and every minute while open. The voice server is
compiled in, so users never type it: build with
`--dart-define=FRIENDLY_VOICE_ENGINE_URL=http://host:8020` (CI uses the
repository variable of the same name). Without it the app uses the local
sketch engine. Design: ADR 0009.

```bash
cd flutter_app
cargo install flutter_rust_bridge_codegen --version 2.13.0 cargo-expand  # only to regenerate bindings
flutter pub get
flutter test                      # native tests need: cargo build -p friendly_voice_bridge
flutter run                       # device/emulator; cargokit compiles the Rust bridge
flutter build apk --release       # Android (rustup targets for the ABIs)
flutter build ios --no-codesign   # iOS, on macOS (rustup iOS targets)
```

## CI/CD

| Workflow | When | What it does |
|---|---|---|
| `ci.yml` | every push/PR | `fmt`, `clippy -D warnings`, all Rust tests |
| `engine.yml` | every push/PR | engine unit + integration tests with the real model, CLI ⇄ engine end-to-end, Docker image build |
| `flutter.yml` | every push/PR | regenerated bridge bindings must match, Rust bridge tests, Dart format/analyze/tests (some against the real Rust core), Android release APK, unsigned iOS build (macOS) |
| `flutter.yml` → `release` | tag `v*` | creates the GitHub Release with the APK and the unsigned iOS app |

To sign the release APK, add the secrets `ANDROID_KEYSTORE_BASE64` (keystore
in base64), `ANDROID_KEYSTORE_PASSWORD`, `ANDROID_KEY_ALIAS` and
`ANDROID_KEY_PASSWORD`. Without them the release APK is signed with the debug
key. The iOS build is unsigned (signing needs an Apple Developer account).

## Architecture

```
crates/
├── domain/          pure rules: Speaker, Language, AudioClip, voice (gender, prosody), tone, schedules
├── application/     use cases + ports (traits)
├── infrastructure/  WAV/FLAC/MP3/M4A/OGG, DSP, JSON, voice engines, catalog, delivery
├── bootstrap/       shared composition root (CLI and the Flutter app)
└── cli/             command line interface
flutter_app/         Flutter app (Android + iOS): atomic design, Riverpod, Clean Architecture
└── rust/            flutter_rust_bridge API over the use cases (workspace member)
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
