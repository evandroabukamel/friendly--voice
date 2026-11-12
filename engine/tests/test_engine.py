"""Integration tests: real models (skipped when the weights are not present).

Run ``friendly-voice-engine download`` first, or point FV_MODELS_DIR at them.
"""

import base64
import os
from pathlib import Path

import numpy as np
import pytest

from friendly_voice_engine import audio as fx
from friendly_voice_engine.config import Settings
from friendly_voice_engine.download import missing

settings = Settings.from_env()
pytestmark = pytest.mark.skipif(
    bool(missing(settings.models_dir)), reason="model weights not downloaded"
)


def voice_like(seconds: float, f0: float, sr: int = 16_000) -> np.ndarray:
    """Harmonic signal with syllable-like pauses (a stand-in for a recording)."""
    t = np.arange(int(seconds * sr)) / sr
    voiced = (t % 0.5) < 0.35
    wave = sum(np.sin(2 * np.pi * f0 * k * t) / k for k in range(1, 6))
    return (0.15 * wave * voiced).astype(np.float32)


@pytest.fixture(scope="module")
def client(tmp_path_factory):
    from fastapi.testclient import TestClient

    from friendly_voice_engine.server import create_app

    voices = tmp_path_factory.mktemp("voices")
    os.environ["FV_VOICES_DIR"] = str(voices)
    return TestClient(create_app(settings=Settings.from_env()))


def b64wav(audio: np.ndarray, sr: int = 16_000) -> str:
    return base64.b64encode(fx.encode_wav(audio, sr)).decode()


def test_health(client):
    body = client.get("/health").json()
    assert body["status"] == "ok"
    assert 2.0 < body["base_voice_syllables_per_second"] < 12.0


def test_clone_and_speak_with_accent_and_tone(client):
    enrolled = client.post(
        "/v1/voices",
        json={"name": "Vó", "references": [{"audio_wav_base64": b64wav(voice_like(6, 210)), "transcript": "x"}]},
    )
    assert enrolled.status_code == 200, enrolled.text
    voice_id = enrolled.json()["voice_id"]

    words = [
        {"phonemes": ["b", "õ"], "stress": 1},
        {"phonemes": ["dʒ", "i", "a"], "stress": 1, "pause_after": True},
        {"phonemes": ["v", "o", "s", "e"], "stress": 3},
        {"phonemes": ["ɛ"], "stress": 0},
        {"phonemes": ["f", "ɔ", "h", "tʃ", "i"], "stress": 1},
    ]

    def say(**extra):
        r = client.post("/v1/synthesize", json={"voice_id": voice_id, "words": words, **extra})
        assert r.status_code == 200, r.text
        assert r.headers["content-type"] == "audio/wav"
        audio, sr = fx.decode_wav(r.content)
        return audio, sr

    normal, sr = say(syllables_per_second=5.0)
    assert len(normal) / sr > 0.8
    assert np.max(np.abs(normal)) > 0.1

    slow, _ = say(syllables_per_second=3.0)
    assert len(slow) > len(normal) * 1.3

    quiet, _ = say(syllables_per_second=5.0, volume_db=-10)
    assert np.sqrt(np.mean(quiet**2)) < np.sqrt(np.mean(normal**2)) / 2


def test_errors(client):
    assert client.post("/v1/synthesize", json={"voice_id": "doesnotexist", "words": [{"phonemes": ["a"]}]}).status_code == 404
    assert client.post("/v1/voices", json={"name": "x", "references": [{"audio_wav_base64": "@@"}]}).status_code == 400
    short = b64wav(voice_like(0.3, 200))
    assert client.post("/v1/voices", json={"name": "x", "references": [{"audio_wav_base64": short}]}).status_code == 422
