# ADR 0008 — Synthesized-speech cache

**Status:** accepted · Sprint 8

## Context
Synthesis with the neural engine takes about 4 s per sentence on a laptop
GPU and is impossible when the phone cannot reach the server. Schedules and
"speak now" often repeat the same talk (same person, text and tone), and the
user asked that generated voices be cached locally so the server is not
asked again for the same talk.

## Decision
A decorator, `CachedSpeechSynthesizer` (infrastructure), wraps whatever
`SpeechSynthesizer` the composition root picks (sketch or HTTP engine). No
port or use case changes.

* **Where:** `<data>/cache/speech/<speaker-id>/<key>.wav`, inside the data
  folder, so it stays local like the rest of the person's data (ADR 0004).
* **Key:** a stable hash (FNV-1a 128, not `DefaultHasher`, which may change
  between Rust versions) of the engine kind (`sketch`/`neural`), the whole
  `VoiceProfile` and the whole `SpeechScript`. That covers the voice version
  (`VoiceProfile.built_at`, set when the voice is generated, so rebuilding
  the voice invalidates it), the final personalized text, the language, the
  tone style and pitch shift (applied by the engine) and the tone-coloured
  prosody (used by the sketch engine).
* **Writes are atomic:** temp file in the same folder, `fsync`, rename. A
  crash never leaves a half-written `.wav` that would be served later; empty
  files count as a miss.
* Engine errors are never cached; a failed cache write only costs a later
  request.
* **Cleanup:** `SpeechCache::clear` deletes a person's folder when their voice
  is regenerated, when consent is revoked and when the person is deleted.
  Changing samples, the listener name or the language does not clear it
  (the key already tells those talks apart).

## Consequences
* + Repeated talks play instantly and offline.
* + Switching engines or rebuilding the voice never serves stale audio.
* + Revoking consent or deleting a person leaves no cached speech behind.
* − Within one voice the cache grows without bound; no eviction yet.
  Deleting the folder is always safe.
