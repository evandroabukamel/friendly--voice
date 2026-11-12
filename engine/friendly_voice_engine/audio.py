"""WAV (de)serialization and simple effects."""

from __future__ import annotations

import io

import librosa
import numpy as np
import soundfile as sf


def decode_wav(data: bytes) -> tuple[np.ndarray, int]:
    """Decodes a WAV (or any format libsndfile reads) to mono float32."""
    audio, sample_rate = sf.read(io.BytesIO(data), dtype="float32", always_2d=True)
    return audio.mean(axis=1), int(sample_rate)


def encode_wav(audio: np.ndarray, sample_rate: int) -> bytes:
    buffer = io.BytesIO()
    sf.write(buffer, np.clip(audio, -1.0, 1.0), sample_rate, format="WAV", subtype="PCM_16")
    return buffer.getvalue()


def resample(audio: np.ndarray, sample_rate: int, target: int) -> np.ndarray:
    if sample_rate == target:
        return audio
    return librosa.resample(audio, orig_sr=sample_rate, target_sr=target)


def pitch_shift(audio: np.ndarray, sample_rate: int, semitones: float) -> np.ndarray:
    if abs(semitones) < 0.05:
        return audio
    return librosa.effects.pitch_shift(audio, sr=sample_rate, n_steps=semitones)


def apply_gain(audio: np.ndarray, gain_db: float, ceiling: float = 0.97) -> np.ndarray:
    out = audio * (10.0 ** (gain_db / 20.0))
    peak = float(np.max(np.abs(out))) if out.size else 0.0
    if peak > ceiling:  # never clip
        out = out * (ceiling / peak)
    return out.astype(np.float32)


def normalize_peak(audio: np.ndarray, peak: float = 0.9) -> np.ndarray:
    current = float(np.max(np.abs(audio))) if audio.size else 0.0
    return audio if current <= 1e-6 else (audio * (peak / current)).astype(np.float32)


def silence(seconds: float, sample_rate: int) -> np.ndarray:
    return np.zeros(int(seconds * sample_rate), dtype=np.float32)
