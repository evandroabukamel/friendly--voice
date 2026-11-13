"""Runtime configuration, read from environment variables."""

from __future__ import annotations

import os
from dataclasses import dataclass
from pathlib import Path


def _default_home() -> Path:
    return Path(os.environ.get("XDG_CACHE_HOME", Path.home() / ".cache")) / "friendly-voice-engine"


@dataclass(frozen=True)
class Settings:
    #: Where cloned voices (model conditionals + metadata) are stored.
    voices_dir: Path
    #: Torch device: "auto" (GPU when there is one), "cpu", "cuda", "cuda:0"...
    device: str

    @classmethod
    def from_env(cls) -> "Settings":
        home = Path(os.environ.get("FV_ENGINE_HOME", _default_home()))
        return cls(
            voices_dir=Path(os.environ.get("FV_VOICES_DIR", home / "voices")),
            device=os.environ.get("FV_DEVICE", "auto"),
        )
