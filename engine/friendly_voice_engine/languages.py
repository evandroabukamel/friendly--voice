"""Languages the speech model can speak (Chatterbox Multilingual)."""

from __future__ import annotations

SUPPORTED = {
    "ar": "Arabic", "da": "Danish", "de": "German", "el": "Greek", "en": "English",
    "es": "Spanish", "fi": "Finnish", "fr": "French", "he": "Hebrew", "hi": "Hindi",
    "it": "Italian", "ja": "Japanese", "ko": "Korean", "ms": "Malay", "nl": "Dutch",
    "no": "Norwegian", "pl": "Polish", "pt": "Portuguese", "ru": "Russian",
    "sv": "Swedish", "sw": "Swahili", "tr": "Turkish", "zh": "Chinese",
}


def model_language(tag: str) -> str:
    """``pt-BR`` → ``pt``. Raises ``ValueError`` for an unsupported language."""
    code = tag.strip().replace("_", "-").split("-")[0].lower()
    if code not in SUPPORTED:
        raise ValueError(f"unsupported language '{tag}'; supported: {', '.join(sorted(SUPPORTED))}")
    return code
