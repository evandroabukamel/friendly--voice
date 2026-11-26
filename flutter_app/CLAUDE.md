# flutter_app/ — Flutter app (Android + iOS)

Presentation only: every business rule lives in the Rust core (ADR 0009).

## Product rules for the UI

- Three tabs in a bottom navigation bar: **Reminders**, **Voices**,
  **Settings**.
- Look: a clean, warm design in the spirit of the old Timely alarm app.
  The background gradient follows the time of day (dawn, day, dusk,
  night, all warm), and times and titles use large, light-weight type.
- Every edit **saves itself**: screens have no Save button.
- The voice server URL comes from the build
  (`--dart-define=FRIENDLY_VOICE_ENGINE_URL=…`). Users never type it.

## Languages

- The 21 languages of the Kotlin app (Settings → App language; default:
  the phone's). Texts live in `lib/l10n/app_*.arb`, keyed by the Android
  string names in camelCase ("alarm" → "reminder"); `app_en.arb` is the
  template. `flutter gen-l10n` (also run by `flutter pub get`) writes the
  Dart classes there.
- `test/l10n/strings_test.dart` fails when a language lacks a text or a
  text is unused: an unused Android text means a missing feature.
- Delegates: `AppLocalizations.delegate` + material_ui's
  `GlobalMaterialLocalizations.delegates` (flutter_localizations' Material
  delegates do not serve material_ui widgets).

## Layers

- `lib/domain` (ports; data types are the FRB-generated ones),
  `lib/application` (`DeliverDueMessages`), `lib/infrastructure` (adapters),
  `lib/presentation` (Riverpod state; atoms → molecules → organisms →
  templates → pages). Providers in `presentation/state/providers.dart` are
  the only composition root.

## Bridge (`rust/src/api/`)

- Plain structs and plain enums only (rich enums would need freezed);
  counts as `u32` (usize becomes BigInt in Dart); derive `PartialEq` only
  (`Eq` leaks into the generated comments and differs between rustc
  versions).
- After changing it: `flutter_rust_bridge_codegen generate` (FRB 2.13.0 +
  cargo-expand) and commit `lib/src/rust/`. CI fails on any diff.

## Tests

- Hand-written fakes in `test/support`; every platform adapter is faked via
  `fakeAdapters()`. `test/contract` runs the same scenarios against the fake
  and the real Rust core (native, tag `native`; needs
  `cargo build -p friendly_voice_bridge`).

## Packages and platforms

- Package rule: well-ranked packages released in the last 4 months;
  flutter.dev packages excepted. **Ask before adding any package.**
- Android compileSdk 37 (flutter_local_notifications 23); exact alarms via
  `SCHEDULE_EXACT_ALARM`, inexact fallback. iOS: notifications delegate,
  `NSAllowsLocalNetworking` for a LAN voice server.
- Build and run from `flutter_app/` (`flutter run` / `flutter build apk`):
  Cargokit builds the Rust library only through the Flutter build.

## Commands

```bash
flutter test
flutter_rust_bridge_codegen generate
flutter build apk --release          # or: flutter build ios --no-codesign (macOS)
```
