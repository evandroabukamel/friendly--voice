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

## Android bridge (JNI)

`app.friendlyvoice.NativeBridge` (Kotlin) → `crates/mobile/src/jni_bridge.rs`
→ `crates/mobile/src/api.rs`. Every function returns
`{"ok":true,"data":...}` or `{"ok":false,"error":"..."}`; Java `null` strings
become `None`; panics never cross into the JVM.

| Kotlin `external fun` (all return `String`) | `data` |
|---|---|
| `configure(dataDir: String, engineUrl: String?)` | `{engine}` |
| `catalog()` | themes, tones, languages |
| `listSpeakers()` | `[speaker item]` (revoked people excluded) |
| `registerSpeaker(name: String, relationship: String?, consentBy: String?, language: String?, listenerName: String?)` | the stored speaker plus the speaker item fields |
| `setListenerName(speakerId: String, listenerName: String?)` | speaker item |
| `revokeConsent(speakerId: String, revokedBy: String?)` | `null` |
| `deleteSpeaker(speakerId: String)` | `null` |
| `addSample(speakerId: String, wav: ByteArray, transcript: String?)` | sample item |
| `listSamples(speakerId: String)` | `[sample item]` |
| `buildVoice(speakerId: String)` | `{language, gender, prosody, sample_count, total_audio_secs}` |
| `speak(speakerId: String, category: String?, text: String?, tone: String?, pitchShift: Float, listener: String?)` | `{text, tone, language, audio_path}` |
| `createSchedule(requestJson: String)` | schedule item |
| `updateSchedule(id: String, requestJson: String)` | schedule item |
| `setScheduleEnabled(id: String, enabled: Boolean)` | schedule item |
| `listSchedules()` | `[schedule item]` |
| `removeSchedule(id: String)` | `null` |
| `runDueSchedules(listener: String?)` | `[{schedule_id, ok, text, tone, audio_path}]` (or `{schedule_id, ok: false, error}`) |

* **speaker item:** `{id, name, relationship, language, consent_granted_by,
  listener_name, voice_ready, gender, sample_count, total_sample_secs}`.
* **sample item:** `{id, duration_secs, transcript, mean_pitch_hz}`.
* **schedule request** (create/update): `{speaker_id, kind:
  "daily"|"weekly"|"interval"|"once", at ("HH:MM", or "YYYY-MM-DD HH:MM" for
  once), days ("mon,tue,..."), every_minutes, within ("HH:MM-HH:MM"), quiet
  ("HH:MM-HH:MM"), category, text, tone, pitch_shift}`.
* **schedule item:** the stored schedule (`id, speaker_id, recurrence,
  content, tone, quiet_hours, enabled, created_at, last_fired_at,
  active_since`) plus `speaker_name` (null when the person was deleted),
  `voice_available` (false when the person is revoked or deleted), `next_due`
  ("dd/MM HH:mm", null when disabled), `time` ("HH:mm", null for interval),
  `tone_label` and `request`, the schedule request above; sending it back to
  `updateSchedule` yields the same schedule. Turning on a schedule whose voice
  is unavailable fails ("edit the schedule to choose another voice");
  `updateSchedule` can move it to another voice, keeping its on/off state.

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
