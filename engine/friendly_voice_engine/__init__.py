"""Friendly Voice engine: clones a person's timbre from their recordings.

Pipeline for every message:

1. **Base speech (Piper, pt-BR).** The Rust core sends the exact phonemes to
   say, already carrying the learned accent (letter pronunciation, word
   endings and word junctions). Piper turns those phonemes into natural
   Brazilian Portuguese speech, so the accent is preserved verbatim.
2. **Timbre (OpenVoice V2 tone color converter).** The base speech is
   converted to the voice of the person, using a speaker embedding extracted
   from the uploaded recordings.
3. **Tone.** The tone chosen for the message (calm, cheerful, ...) is applied
   as speaking rate (Piper length scale), pitch shift and gain.
"""

__version__ = "0.1.0"
