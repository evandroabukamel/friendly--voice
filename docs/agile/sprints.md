# Sprints

One-week sprints. Every sprint delivers a **usable, tested increment**
(see the [Definition of Done](definition-of-done.md)).

| Sprint | Goal | Stories | Points |
|-------:|------|---------|-------:|
| 0 | Foundation: layered workspace, CI, initial backlog | — (enablers) | — |
| 1 | Pure domain: people, phonetics, accent model | US-01 (+ base of US-03/04) | 2 |
| 2 | Cloning: samples, accent learning and voice | US-02, US-03, US-04, US-10 | 24 |
| 3 | Messages and schedules end to end in the CLI | US-05, US-06, US-07, US-08, US-09 | 14 |
| 4 | Tone of voice per message and Android app with CI/CD | US-18, US-19, US-20 | 21 |
| 5 | Real timbre cloning (OpenVoice) and English documentation | US-11, US-21 | 11 |
| 6 | FLAC input; CLI commands and output in English | US-24, US-25, US-26 | 7 |
| 8 | Alarm switch and editing, listener name per voice, speech cache, revoke consent | US-15, US-30, US-31, US-32, US-33 | 13 |
| 9 | Flutter app for Android and iOS; Kotlin app removed | US-35 | 13 |

Average velocity (sprints 1–5): **~14 pts/sprint**.

## Sprint 0 — Foundation
* Cargo workspace with one crate per layer, and the dependency rule enforced
  by each crate's `Cargo.toml`.
* CI pipeline: `fmt`, `clippy -D warnings`, tests.
* Prioritized backlog and Definition of Done.

## Sprint 1 — Domain
**Increment:** domain library with all rules tested without I/O.
* `Speaker` entity with mandatory consent (US-01).
* IPA phonemes, IPA parser, words and utterances.
* Accent model: segmental, word-ending and juncture rules; ordered
  derivation; citation × heard alignment.

**Retrospective**
* 👍 Modelling the accent as ordered rules made the tests readable
  ("noite" → /nojtʃi/ proves rules feed each other).
* 👎 Combining word endings with junctions caused a conflict ("mar azul": the
  *r* was dropped before linking). **Action:** `blocked_before_vowel_word` context.

## Sprint 2 — Cloning with accent
**Increment:** `sample add` + `voice generate` working with the local engine.
* WAV decoding, DSP analysis (autocorrelation pitch, pauses).
* pt-BR G2P (citation form) with stress, digraphs, nasals and diphthongs.
* `AccentAnalyzer`: opportunities × realizations per phenomenon; discovers
  patterns outside the catalog; labels the closest regional preset.
* `VoiceCloningEngine` port + HTTP adapter for a neural engine (feature).

**Retrospective**
* 👍 In-memory fakes behind the ports made the use cases testable in ms.
* 👎 Junctions masked coda rules ("os amigos" → z counted as a failure of
  s → ʃ). **Action:** evidence "explained by a juncture" does not count against the rule.

## Sprint 3 — Messages and schedules
**Increment:** full flow in the CLI, covered by acceptance tests.
* Message catalog by theme, personalized with the listener's name.
* `speak`, `pronounce` (with intensity), `schedule create/list/remove/run`.
* Schedules with windows, weekdays, quiet hours and a single catch-up firing
  when the app was off.

**Retrospective**
* 👍 Acceptance tests on the real binary mirror the Gherkin criteria.
* 👎 The interval window started "off the clock" (09:30 instead of 09:00).
  **Action:** jump to the window start; regression test added.

## Sprint 4 — Tone of voice and Android
**Increment:** tone of voice chosen for each message (CLI and app) and the APK
built automatically by CI, with a release on every tag.
* `VoiceTone`: six styles + ±6 semitone shift, applied **on top of** the
  person's natural prosody; initial tone suggested by the theme.
* Composition root extracted to `crates/bootstrap` (reused by CLI/Android).
* `crates/mobile`: JSON API testable on the host + panic-safe JNI wrappers.
* Kotlin Android app (no business rules) and WorkManager for the schedules.
* `android.yml` workflow: cargo-ndk (3 ABIs), CLI for Termux, debug/release
  APK, lint, artifacts and a GitHub Release on `v*` tags.

**Retrospective**
* 👍 Clean Architecture paid off: Android only needed a new presentation
  layer; no use case changed.
* 👎 The Android SDK was not available in the development environment.
  **Action:** Kotlin was checked by compiling against `android-all` (API 34);
  the full APK is validated in CI (green on the first run).

## Sprint 5 — Timbre cloning and English documentation
**Increment:** messages spoken with the cloned timbre of the person, the
learned accent and the chosen tone; documentation and comments in English.
* License review: OpenVoice V2 (MIT) chosen over XTTS v2 (non-commercial
  only) — ADR 0006.
* `engine/`: FastAPI server; Piper pt-BR speaks the accented phonemes and the
  OpenVoice V2 tone color converter applies the person's timbre; weights
  downloaded with SHA-256 verification; Dockerfile.
* HTTP adapter sends phonemes per word with stress and pauses, reports the
  server's error details and treats a missing phone recognizer (501) as
  "no evidence" instead of failing.
* `engine.yml` workflow: engine tests with the real models, CLI ⇄ engine
  end-to-end test, Docker build.
* README, docs, ADRs and code comments rewritten in English (US-21).

**Retrospective**
* 👍 Keeping pronunciation in the domain made the engine simple: it never
  re-reads text, so the accent cannot be "normalized away".
* 👎 The tests use synthetic reference audio; perceptual quality with real
  recordings still has to be checked by listening. **Action:** add a
  listening checklist to the Definition of Done for voice stories.

