"""Splitting the message into chunks the speech model says reliably."""

from __future__ import annotations

import re

#: Longer inputs make the model skip or repeat words.
MAX_CHUNK_CHARS = 220
#: Average speaking rate used to tell a cut-off take from a complete one.
CHARS_PER_SECOND = 14.0

_SENTENCE_END = re.compile(r"(?<=[.!?…;:])\s+")
_CLAUSE_END = re.compile(r"(?<=[,—–])\s+")


def chunks(text: str, limit: int = MAX_CHUNK_CHARS) -> list[str]:
    """Sentences, merged while they fit in ``limit``; a sentence that is too
    long is cut at commas, and then between words."""
    text = " ".join(text.split())
    if not text:
        return []
    pieces: list[str] = []
    for sentence in _SENTENCE_END.split(text):
        if len(sentence) <= limit:
            pieces.append(sentence)
            continue
        for clause in _CLAUSE_END.split(sentence):
            pieces.extend(_by_words(clause, limit) if len(clause) > limit else [clause])
    merged: list[str] = []
    for piece in pieces:
        if merged and len(merged[-1]) + 1 + len(piece) <= limit:
            merged[-1] += " " + piece
        else:
            merged.append(piece)
    return merged


def _by_words(text: str, limit: int) -> list[str]:
    out: list[str] = []
    for word in text.split():
        if out and len(out[-1]) + 1 + len(word) <= limit:
            out[-1] += " " + word
        else:
            out.append(word)
    return out


def expected_seconds(text: str) -> float:
    return max(0.5, len(text) / CHARS_PER_SECOND)


def plausible(text: str, seconds: float) -> bool:
    """A take far shorter (cut off) or far longer (rambling) than the text
    needs is retried."""
    expected = expected_seconds(text)
    return 0.45 * expected <= seconds <= 2.5 * expected + 1.0
