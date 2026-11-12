# ADR 0005 — Android reusing the Rust core through JNI

**Status:** accepted · Sprint 4

## Context
We want the app on Android without duplicating business rules (accent, tone,
schedules) in Kotlin, and with an automated CI/CD build.

## Decision
* `crates/bootstrap` holds the composition root, used by the CLI and the app.
* `crates/mobile` (cdylib) exposes the use cases to Kotlin through JNI. The
  API is deliberately "stringly typed": simple inputs and JSON output
  (`{"ok":..,"data"|"error"}`), which keeps the FFI boundary minimal and lets
  the bridge be tested on the host, without an emulator. Panics are caught at
  the boundary.
* The Kotlin app (`android/`) only contains UI, preferences, notifications and
  the periodic scheduler (WorkManager).
* CI: `cargo-ndk` builds the `.so` files for `arm64-v8a`, `armeabi-v7a` and
  `x86_64` into `jniLibs/`; Gradle builds the APKs; `v*` tags publish a release.

## Alternatives considered
* **UniFFI** — generates typed bindings, but adds a code generation step and
  build dependencies; to be reconsidered if the API grows.
* **Rewriting in Kotlin** — would duplicate the accent model and the schedules.

## Consequences
* + A single implementation of the rules, tested once.
* + The same logic on the phone and in the CLI.
* − Schedule precision on Android is limited to 15 min (WorkManager).
* − Without a configured voice engine, the phone plays the prosody sketch.
