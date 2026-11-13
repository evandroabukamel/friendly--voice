"""WAV (de)serialization and simple effects."""

from __future__ import annotations

import io

import librosa
import numpy as np
import parselmouth
import soundfile as sf
from parselmouth.praat import call

#: Shifts smaller than this are inaudible as a tone change but any
#: resynthesis still costs quality, so the audio is left untouched.
MIN_PITCH_SHIFT = 1.0
#: Pitch search range (Hz) for the PSOLA analysis; covers adult voices.
PITCH_FLOOR, PITCH_CEILING = 75.0, 600.0


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
    """Shifts the pitch with PSOLA (Praat overlap-add), which keeps formants
    and avoids the metallic sound of a phase vocoder."""
    if abs(semitones) < MIN_PITCH_SHIFT:
        return audio
    sound = parselmouth.Sound(audio.astype(np.float64), sampling_frequency=sample_rate)
    manipulation = call(sound, "To Manipulation", 0.01, PITCH_FLOOR, PITCH_CEILING)
    tier = call(manipulation, "Extract pitch tier")
    call(tier, "Multiply frequencies", sound.xmin, sound.xmax, 2.0 ** (semitones / 12.0))
    call([tier, manipulation], "Replace pitch tier")
    shifted = call(manipulation, "Get resynthesis (overlap-add)")
    return shifted.values[0].astype(np.float32)


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
