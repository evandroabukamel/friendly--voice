"""Turns the accented phonemes sent by the Rust core into Piper input.

The Rust core already decided *how* each word is pronounced (that is where
the learned accent lives). This module only adapts the symbols to the
inventory of the Piper voice (espeak-ng flavoured IPA), adds stress marks and
pauses, and maps everything to Piper phoneme ids. No text normalization or
grapheme-to-phoneme step happens here, on purpose.
"""

from __future__ import annotations

import unicodedata
from dataclasses import dataclass, field

STRESS = "ˈ"

#: Our symbol → Piper/espeak symbol, when they differ.
SUBSTITUTIONS = {
    "g": "ɡ",  # Piper uses U+0261 LATIN SMALL LETTER SCRIPT G
    "j̃": "j",
    "w̃": "w",
}

#: Consonants that can be the second element of an onset cluster (pr, bl...).
LIQUIDS = {"ɾ", "l", "r"}

VOWEL_BASES = set("aeiouɛɔɐəɨɪʊ")


def is_vowel(symbol: str) -> bool:
    base = unicodedata.normalize("NFD", symbol)[:1]
    return base in VOWEL_BASES


def is_glide(symbol: str) -> bool:
    return symbol in {"j", "w", "j̃", "w̃"}


@dataclass
class Word:
    phonemes: list[str]
    #: Index (in ``phonemes``) of the stressed vowel, if any.
    stress: int | None = None
    #: A pause (punctuation) follows the word.
    pause_after: bool = False

    @classmethod
    def from_json(cls, data: dict) -> "Word":
        return cls(
            phonemes=[str(p) for p in data.get("phonemes", [])],
            stress=data.get("stress"),
            pause_after=bool(data.get("pause_after", False)),
        )


def stressed_syllable_start(phonemes: list[str], stress: int) -> int:
    """Index where the stressed syllable starts (its onset), so that the
    stress mark goes before the onset, as in espeak output: ``kaˈzaɾ``."""
    start = stress
    if start > 0 and not is_vowel(phonemes[start - 1]) and not is_glide(phonemes[start - 1]):
        start -= 1
        if (
            start > 0
            and phonemes[start] in LIQUIDS
            and not is_vowel(phonemes[start - 1])
            and not is_glide(phonemes[start - 1])
        ):
            start -= 1
    return start


def word_to_piper(word: Word) -> str:
    symbols = [SUBSTITUTIONS.get(p, p) for p in word.phonemes]
    if word.stress is not None and 0 <= word.stress < len(symbols):
        at = stressed_syllable_start(word.phonemes, word.stress)
        symbols.insert(at, STRESS)
    return unicodedata.normalize("NFD", "".join(symbols))


def utterance_to_piper(words: list[Word]) -> str:
    """Joins words with spaces; pauses become commas and the utterance ends
    with a period (Piper voices were trained with punctuation tokens)."""
    parts: list[str] = []
    for i, word in enumerate(words):
        text = word_to_piper(word)
        if not text:
            continue
        last = i == len(words) - 1
        if word.pause_after and not last:
            text += ","
        parts.append(text)
    return (" ".join(parts) + ".") if parts else ""


@dataclass
class PhonemeIdMapper:
    """Maps a Piper phoneme string to ids using the voice's ``phoneme_id_map``."""

    id_map: dict[str, list[int]]
    skipped: set[str] = field(default_factory=set)

    def to_ids(self, phonemes: str) -> list[int]:
        pad = self.id_map["_"]
        ids = list(self.id_map["^"]) + list(pad)
        for ch in phonemes:
            mapped = self.id_map.get(ch)
            if mapped is None:
                self.skipped.add(ch)
                continue
            ids.extend(mapped)
            ids.extend(pad)
        ids.extend(self.id_map["$"])
        return ids


def count_syllables(phonemes: str) -> int:
    """Syllables = vowel letters that are not combining marks."""
    return sum(1 for ch in phonemes if ch in VOWEL_BASES)