## Sprint 6 — FLAC input
**Increment:** recordings can be uploaded as FLAC in the CLI and the Android app.
* `FlacDecoder` (pure-Rust `claxon`): any bit depth, mixed down to mono;
  optional ID3v2 tag skipped.
* `AudioFileDecoder` picks the decoder by magic bytes (`RIFF…WAVE` / `fLaC`),
  never by extension; adding a format does not touch the use cases (OCP).
* Stored files keep the original bytes with the detected extension; the HTTP
  adapter converts FLAC references to WAV, so the engine contract is unchanged.
* Tests generate FLAC with `flacenc`; also checked against ffmpeg-encoded
  files (24-bit stereo 48 kHz and 16-bit mono) through the OpenVoice engine.

## Sprint 6 (continued) — CLI commands in English
* Commands and options renamed to English (`person add --name`,
  `sample add --transcript`, `voice generate`, `speak --theme --tone
  --pitch-shift`, `schedule create --weekly`, `schedule run --watch`...).
* The Portuguese names are kept as hidden clap aliases, covered by an
  acceptance test, so existing scripts and cron jobs keep working.
* Themes, tones and weekdays accept English values too (`good-morning`,
  `warm`, `mon`).
* Then all output was translated (US-26): CLI help, messages and reports,
  errors from every layer, accent labels and rule descriptions, schedule
  descriptions, delivery notes and the Android screens. English names became
  canonical for themes and tones (`warm`, `good-morning`); the Portuguese ones
  remain accepted. The spoken message catalog stays in pt-BR, because the
  voice and pronunciation model are Portuguese.
* The default branch is now `main`.

## Sprint 7 — Zero-shot cloning, language and gender
**Increment:** the voice is cloned with Chatterbox Multilingual from all the
person's recordings; a language is chosen per person; female/male is detected.
* Listening review of the Piper + OpenVoice pipeline on 28 real recordings:
  the timbre was close but the speech sounded foreign and robotic, and no
  phoneme notation fixed it (ADR 0007).
* Prototype on a laptop GPU: 100 % of the takes understood by Whisper,
  speaker similarity ≈ 0.90 (her own recordings: 0.90–0.94).
* Removed: accent rules, G2P, phone recognizer, `pronounce`, `--intensity`,
  `--phonemes`; the transcript became optional.
* Added: `Language` (23 languages, `person add --language`, `languages`),
  `VoiceGender` from the mean pitch, speaker embedding averaged over every
  recording with the closest reference window, retries for cut-off takes.

## Sprint 8 — Alarm editing, listener name and speech cache
**Increment:** the app can switch alarms on and off and edit them, each voice
can call the listener by its own name, and generated speech is cached.
* US-30 `SetScheduleEnabled`: re-enabling only fires future occurrences
  (`Schedule.active_since`).
* US-31 `UpdateSchedule`: keeps id and on/off state; `listSchedules` returns a
  `request` that round-trips to `updateSchedule`.
* US-32 `Speaker.listener_name` (`person add --listener-name`,
  `setListenerName`), winning over `--my-name` / the app's Settings name.
* US-33 `CachedSpeechSynthesizer` around every engine (ADR 0008);
  `VoiceProfile.built_at` versions the voice.
* Bridge: `listSamples`, richer `listSpeakers` (sample totals, consent,
  listener name). US-16 MP3 import in progress on the Android side.
* US-15 `RevokeConsent` and `DeleteSpeaker` (idempotent, crash-safe
  cleanup), engine `DELETE /v1/voices/{id}`, speech cache cleared on voice
  regeneration, schedules report `voice_available`; CLI `person revoke`,
  `person delete`, `person set-listener-name`, `speak --listener-name`.

## Sprint 9 — Flutter app for Android and iOS
**Increment:** the same app on Android and iOS, built by CI on every push and
released on `v*` tags; the Kotlin app, its JNI crate and its workflow removed.
Every technical choice was made by the product owner (ADR 0009).
* `flutter_app/` (Flutter 3.50.1), same application/bundle id; typed
  flutter_rust_bridge API over the use cases (opaque `Session`, `AppFailure`).
* Domain rule added in Rust: `Schedule::upcoming` (next firings for exact
  notifications, capped below iOS's 64 pending).
* Dart: ports + adapters (Rust core, preferences, file picker, SoLoud player,
  local notifications), Riverpod state, atomic design (atoms → molecules →
  organisms → template → page) reproducing the Kotlin screen in Material 3.
* `flutter.yml`: bindings must match, Rust + Dart tests (incl. native),
  Android APK (release signing from secrets), unsigned iOS build.

**Retrospective**
* 👍 Test-first found real bugs before users: `usize` arriving as `BigInt`,
  a cancelled file choice leaving "Processing…", and the hand-written fake
  drifting from the Rust rules — fixed once with a shared contract test that
  runs against both.
* 👎 Two CI failures came from setup details: generated comments that
  depended on the rustc version (`derive(Eq)`), and the compileSdk 37
  required by flutter_local_notifications 23. **Action:** both rules are now
  in CLAUDE.md.
* 👎 The first port copied the older single-page Kotlin screen instead of
  the tabbed design on `main`. **Fix:** Reminders/Voices/Settings tabs, the
  daylight design system, self-saving editors and the build-time server URL;
  the UI product rules are now in `flutter_app/CLAUDE.md`.
* 👎 The port was English only and missed Android features. **Fix:** the
  21 languages, converted from the Kotlin `strings.xml` files; a test that
  fails when an Android text goes unused surfaced the missing features:
  microphone recording, several files per pick, MP3/M4A/OGG (decoded in
  the core, so the CLI gains them too), the saved confirmations and the
  locked switch of a reminder whose voice was deleted.

