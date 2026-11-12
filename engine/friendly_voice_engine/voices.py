"""Persistence of cloned voices: one speaker embedding per voice."""

from __future__ import annotations

import json
import re
import uuid
from dataclasses import asdict, dataclass
from pathlib import Path

import torch

VOICE_ID = re.compile(r"^[a-z0-9-]{8,64}$")


@dataclass
class VoiceMeta:
    voice_id: str
    name: str
    language: str
    reference_count: int
    reference_seconds: float


class VoiceStore:
    def __init__(self, root: Path):
        self.root = root
        self.root.mkdir(parents=True, exist_ok=True)

    def _dir(self, voice_id: str) -> Path:
        if not VOICE_ID.match(voice_id):
            raise KeyError(voice_id)
        return self.root / voice_id

    def save(self, meta: VoiceMeta, embedding: torch.Tensor) -> None:
        directory = self._dir(meta.voice_id)
        directory.mkdir(parents=True, exist_ok=True)
        torch.save(embedding, directory / "embedding.pt")
        (directory / "voice.json").write_text(json.dumps(asdict(meta), ensure_ascii=False, indent=2), encoding="utf-8")

    def load(self, voice_id: str) -> tuple[VoiceMeta, torch.Tensor]:
        directory = self._dir(voice_id)
        if not (directory / "embedding.pt").exists():
            raise KeyError(voice_id)
        meta = VoiceMeta(**json.loads((directory / "voice.json").read_text(encoding="utf-8")))
        return meta, torch.load(directory / "embedding.pt", map_location="cpu")

    @staticmethod
    def new_id() -> str:
        return uuid.uuid4().hex
