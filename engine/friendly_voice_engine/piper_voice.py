"""Piper (VITS, ONNX) base speaker for Brazilian Portuguese."""

from __future__ import annotations

import json
from pathlib import Path

import numpy as np
import onnxruntime as ort

from .phonemes import PhonemeIdMapper, count_syllables, utterance_to_piper, Word

#: Sentence used to measure the base voice's natural speaking rate.
CALIBRATION = [
    Word(["v", "o", "s", "e"], 3),
    Word(["k", "õ", "s", "e", "g", "i"], 3),
    Word(["f", "a", "z", "e", "ɾ"], 3),
    Word(["t", "u", "d", "u"], 1),
    Word(["k", "õ"], 1),
    Word(["k", "a", "l", "m", "a"], 1, pause_after=True),
    Word(["ũ"], 0),
    Word(["p", "a", "s", "u"], 1),
    Word(["d", "e"], None),
    Word(["k", "a", "d", "a"], 1),
    Word(["v", "e", "s"], 1),
]


class PiperVoice:
    def __init__(self, model_path: Path, config_path: Path):
        config = json.loads(config_path.read_text(encoding="utf-8"))
        self.sample_rate: int = config["audio"]["sample_rate"]
        inference = config.get("inference", {})
        self.noise_scale = float(inference.get("noise_scale", 0.667))
        self.noise_w = float(inference.get("noise_w", 0.8))
        self.mapper = PhonemeIdMapper(config["phoneme_id_map"])
        options = ort.SessionOptions()
        options.log_severity_level = 3
        self.session = ort.InferenceSession(str(model_path), sess_options=options)
        self.syllables_per_second = self._measure_rate()

    def synthesize_phonemes(self, phonemes: str, length_scale: float = 1.0) -> np.ndarray:
        ids = self.mapper.to_ids(phonemes)
        audio = self.session.run(
            None,
            {
                "input": np.array([ids], dtype=np.int64),
                "input_lengths": np.array([len(ids)], dtype=np.int64),
                "scales": np.array([self.noise_scale, length_scale, self.noise_w], dtype=np.float32),
            },
        )[0]
        return audio.squeeze().astype(np.float32)

    def synthesize(self, words: list[Word], length_scale: float = 1.0) -> np.ndarray:
        return self.synthesize_phonemes(utterance_to_piper(words), length_scale)

    def _measure_rate(self) -> float:
        phonemes = utterance_to_piper(CALIBRATION)
        audio = self.synthesize_phonemes(phonemes)
        speech_secs = voiced_seconds(audio, self.sample_rate)
        return count_syllables(phonemes) / max(speech_secs, 1e-3)


def voiced_seconds(audio: np.ndarray, sample_rate: int, frame_ms: float = 20.0) -> float:
    """Duration of the non-silent part of the audio (simple energy gate)."""
    frame = max(1, int(sample_rate * frame_ms / 1000))
    n = len(audio) // frame
    if n == 0:
        return 0.0
    rms = np.sqrt(np.mean(audio[: n * frame].reshape(n, frame) ** 2, axis=1))
    threshold = 0.1 * np.percentile(rms, 95)
    return float(np.count_nonzero(rms >= threshold)) * frame / sample_rate
