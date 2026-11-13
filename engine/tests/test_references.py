"""Unit tests for the reference selection, the text chunks and the languages
(no model weights needed)."""

import numpy as np
import pytest

from friendly_voice_engine import references as refs
from friendly_voice_engine import text as tx
from friendly_voice_engine.languages import model_language

SR = 16_000


def speech(seconds: float, f0: float = 220.0, pause_every: float | None = None) -> np.ndarray:
    t = np.arange(int(seconds * SR)) / SR
    wave = sum(np.sin(2 * np.pi * f0 * k * t) / k for k in range(1, 6))
    if pause_every:
        wave = wave * ((t % pause_every) < pause_every / 2)
    return (0.2 * wave).astype(np.float32)


def test_windows_prefer_continuous_speech():
    # 20 s of choppy speech followed by 12 s of continuous speech
    audio = np.concatenate([speech(20, pause_every=1.0), speech(12)])
    best = refs.candidate_windows(audio, SR, seconds=10, keep=1)[0]
    assert refs.speech_ratio(best, SR) > 0.95


def test_windows_do_not_overlap_and_short_recordings_are_kept_whole():
    windows = refs.candidate_windows(speech(40), SR, seconds=10, keep=2)
    assert len(windows) == 2 and all(len(w) == 10 * SR for w in windows)
    short = speech(4)
    assert refs.candidate_windows(short, SR, seconds=10)[0] is short


def test_clipped_windows_lose():
    clean, clipped = speech(10), np.clip(speech(10) * 20, -1, 1)
    audio = np.concatenate([clipped, clean])
    best = refs.candidate_windows(audio, SR, seconds=10, keep=1)[0]
    assert refs.clipping_ratio(best) == 0.0


@pytest.mark.parametrize("f0", [110.0, 240.0])
def test_median_pitch(f0):
    assert abs(refs.median_pitch(speech(3, f0), SR) - f0) < f0 * 0.05


def test_closest_embedding():
    embeddings = np.array([[1.0, 0.0], [0.6, 0.8], [0.0, 1.0]])
    assert refs.closest(embeddings, np.array([0.5, 0.7])) == 1


def test_chunks_follow_sentences_and_respect_the_limit():
    text = "Bom dia, meu amor! Levanta com fé.   Hoje é dia de coisa boa."
    assert tx.chunks(text, limit=40) == ["Bom dia, meu amor! Levanta com fé.", "Hoje é dia de coisa boa."]
    long = "palavra, " * 60
    assert all(len(c) <= 50 for c in tx.chunks(long, limit=50))
    assert tx.chunks("   ") == []


def test_cut_off_or_rambling_takes_are_implausible():
    text = "Bom dia, meu amor! Levanta com fé, Evandro, que hoje é dia de coisa boa."
    assert tx.plausible(text, 5.5)
    assert not tx.plausible(text, 1.7)  # cut off
    assert not tx.plausible(text, 30.0)  # rambling


def test_languages():
    assert model_language("pt-BR") == "pt"
    assert model_language("en_US") == "en"
    with pytest.raises(ValueError):
        model_language("tlh")
