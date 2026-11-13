"""Choosing, from the person's recordings, the audio that conditions the voice.

Pure signal processing (no model weights), so it is unit tested directly.
"""

from __future__ import annotations

import numpy as np

from . import audio as fx

#: Sample rate used for the analysis.
ANALYSIS_RATE = 16_000
#: Length of the reference window the cloning model listens to.
WINDOW_SECONDS = 10.0
#: Frames quieter than this fraction of the clip's loud frames count as pauses.
SILENCE_RATIO = 0.1
#: Candidate windows kept per recording (the best ones, not overlapping).
WINDOWS_PER_RECORDING = 2


def frame_rms(audio: np.ndarray, frame: int) -> np.ndarray:
    usable = len(audio) // frame * frame
    if usable == 0:
        return np.zeros(0, dtype=np.float32)
    return np.sqrt(np.mean(audio[:usable].reshape(-1, frame) ** 2, axis=1))


def speech_ratio(audio: np.ndarray, sample_rate: int) -> float:
    """Share of 20 ms frames that carry speech (not pauses)."""
    rms = frame_rms(audio, max(1, sample_rate // 50))
    if rms.size == 0:
        return 0.0
    loud = float(np.percentile(rms, 95))
    return float(np.mean(rms > loud * SILENCE_RATIO)) if loud > 1e-4 else 0.0


def clipping_ratio(audio: np.ndarray) -> float:
    return float(np.mean(np.abs(audio) >= 0.99)) if audio.size else 0.0


def candidate_windows(
    audio: np.ndarray, sample_rate: int, seconds: float = WINDOW_SECONDS, keep: int = WINDOWS_PER_RECORDING
) -> list[np.ndarray]:
    """The ``keep`` windows of a recording with the most continuous speech and
    no clipping. A recording shorter than the window is a candidate as is."""
    size = int(seconds * sample_rate)
    if len(audio) <= size:
        return [audio]
    hop = sample_rate  # one second
    scored = []
    for start in range(0, len(audio) - size + 1, hop):
        window = audio[start:start + size]
        scored.append((speech_ratio(window, sample_rate) - 10 * clipping_ratio(window), start))
    scored.sort(reverse=True)
    picked: list[int] = []
    for _, start in scored:
        if all(abs(start - other) >= size for other in picked):
            picked.append(start)
        if len(picked) == keep:
            break
    return [audio[s:s + size] for s in picked]


def median_pitch(audio: np.ndarray, sample_rate: int, max_seconds: float = 30.0) -> float:
    """Median fundamental frequency (Hz) of the voiced frames, 0 if none."""
    import librosa

    clip = fx.resample(audio, sample_rate, ANALYSIS_RATE)[: int(max_seconds * ANALYSIS_RATE)]
    f0, voiced, _ = librosa.pyin(clip, fmin=60, fmax=500, sr=ANALYSIS_RATE, frame_length=1024)
    f0 = f0[voiced & ~np.isnan(f0)]
    return float(np.median(f0)) if f0.size else 0.0


def closest(embeddings: np.ndarray, target: np.ndarray) -> int:
    """Index of the embedding with the highest cosine similarity to ``target``."""
    norms = np.linalg.norm(embeddings, axis=1) * np.linalg.norm(target)
    return int(np.argmax(embeddings @ target / np.maximum(norms, 1e-9)))
