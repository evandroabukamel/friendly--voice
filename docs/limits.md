# Limits, configuration and data folder

## Limits

| Limit | Value | Where |
|---|---|---|
| Sample duration | 2 s – **180 s** (longer: split into pieces) | `domain/src/audio.rs` `SampleQualityPolicy` |
| Sample rate | ≥ 16 kHz | same |
| Clipping | ≤ 1 % of samples at full scale | same |
| Loudness | RMS ≥ 0.005 (not silent) | same |
| Voice material | ≥ 1 sample and ≥ 10 s in total | `domain/src/voice.rs` `EnrollmentPolicy` |
| Input formats | WAV (PCM/float), FLAC (any bit depth, mixed to mono) | `infrastructure/src/audio/` |
| Message text | ≤ 600 chars; engine chunks ≤ 220 chars | `domain/src/message.rs`, `engine/.../text.py` |
| Pitch shift | ±6 semitones | `domain/src/tone.rs` |
| Tones | natural, warm, cheerful, calm, firm, whisper | `domain/src/tone.rs` |
| Languages | 23 (`fv languages`); themes only in Portuguese | `domain/src/language.rs` |
| Gender boundary | mean pitch 160 Hz | `domain/src/voice.rs` `VoiceGender` |
| Schedule interval | ≥ 1 min; rules that never fire are refused | `domain/src/schedule.rs` |
| Schedule look-ahead | 8 days | same |
| Engine payload | ≤ 64 MiB per audio | `infrastructure/src/voice/http.rs` |

## CLI configuration

| Env var | Flag | Meaning |
|---|---|---|
| `FRIENDLY_VOICE_DATA` | `--data-dir` | data folder, default `.friendly-voice` **relative to the cwd** (git-ignored) |
| `FRIENDLY_VOICE_ENGINE_URL` | `--engine-url` | engine URL (http-engine builds only) |
| `FRIENDLY_VOICE_ME` | `--my-name` | listener's name used in messages |
| `FRIENDLY_VOICE_PLAYER` | `--player` | command that plays generated audio (file path appended); `windows` = built-in PowerShell `SoundPlayer` |

## Data folder

`speakers.json`, `samples.json`, `audio/<speaker-id>/<sample-id>.{wav,flac}`,
`voices.json`, `schedules.json`, `outbox/` (WAV + note with text, language
and tone), speech cache. A sample can carry `<file>.txt` (transcript) next
to it.
