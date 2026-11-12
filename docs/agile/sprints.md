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
**Increment:** `amostra adicionar` + `voz gerar` working with the local engine.
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
* `falar`, `pronuncia` (with intensity), `agenda criar/listar/remover/executar`.
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
