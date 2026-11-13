# ADR 0006 — OpenVoice V2 (not XTTS v2) for voice cloning

**Status:** superseded by [ADR 0007](0007-zero-shot-cloning-with-chatterbox.md) · Sprint 5

## Context
The app needs a free-to-use model to clone the timbre of the uploaded
recordings. The two candidates were:

| | OpenVoice V2 (MyShell) | XTTS v2 (Coqui) |
|---|---|---|
| License | **MIT** for code and weights (since April 2024), commercial use allowed | Coqui Public Model License: **non-commercial only**, including outputs; Coqui shut down, so no commercial license can be bought |
| Approach | Tone color converter: changes the timbre of any speech, language-agnostic | End-to-end multilingual TTS (Portuguese included) |
| Control of pronunciation | Total: the base speech can be generated from our own phonemes | Text input only: it re-reads the text with its own pronunciation |
| Cost | Small converter (~130 MB), runs on CPU | Larger model, GPU recommended |

## Decision
Use **OpenVoice V2's tone color converter** for timbre, fed by base speech
from **Piper `pt_BR-faber-medium`**, which accepts phoneme input. The learned
accent (ADR 0002) is sent as phonemes, so Piper speaks exactly the accented
pronunciation and OpenVoice turns it into the person's voice. The server is
`engine/` (Python, FastAPI); OpenVoice is installed from a pinned commit with
`--no-deps`, and only its converter modules are imported.

## Consequences
* + Permissive licenses for the cloning model; usable in a product.
* + The accent survives synthesis, because pronunciation is never delegated
  to the neural model.
* + Fast on CPU (≈ real time or better).
* − Base speech quality and intonation come from Piper (a single pt-BR voice);
  the converter changes timbre, not speaking style.
* − The Piper voice was fine-tuned from a voice whose dataset (Blizzard 2013
  *lessac*) has its own license: review it, or replace the base voice, before
  commercial distribution.
* − No audio watermark is added yet (OpenVoice's optional `wavmark`); a
  follow-up story could add it to mark cloned audio.
