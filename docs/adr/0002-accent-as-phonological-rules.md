# ADR 0002 — Accent as ordered phonological rules in the domain

**Status:** accepted · Sprint 1

## Context
The feature must take the accent into account: letter pronunciation, word
endings and word junctions. Neural cloning engines copy the timbre well, but
they tend to "normalize" pronunciation to the language standard, and they are
not explainable.

## Decision
Model the accent in the **domain** as `target → replacement / context` rules
applied in order (as in generative phonology), of three kinds: segmental, word
ending and juncture. Rules are learned by comparing the citation form (G2P)
with the heard phonemes, and are handed to the engine as phonemes per word
(and SSML `<phoneme>` for engines that only take SSML).

## Consequences
* + Explainable: the user sees each phenomenon with its measured frequency.
* + Controllable: accent intensity, rule editing.
* + Testable without audio.
* − Needs observed phonemes (manual annotation or a phone recognizer).
* − Fine prosodic phenomena (per-phoneme duration, intonation) are left for
  future stories (US-12, US-13).
