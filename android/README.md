# Friendly Voice — Android app

Kotlin presentation layer over the Rust core (`crates/mobile`, reached through
JNI in `NativeBridge.kt`). Every business rule (consent, schedules, tones,
validation of samples) lives in Rust; the app shows screens and forwards
choices.

Build: `scripts/build-android.sh [--release]` (needs `cargo-ndk` and
`ANDROID_NDK_HOME`). JVM unit tests: `gradle -p android testDebugUnitTest`
(the native library must have been built first, see `checkNativeLibs`).

## Screens

Bottom navigation with three tabs (`res/menu/bottom_nav.xml`,
`res/navigation/nav_graph.xml`):

| Screen | Fragment | What it does |
|---|---|---|
| Alarms (home) | `ui/alarms/AlarmsFragment` | alarm cards with an ON/OFF switch; tap → edit; `+` → create |
| Create alarm | `ui/alarms/EditAlarmFragment` | voice, theme, tone, pitch, repeat, quiet hours, Play now, Save |
| Voices | `ui/voices/VoicesFragment` | voice cards; tap → Import voice; `+` → new voice |
| Import voice | `ui/voices/ImportVoiceFragment` | name, consent, language, nickname; add files / record; Create voice |
| Settings | `ui/settings/SettingsFragment` | your name and the voice-server URL |

## Code layout

```
app/src/main/java/app/friendlyvoice/
  NativeBridge.kt        every JNI declaration (one file: renames are one-line fixes)
  FriendlyVoiceApp.kt    holds the single NativeRepository
  ScheduleWorker.kt      WorkManager, every 15 min: fires due schedules
  core/                  pure Kotlin (no Android types), unit-tested on the JVM
  data/                  NativeRepository (all JNI calls), Prefs
  audio/                 AudioImporter (MP3 → WAV), VoiceRecorder, MessagePlayer
  ui/                    MainActivity, fragments, ViewModels, adapters
app/src/test/            JUnit tests for core/
```

Rules the code follows:

- **No main-thread work**: `NativeRepository` runs every JNI call on
  `Dispatchers.IO`; file reading, MP3 decoding and recording run on IO too;
  `MessagePlayer` uses `prepareAsync`.
- **One writer**: a `Mutex` in `NativeRepository` serializes every call, and
  `ScheduleWorker` goes through the same instance.
- **No flooding the server**: Create voice, Play now and Save are guarded by
  `SingleFlight` (button disabled, extra taps ignored); alarm switches go
  through `ToggleCoalescer` (400 ms debounce, last state wins, only sent if it
  differs from the stored one); the catalog is cached; lists refresh on resume
  only.

## Resources: Atomic Design

All visual components are XML under `res/`; Kotlin only binds data
(ViewBinding) and listeners — no views are built in code.

| Level | Where | Examples |
|---|---|---|
| Tokens | `values/colors.xml`, `values/dimens.xml`, `values/strings.xml` | sunrise palette, 28dp card radius, 18sp body, plurals |
| Atoms | styles in `values/styles.xml` / `values/themes.xml`, `layout/atom_*.xml` | `Widget.FriendlyVoice.Button`, `.Text.Time`, `.Card`, `.Chip.Day`, `.Switch`; `atom_dropdown_item` |
| Molecules | `layout/molecule_*.xml` | `molecule_alarm_card`, `molecule_voice_card`, `molecule_sample_row`, `molecule_time_range`, `molecule_screen_header` |
| Organisms | `layout/organism_*.xml`, included with `<include android:id=…>` | `organism_repeat_picker`, `organism_quiet_hours`, `organism_tone_pitch`, `organism_sample_actions` |
| Screens | `layout/fragment_*.xml`, `layout/activity_*.xml` | `fragment_alarms`, `fragment_edit_alarm`, `activity_main` |

New UI: add tokens first, then reuse or add an atom style, then compose
molecules and organisms; a screen layout should mostly be includes and cards.

## Languages

The UI follows the system language and can be changed in Settings → App
language (AppCompat per-app locales; `res/xml/locales_config.xml` +
`AppLocalesMetadataHolderService` with `autoStoreLocales` so the choice
persists below Android 13). Supported: pt-BR, pt-PT, en-GB, en-US, en-NZ,
en-AU, es-ES, es-MX, es-AR, de, fr, it, zh-CN, zh-TW, ja, ru, uk, tr, ar,
fi, nb (also `values-no` for devices that still report the legacy `no` code).

- `values/` is English (US) and holds every key; format-only strings are
  `translatable="false"`.
- `values-pt` (Brazilian) and `values-es` (Spain) are complete; regional
  folders (`-rPT`, `-rMX`, `-rAR`, `-rGB`, `-rAU`, `-rNZ`) hold only real
  differences, and `-rBR`, `-rES`, `-rUS` are intentionally empty.
- `LocaleResourcesTest` (JVM) fails if a locale misses or adds keys, changes
  placeholders, or lacks a plural quantity its language needs (Arabic
  zero/one/two/few/many/other, Russian/Ukrainian one/few/many/other).
- Arabic is right-to-left: layouts use start/end only and directional icons
  are `autoMirrored`.
- A new voice's language defaults to the catalog code closest to the app
  locale (`core/LanguageMatch`). Spoken message themes stay Portuguese
  (backlog US-28 in the Rust core).
- To add a language: add its `values-xx/strings.xml`, list it in
  `res/xml/locales_config.xml`, `values/arrays.xml` and `LocaleResourcesTest`.
