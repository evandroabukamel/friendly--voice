"""Persistence of cloned voices: the model conditionals plus the reference
window they were built from (kept for listening and auditing)."""

from __future__ import annotations

import json
import re
import uuid
from dataclasses import asdict, dataclass
from pathlib import Path

VOICE_ID = re.compile(r"^[a-z0-9-]{8,64}$")
CONDITIONALS = "conditionals.pt"


@dataclass
class VoiceMeta:
    voice_id: str
    name: str
    language: str
    reference_count: int
    reference_seconds: float
    median_pitch_hz: float = 0.0


class VoiceStore:
    def __init__(self, root: Path):
        self.root = root
        self.root.mkdir(parents=True, exist_ok=True)

    def _dir(self, voice_id: str) -> Path:
        if not VOICE_ID.match(voice_id):
            raise KeyError(voice_id)
        return self.root / voice_id

    def save(self, meta: VoiceMeta, conditionals, reference_wav: bytes) -> None:
        directory = self._dir(meta.voice_id)
        directory.mkdir(parents=True, exist_ok=True)
        conditionals.save(directory / CONDITIONALS)
        (directory / "reference.wav").write_bytes(reference_wav)
        (directory / "voice.json").write_text(json.dumps(asdict(meta), ensure_ascii=False, indent=2), encoding="utf-8")

    def load(self, voice_id: str, device: str = "cpu"):
        """Returns ``(meta, conditionals)``; ``KeyError`` for an unknown voice
        (including voices made by older engine versions)."""
        from chatterbox.mtl_tts import Conditionals

        directory = self._dir(voice_id)
        if not (directory / CONDITIONALS).exists():
            raise KeyError(voice_id)
        meta = VoiceMeta(**json.loads((directory / "voice.json").read_text(encoding="utf-8")))
        return meta, Conditionals.load(directory / CONDITIONALS, map_location=device).to(device)

    @staticmethod
    def new_id() -> str:
        return uuid.uuid4().hex
