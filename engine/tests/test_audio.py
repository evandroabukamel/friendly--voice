"""Unit tests for the audio effects (no model weights needed)."""

import numpy as np
import parselmouth

from friendly_voice_engine import audio as fx

SR = 16_000


def voiced(f0: float, seconds: float = 1.0) -> np.ndarray:
    t = np.arange(int(seconds * SR)) / SR
    return (0.2 * sum(np.sin(2 * np.pi * f0 * k * t) / k for k in range(1, 6))).astype(np.float32)


def median_f0(audio: np.ndarray) -> float:
    pitch = parselmouth.Sound(audio.astype(np.float64), SR).to_pitch(pitch_floor=75, pitch_ceiling=600)
    f0 = pitch.selected_array["frequency"]
    return float(np.median(f0[f0 > 0]))


def test_small_shift_leaves_audio_untouched():
    audio = voiced(220.0)
    assert fx.pitch_shift(audio, SR, 0.5) is audio


def test_psola_shift_moves_f0_by_the_requested_semitones():
    audio = voiced(220.0)
    shifted = fx.pitch_shift(audio, SR, 2.0)
    assert abs(len(shifted) - len(audio)) <= SR // 100  # duration is kept
    assert abs(12 * np.log2(median_f0(shifted) / 220.0) - 2.0) < 0.2
