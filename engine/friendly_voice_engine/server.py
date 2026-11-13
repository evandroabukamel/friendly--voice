"""HTTP API consumed by the Rust ``HttpVoiceEngine`` (see docs/engine-api.md)."""

from __future__ import annotations

import base64
import binascii
import importlib.util
from functools import lru_cache

import numpy as np
from fastapi import FastAPI, HTTPException
from fastapi.responses import Response
from pydantic import BaseModel, Field

from . import __version__
from . import audio as fx
from .config import Settings
from .engine import Engine, SynthesisRequest
from .languages import SUPPORTED, model_language


class Reference(BaseModel):
    audio_wav_base64: str
    transcript: str | None = None


class EnrollRequest(BaseModel):
    name: str
    language: str = "pt-BR"
    #: Detected by the app from the recordings ("female" / "male"); informative.
    gender: str | None = None
    references: list[Reference] = Field(min_length=1)


class SynthesizeRequest(BaseModel):
    voice_id: str
    language: str = "pt-BR"
    text: str
    style: str = "natural"
    pitch_shift_semitones: float = 0.0
    volume_db: float = 0.0


class AudioRequest(BaseModel):
    language: str = "pt-BR"
    audio_wav_base64: str


def _decode(b64: str) -> tuple[np.ndarray, int]:
    try:
        return fx.decode_wav(base64.b64decode(b64, validate=True))
    except (binascii.Error, RuntimeError, ValueError) as exc:
        raise HTTPException(400, f"invalid audio: {exc}") from exc


@lru_cache(maxsize=1)
def _whisper():
    from faster_whisper import WhisperModel

    # On the CPU, so that it never competes with the voice model for GPU memory.
    return WhisperModel("small", device="cpu", compute_type="int8")


def create_app(engine: Engine | None = None, settings: Settings | None = None) -> FastAPI:
    app = FastAPI(title="Friendly Voice engine", version=__version__)
    state: dict[str, Engine] = {}

    def get_engine() -> Engine:
        if "engine" not in state:
            state["engine"] = engine or Engine(settings or Settings.from_env())
        return state["engine"]

    @app.get("/health")
    def health() -> dict:
        e = get_engine()
        return {
            "status": "ok",
            "version": __version__,
            "device": e.device,
            "languages": sorted(SUPPORTED),
            "transcribe": importlib.util.find_spec("faster_whisper") is not None,
        }

    @app.post("/v1/voices")
    def enroll(req: EnrollRequest) -> dict:
        references = [_decode(r.audio_wav_base64) for r in req.references]
        try:
            meta = get_engine().enroll(req.name, references, req.language)
        except ValueError as exc:
            raise HTTPException(422, str(exc)) from exc
        return {
            "voice_id": meta.voice_id,
            "reference_count": meta.reference_count,
            "reference_seconds": meta.reference_seconds,
            "median_pitch_hz": meta.median_pitch_hz,
        }

    @app.post("/v1/synthesize")
    def synthesize(req: SynthesizeRequest) -> Response:
        try:
            audio, sample_rate = get_engine().synthesize(
                SynthesisRequest(
                    voice_id=req.voice_id,
                    text=req.text,
                    language=req.language,
                    style=req.style,
                    pitch_shift_semitones=req.pitch_shift_semitones,
                    volume_db=req.volume_db,
                )
            )
        except KeyError as exc:
            raise HTTPException(
                404, f"unknown voice_id: {req.voice_id} (generate the voice again)"
            ) from exc
        except ValueError as exc:
            raise HTTPException(422, str(exc)) from exc
        return Response(fx.encode_wav(audio, sample_rate), media_type="audio/wav")

    @app.post("/v1/transcribe")
    def transcribe(req: AudioRequest) -> dict:
        if importlib.util.find_spec("faster_whisper") is None:
            raise HTTPException(
                501, "transcribe is not installed on this server (pip install 'friendly-voice-engine[transcribe]')"
            )
        try:
            language = model_language(req.language)
        except ValueError as exc:
            raise HTTPException(422, str(exc)) from exc
        audio, sample_rate = _decode(req.audio_wav_base64)
        segments, _ = _whisper().transcribe(fx.resample(audio, sample_rate, 16_000), language=language)
        return {"text": " ".join(s.text.strip() for s in segments).strip()}

    return app
