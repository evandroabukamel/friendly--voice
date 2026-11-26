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
| `friendly_voice_bridge` (`flutter_app/rust`) | Mobile interface: typed flutter_rust_bridge API (opaque `Session`, `AppFailure`) | `bootstrap`, `flutter_rust_bridge` |

The Flutter app (`flutter_app/`, Android and iOS) is just another
presentation layer: Riverpod state → Dart ports (`VoiceCore`, ...) →
`RustVoiceCore` → the bridge → use cases. No business rule is rewritten in
Dart; the Dart side has its own clean layers (domain ports, application,
infrastructure adapters, atomic-design presentation). See ADR 0009, which
supersedes the Kotlin/JNI app of ADR 0005.

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

## Synthesized-speech cache

Every engine sits behind `CachedSpeechSynthesizer` (ADR 0008), wired in
`bootstrap`. A talk is stored at `<data>/cache/speech/<speaker-id>/<key>.wav`,
the key being a stable hash of the engine kind, the whole `VoiceProfile`
(including `built_at`, the voice version) and the whole `SpeechScript`
(final text, language, tone style, pitch shift, prosody). The same talk is
never asked from the engine twice; a rebuilt voice or another tone misses.
Files are written to a temp file and renamed, so a half file is never served.
Regenerating a voice deletes that person's cached files (`SpeechCache::clear`);
changing samples, the listener name or the language does not.

## Revoking consent and deleting a person (US-15)

`RevokeConsent` marks the person revoked first (`Speaker.consent_revoked =
{by, at}`), so every later use is refused (`DomainError::ConsentRevoked`):
adding samples, building the voice, speaking (now or scheduled), creating or
editing a schedule with that person, turning their schedules on. Then
`VoiceDataEraser` runs idempotent steps in a crash-safe order: switch off the
person's schedules (kept), clear the speech cache, delete the audio files,
delete the sample entries, ask the engine to forget the voice
(`VoiceCloningEngine::forget`, `DELETE /v1/voices/{id}`) and only then delete
the profile, which holds the engine's voice id. If the voice server is
unreachable the error says to run it again; a re-run finishes the cleanup.
The person stays as a revoked record and is no longer listed; using them again
needs a new registration with a new consent.

`DeleteSpeaker` does the same and finally removes the record; an unknown id
is fine.

## Schedules

`Schedule` fires the next occurrence after its anchor, the latest of
`created_at`, `last_fired_at` and `active_since`. If the app was off it fires
once (single catch-up, US-08). Two operations move the anchor to "now" so
nothing missed is replayed:

* `SetScheduleEnabled`: the alarm switch. Idempotent; re-enabling only fires
  future occurrences; a one-time message whose date passed cannot be
  re-enabled (edit it instead).
* `UpdateSchedule`: edit an alarm. Replaces person, rule, content, tone and
  quiet hours, keeping the id and the on/off state. An identical rule is a
  no-op.

## Listener name

The messages call the listener by name, chosen in this order
(`SpeakMessage`, for "speak now" and scheduled runs):

1. a name for this one message (`SpeakRequest.listener_override`, CLI
   `speak --listener-name`);
2. the person's own name for the listener, `Speaker.listener_name`
   ("Filhote", "Campeão");
3. the caller's name: the app's Settings name or CLI `--my-name`.

## Flutter bridge (`flutter_app/rust/src/api`)

Dart holds an opaque `Session` (one data folder and engine); every method
returns its typed value or throws `AppFailure { message }`.

| `Session` method | Returns |
|---|---|
| `open(dataDir, engineUrl?)` · `engineName` | the session · engine label |
| `catalog()` (free function) | themes, tones, languages, max pitch shift |
| `registerSpeaker(name, relationship?, consentBy?, language?, listenerName?)` | `SpeakerSummary` |
| `listSpeakers()` | `[SpeakerSummary]` (revoked people excluded) |
| `setListenerName(speakerId, listenerName?)` | `SpeakerSummary` (blank clears) |
| `deleteSpeaker(speakerId)` | — (idempotent; their schedules are switched off and kept) |
| `addSample(speakerId, audio, transcript?)` · `listSamples(speakerId)` | `SampleSummary` · `[SampleSummary]` |
| `buildVoice(speakerId)` | `VoiceSummary` |
| `speak(speakerId, theme?, text?, toneStyle?, pitchShift, listener?)` | `SpokenMessage` (WAV path) |
| `createSchedule(request)` · `updateSchedule(id, request)` · `setScheduleEnabled(id, enabled)` | `ScheduleSummary` |
| `listSchedules()` · `removeSchedule(id)` | `[ScheduleSummary]` · — |
| `runDueSchedules(listener?)` · `upcomingAlarms(limit)` | `[DueMessage]` · `[Alarm]` (epoch ms, soonest first) |

* **SpeakerSummary:** id, name, relationship, language, voiceReady, gender,
  consentBy, listenerName, sampleCount, totalSampleSecs.
* **ScheduleRequest:** speakerId, kind (daily | weekly | interval | once),
  at (`HH:MM`, or `YYYY-MM-DD HH:MM` for once), days (`mon,fri`),
  everyMinutes, within and quiet (`HH:MM-HH:MM`), theme, text, tone,
  pitchShift.
* **ScheduleSummary:** id, speakerId, speakerName (null once the person was
  deleted), voiceAvailable, enabled, time (`HH:MM`, null for intervals),
  toneLabel, nextDue (`dd/mm HH:MM`, null when off) and `request`: sending it
  back to `updateSchedule` yields the same schedule. Switching on a schedule
  whose voice is unavailable fails; `updateSchedule` can move it to another
  voice, keeping its on/off state.

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
