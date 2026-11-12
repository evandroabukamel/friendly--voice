"""Runtime configuration, read from environment variables."""

from __future__ import annotations

import os
from dataclasses import dataclass
from pathlib import Path


def _default_home() -> Path:
    return Path(os.environ.get("XDG_CACHE_HOME", Path.home() / ".cache")) / "friendly-voice-engine"


@dataclass(frozen=True)
class Settings:
    #: Where model weights are stored (see ``friendly-voice-engine download``).
    models_dir: Path
    #: Where cloned voices (speaker embeddings + metadata) are stored.
    voices_dir: Path
    #: Torch device for the tone color converter ("cpu", "cuda", "cuda:0"...).
    device: str
    #: OpenVoice "tau": lower keeps more of the target timbre, higher more of the source.
    tau: float

    @classmethod
    def from_env(cls) -> "Settings":
        home = Path(os.environ.get("FV_ENGINE_HOME", _default_home()))
        return cls(
            models_dir=Path(os.environ.get("FV_MODELS_DIR", home / "models")),
            voices_dir=Path(os.environ.get("FV_VOICES_DIR", home / "voices")),
            device=os.environ.get("FV_DEVICE", "cpu"),
            tau=float(os.environ.get("FV_TAU", "0.3")),
        )
