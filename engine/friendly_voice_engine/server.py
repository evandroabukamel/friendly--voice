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
from .phonemes import Word


class Reference(BaseModel):
    audio_wav_base64: str
    transcript: str = ""


class EnrollRequest(BaseModel):
    name: str
    language: str = "pt-BR"
    references: list[Reference] = Field(min_length=1)


class WordIn(BaseModel):
    phonemes: list[str]
    stress: int | None = None
    pause_after: bool = False


class SynthesizeRequest(BaseModel):
    voice_id: str
    language: str = "pt-BR"
    text: str = ""
    ssml: str = ""
    #: Accented pronunciation, one entry per word (preferred).
    words: list[WordIn] | None = None
    #: Fallback: IPA per word, without stress information.
    ipa_words: list[str] = []
    pitch_hz: float = 0.0
    syllables_per_second: float = 0.0
    style: str = "natural"
    pitch_shift_semitones: float = 0.0
    volume_db: float = 0.0


class AudioRequest(BaseModel):
    language: str = "pt-BR"
    audio_wav_base64: str
    transcript: str | None = None


def _decode(b64: str) -> tuple[np.ndarray, int]:
    try:
        return fx.decode_wav(base64.b64decode(b64, validate=True))
    except (binascii.Error, RuntimeError, ValueError) as exc:
        raise HTTPException(400, f"invalid audio: {exc}") from exc


def _optional(module: str, feature: str) -> None:
    if importlib.util.find_spec(module) is None:
        raise HTTPException(
            501, f"{feature} is not installed on this server (pip install 'friendly-voice-engine[{feature}]')"
        )


@lru_cache(maxsize=1)
def _whisper():
    from faster_whisper import WhisperModel

    return WhisperModel("small", device="cpu", compute_type="int8")


@lru_cache(maxsize=1)
def _phoneme_recognizer():
    from transformers import pipeline

    # Multilingual phone recognizer that outputs espeak-style IPA (Apache-2.0).
    return pipeline("automatic-speech-recognition", model="facebook/wav2vec2-xlsr-53-espeak-cv-ft")


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
            "base_voice_syllables_per_second": round(e.piper.syllables_per_second, 2),
            "transcribe": importlib.util.find_spec("faster_whisper") is not None,
            "phonemes": importlib.util.find_spec("transformers") is not None,
        }

    @app.post("/v1/voices")
    def enroll(req: EnrollRequest) -> dict:
        references = [_decode(r.audio_wav_base64) for r in req.references]
        try:
            meta = get_engine().enroll(req.name, references, req.language)
        except ValueError as exc:
            raise HTTPException(422, str(exc)) from exc
        return {"voice_id": meta.voice_id, "reference_count": meta.reference_count,
                "reference_seconds": meta.reference_seconds}

    @app.post("/v1/synthesize")
    def synthesize(req: SynthesizeRequest) -> Response:
        if req.words:
            words = [Word(w.phonemes, w.stress, w.pause_after) for w in req.words]
        else:
            words = [Word(list(ipa)) for ipa in req.ipa_words if ipa]
        try:
            audio, sample_rate = get_engine().synthesize(
                SynthesisRequest(
                    voice_id=req.voice_id,
                    words=words,
                    syllables_per_second=req.syllables_per_second,
                    pitch_shift_semitones=req.pitch_shift_semitones,
                    volume_db=req.volume_db,
                )
            )
        except KeyError as exc:
            raise HTTPException(404, f"unknown voice_id: {req.voice_id}") from exc
        except ValueError as exc:
            raise HTTPException(422, str(exc)) from exc
        return Response(fx.encode_wav(audio, sample_rate), media_type="audio/wav")

    @app.post("/v1/transcribe")
    def transcribe(req: AudioRequest) -> dict:
        _optional("faster_whisper", "transcribe")
        audio, sample_rate = _decode(req.audio_wav_base64)
        audio16 = fx.resample(audio, sample_rate, 16_000)
        segments, _ = _whisper().transcribe(audio16, language=req.language.split("-")[0])
        return {"text": " ".join(s.text.strip() for s in segments).strip()}

    @app.post("/v1/phonemes")
    def phonemes(req: AudioRequest) -> dict:
        _optional("transformers", "phonemes")
        audio, sample_rate = _decode(req.audio_wav_base64)
        audio16 = fx.resample(audio, sample_rate, 16_000)
        result = _phoneme_recognizer()(audio16)
        return {"ipa": result["text"]}

    return app
