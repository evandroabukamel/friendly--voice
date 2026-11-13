"""Downloads the model weights into the Hugging Face cache (``HF_HOME``).

* Chatterbox Multilingual (ResembleAI/chatterbox) — MIT license, code and weights.
* faster-whisper ``small`` (only with the ``transcribe`` extra) — MIT license.
"""

from __future__ import annotations

import importlib.util

#: Files of ResembleAI/chatterbox used by the multilingual model.
CHATTERBOX_REPO = "ResembleAI/chatterbox"
CHATTERBOX_FILES = [
    "ve.pt", "t3_mtl23ls_v2.safetensors", "s3gen.pt",
    "grapheme_mtl_merged_expanded_v1.json", "conds.pt", "Cangjie5_TC.json",
]
WHISPER_MODEL = "small"


def download_all() -> None:
    from huggingface_hub import snapshot_download

    snapshot_download(repo_id=CHATTERBOX_REPO, allow_patterns=CHATTERBOX_FILES)
    if importlib.util.find_spec("faster_whisper") is not None:
        from faster_whisper.utils import download_model

        download_model(WHISPER_MODEL)
