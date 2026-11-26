# ADR 0009 — Flutter app (Android + iOS) over the Rust core (supersedes 0005)

**Status:** accepted · Sprint 9

## Context
The Kotlin app (ADR 0005) only ran on Android. The app must also run on
iOS, keep the Kotlin app's features and visual pattern, follow atomic
design, SOLID and Clean Architecture, and use only well-ranked packages
updated in the last 4 months (official flutter.dev packages excepted).

## Decision
All choices below were made by the product owner:

* **Flutter 3.50.1** (latest stable at the time) in the isolated folder
  `flutter_app/`, same application/bundle id as the Kotlin app
  (`app.friendlyvoice`).
* **Business rules stay in Rust**, reached through **flutter_rust_bridge
  2.13** (`flutter_app/rust/`, a workspace member). The bridge API is typed
  (no JSON) and is a presentation layer like the CLI: Dart owns an opaque
  `Session` instead of the JNI bridge's global state; `AppFailure` is the
  single error type. Plain structs and plain enums only, so Dart needs no
  code generation (no freezed). FRB's cargokit builds Rust during the
  Gradle/Xcode build (accepted as FRB tooling, although its build tool pins
  old Dart packages).
* **Dart layers:** `domain/` (ports: `VoiceCore`, `SettingsStore`,
  `AudioFilePicker`, `AudioPlayer`, `AlarmScheduler`; the data types are the
  ones generated from the Rust API, their single source of truth),
  `application/` (`DeliverDueMessages`), `infrastructure/` (adapters),
  `presentation/` (Riverpod state + atomic design: atoms → molecules →
  organisms → template → page). The Riverpod providers are the only
  composition root.
* **Packages:** flutter_riverpod 3.5, flutter_local_notifications 23 +
  timezone, flutter_soloud 5.3 (just_audio/audioplayers were stale),
  file_picker 13.4; flutter.dev: material_ui, shared_preferences,
  path_provider. Lints: very_good_analysis (flutter_lints was stale). Tests
  use hand-written fakes (mocktail was stale).
* **Schedules wake the app with exact OS notifications** on both platforms
  (iOS has no reliable periodic background work). The core lists the next
  firings (`Schedule::upcoming`, at most 60 < iOS's 64 pending); the app
  speaks what is due on start (single catch-up), on a tapped alarm and every
  minute while open. Android 14+: `SCHEDULE_EXACT_ALARM` asked at runtime,
  inexact-while-idle fallback when refused.
* **The Kotlin app's design** (from `main`'s `res/` layouts) in Material 3:
  a bottom navigation bar with Reminders, Voices and Settings; a warm,
  Timely-like look whose gradient follows the time of day, with large,
  light type; edits save themselves (no Save buttons); the voice server URL
  is compiled in (`--dart-define=FRIENDLY_VOICE_ENGINE_URL`), never typed.
  *Revised:* the first port reproduced the older single-page Kotlin screen.
* **CI/CD:** `flutter.yml` — regenerated bindings must match, Rust bridge
  tests, Dart tests (some call the real Rust core on the host), Android
  APK, unsigned iOS build on macOS, release on `v*` tags.
* The Kotlin app, its JNI crate and its workflow are **removed** once the
  Flutter app builds on both platforms.

## Consequences
* + One codebase for Android and iOS; still one implementation of the rules.
* + A shared contract test runs the same scenarios against the fake core and
  the real Rust core, so the fakes cannot drift from the rules.
* − A notification only reminds the user; the message is generated when the
  app opens or the alarm is tapped (the Kotlin worker generated it in the
  background every 15 min, Android only).
* − iOS builds need macOS (CI) and signing needs an Apple Developer account
  (not set up: the iOS artifact is unsigned).
* − Android compiles against API 37 (required by flutter_local_notifications
  23); target and min SDK stay at the Flutter defaults.
