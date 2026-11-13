# ADR 0007 — Zero-shot cloning with Chatterbox Multilingual (supersedes 0002 and 0006)

**Status:** accepted · Sprint 7

## Context
The pipeline of ADR 0002 + 0006 (accent rules in Rust → phonemes → Piper
pt-BR male base voice → OpenVoice timbre conversion) was evaluated by
listening to a real person's 28 recordings. The timbre came close, but the
speech never sounded like her: robotic, muffled ("nose closed"), wrong
stressed syllables, and — whatever phoneme notation was tried — "a foreigner
speaking Portuguese". The limits were structural:

* Piper has one global speaking rate and was trained on one male voice; the
  converter changes timbre, not intonation, rhythm or accent.
* The phonemes the accent model produced were outside what Piper was trained
  on, so every accent decision degraded intelligibility.
* Only Portuguese could be spoken.

## Decision
Replace the whole synthesis path with **Chatterbox Multilingual** (Resemble
AI, MIT license for code and weights), a zero-shot TTS that clones a voice
from a reference recording and reads text in 23 languages:

* **Enrollment** uses every recording: the speaker embedding is their
  average, and the reference the model imitates (intonation, rhythm, accent)
  is the 10 s window whose own embedding is closest to it.
* **Synthesis** sends the text and the person's language; the engine speaks
  sentence by sentence and retries takes that were cut off.
* The **accent model, the G2P and the phone recognizer are removed**: the
  model imitates the accent from the reference audio.
* The app **detects female/male** from the recordings' mean pitch and keeps
  it in the voice profile; a **language** is chosen per person.

On the prototype (3 test sentences, her recordings) Whisper understood 100 %
of every take, and the speaker similarity to her recordings was ≈ 0.90 —
her own recordings score 0.90–0.94 against each other.

## Consequences
* + Natural intonation and rhythm, the person's accent, any of 23 languages.
* + Permissive license; generated audio carries the Perth watermark (US-22).
* − Heavier: ~3 GB of weights; ~4 s per sentence on a laptop GPU, several
  times slower on CPU.
* − The accent is no longer explainable or adjustable (no rules, no
  intensity, no pronunciation preview): US-03, US-04, US-06, US-09 and US-10
  are withdrawn.
* − The message themes are written in Portuguese; other languages speak free
  text only.
* − Speaking rate is not directly controllable; tones map to the model's
  expressiveness (`exaggeration`) and pacing (`cfg_weight`).
