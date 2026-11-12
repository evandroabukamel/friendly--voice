"""Downloads the model weights, verifying their SHA-256 checksums.

* OpenVoice V2 tone color converter — MIT license (myshell-ai/OpenVoiceV2).
* Piper ``pt_BR-faber-medium`` — Brazilian Portuguese base voice (rhasspy/piper-voices).
  Check the voice's MODEL_CARD for its dataset license before distributing it.
"""

from __future__ import annotations

import hashlib
import shutil
import tempfile
import urllib.request
from dataclasses import dataclass
from pathlib import Path

HF = "https://huggingface.co"


@dataclass(frozen=True)
class ModelFile:
    relative_path: str
    url: str
    sha256: str


FILES = [
    ModelFile(
        "openvoice/config.json",
        f"{HF}/myshell-ai/OpenVoiceV2/resolve/main/converter/config.json",
        "9dfff60350b8c63f2c664efd92a61b2516efb22671466960f0e5dfebd881fa47",
    ),
    ModelFile(
        "openvoice/checkpoint.pth",
        f"{HF}/myshell-ai/OpenVoiceV2/resolve/main/converter/checkpoint.pth",
        "9652c27e92b6b2a91632590ac9962ef7ae2b712e5c5b7f4c34ec55ee2b37ab9e",
    ),
    ModelFile(
        "piper/pt_BR-faber-medium.onnx",
        f"{HF}/rhasspy/piper-voices/resolve/main/pt/pt_BR/faber/medium/pt_BR-faber-medium.onnx",
        "858555e3a064209c57088fe6bd70c4c3dc54d03eaa00c45d5ecaf43a33f95aa7",
    ),
    ModelFile(
        "piper/pt_BR-faber-medium.onnx.json",
        f"{HF}/rhasspy/piper-voices/resolve/main/pt/pt_BR/faber/medium/pt_BR-faber-medium.onnx.json",
        "7e694de195ae3fc36dd732c445eb04fb49b649854893cb5506b978f0d50a1d6f",
    ),
]


def sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def missing(models_dir: Path) -> list[ModelFile]:
    return [m for m in FILES if not (models_dir / m.relative_path).exists()]


def download_all(models_dir: Path, log=print) -> None:
    for model in FILES:
        target = models_dir / model.relative_path
        if target.exists() and sha256(target) == model.sha256:
            log(f"ok        {model.relative_path}")
            continue
        target.parent.mkdir(parents=True, exist_ok=True)
        log(f"download  {model.relative_path}")
        with tempfile.NamedTemporaryFile(dir=target.parent, delete=False) as tmp:
            with urllib.request.urlopen(model.url) as response:
                shutil.copyfileobj(response, tmp)
            tmp_path = Path(tmp.name)
        digest = sha256(tmp_path)
        if digest != model.sha256:
            tmp_path.unlink()
            raise RuntimeError(f"checksum mismatch for {model.relative_path}: {digest}")
        tmp_path.replace(target)
