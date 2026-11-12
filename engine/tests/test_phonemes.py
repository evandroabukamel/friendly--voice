"""Unit tests for the phoneme adapter (no model weights needed)."""

import unicodedata

from friendly_voice_engine.phonemes import (
    PhonemeIdMapper,
    Word,
    count_syllables,
    utterance_to_piper,
    word_to_piper,
)


def nfd(s: str) -> str:
    return unicodedata.normalize("NFD", s)


def test_stress_mark_goes_before_the_onset():
    assert word_to_piper(Word(["n", "o", "j", "tʃ", "i"], 1)) == "ˈnojtʃi"
    assert word_to_piper(Word(["k", "a", "z", "a", "ɾ"], 3)) == "kaˈzaɾ"
    # onset clusters (consonant + liquid) stay together
    assert word_to_piper(Word(["k", "o", "b", "ɾ", "a"], 4)) == "koˈbɾa"
    # vowel-initial stressed syllable
    assert word_to_piper(Word(["a", "ɡ", "w", "a"], 0)) == "ˈaɡwa"


def test_symbols_are_adapted_to_piper():
    assert word_to_piper(Word(["g", "a", "t", "u"])) == "ɡatu"
    # precomposed nasal vowels are decomposed (Piper maps the combining tilde)
    assert word_to_piper(Word(["k", "ẽ", "tʃ", "i"])) == nfd("kẽtʃi")


def test_utterance_punctuation_and_accent_are_preserved():
    words = [
        Word(["b", "õ"], 1),
        Word(["dʒ", "i", "a"], 1, pause_after=True),
        Word(["v", "o", "s", "e"], 3),
    ]
    assert utterance_to_piper(words) == nfd("ˈbõ ˈdʒia, voˈse.")
    assert count_syllables(utterance_to_piper(words)) == 5


def test_id_mapping_interleaves_padding_and_reports_unknown_symbols():
    id_map = {"^": [1], "$": [2], "_": [0], "a": [5], "ˈ": [9]}
    mapper = PhonemeIdMapper(id_map)
    assert mapper.to_ids("ˈa?") == [1, 0, 9, 0, 5, 0, 2]
    assert mapper.skipped == {"?"}
