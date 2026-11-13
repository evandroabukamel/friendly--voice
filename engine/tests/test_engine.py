"""Integration tests with the real model (skipped when chatterbox-tts is not
installed). The first run downloads the weights (~3 GB)."""

import base64
import importlib.util
import os

import numpy as np
import pytest

from friendly_voice_engine import audio as fx
from friendly_voice_engine.config import Settings

pytestmark = pytest.mark.skipif(
    importlib.util.find_spec("chatterbox") is None, reason="chatterbox-tts not installed"
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

    os.environ["FV_VOICES_DIR"] = str(tmp_path_factory.mktemp("voices"))
    return TestClient(create_app(settings=Settings.from_env()))


def b64wav(audio: np.ndarray, sr: int = 16_000) -> str:
    return base64.b64encode(fx.encode_wav(audio, sr)).decode()


def test_health(client):
    body = client.get("/health").json()
    assert body["status"] == "ok"
    assert "pt" in body["languages"]


def test_clone_and_speak_with_tone(client):
    enrolled = client.post(
        "/v1/voices",
        json={"name": "Vó", "references": [{"audio_wav_base64": b64wav(voice_like(12, 210))}]},
    )
    assert enrolled.status_code == 200, enrolled.text
    body = enrolled.json()
    assert abs(body["median_pitch_hz"] - 210) < 15

    def say(**extra):
        r = client.post(
            "/v1/synthesize",
            json={"voice_id": body["voice_id"], "text": "Bom dia, meu amor. Vai dar tudo certo.", **extra},
        )
        assert r.status_code == 200, r.text
        assert r.headers["content-type"] == "audio/wav"
        return fx.decode_wav(r.content)

    normal, sr = say()
    assert len(normal) / sr > 1.0
    quiet, _ = say(volume_db=-10)
    assert np.sqrt(np.mean(quiet**2)) < np.sqrt(np.mean(normal**2)) / 2


def test_errors(client):
    ok = {"voice_id": "doesnotexist", "text": "oi"}
    assert client.post("/v1/synthesize", json=ok).status_code == 404
    assert client.post("/v1/voices", json={"name": "x", "references": [{"audio_wav_base64": "@@"}]}).status_code == 400
    short = b64wav(voice_like(0.3, 200))
    assert client.post("/v1/voices", json={"name": "x", "references": [{"audio_wav_base64": short}]}).status_code == 422
    unsupported = {"name": "x", "language": "tlh", "references": [{"audio_wav_base64": b64wav(voice_like(3, 200))}]}
    assert client.post("/v1/voices", json=unsupported).status_code == 422
