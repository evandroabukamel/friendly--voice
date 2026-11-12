# ADR 0003 — Timbre through an external neural engine behind ports

**Status:** accepted · Sprint 2

## Context
Cloning a timbre needs heavy neural models (OpenVoice, XTTS, F5-TTS), written
in Python and often run on GPUs, each with its own license.

## Decision
Define the ports `VoiceCloningEngine`, `SpeechSynthesizer`, `Transcriber` and
`PhoneRecognizer` and a simple HTTP contract (`docs/engine-api.md`), with an
optional Rust adapter (feature `http-engine`). Locally, a **prosody sketch**
synthesizer validates rhythm/intonation and the end-to-end flow.

## Consequences
* + The Rust core stays light, with no ML dependencies.
* + Any model can be plugged in by implementing a few endpoints.
* − Without the server, local audio does not reproduce the real timbre.
* Follow-up: the reference server was added in Sprint 5 (ADR 0006).
