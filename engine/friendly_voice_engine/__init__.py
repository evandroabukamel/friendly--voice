"""Friendly Voice engine: speaks any text in the voice of a person, cloned
from their recordings (zero-shot, no training).

* **Enrollment.** The recordings become the conditionals of Chatterbox
  Multilingual: a speaker embedding averaged over all recordings, plus the
  10 s window of speech closest to it, which the model imitates for timbre,
  intonation, rhythm and accent.
* **Synthesis.** The text is read in the voice's language, sentence by
  sentence; the tone of the message maps to the model's expressiveness, and
  an optional pitch shift and gain are applied afterwards.
"""

__version__ = "0.2.0"
