"""Zero-shot voice cloning with Chatterbox Multilingual.

Enrollment turns the person's recordings into the model's *conditionals*:

* the **speaker embedding** is averaged over every recording, so the timbre
  is the person's overall voice rather than one take;
* the **reference window** (the 10 s of speech the model imitates for
  intonation, rhythm and accent) is the candidate window whose own embedding
  is closest to that average.

Synthesis reads the text in the voice's language, sentence by sentence;
a take that is cut off or rambles is retried with another seed.
"""

from __future__ import annotations

import logging
import threading
from dataclasses import dataclass, field

import numpy as np
import torch

from . import audio as fx
from . import references as refs
from . import text as tx
from .config import Settings
from .languages import model_language
from .voices import VoiceMeta, VoiceStore

log = logging.getLogger("uvicorn.error")

#: References shorter than this carry too little voice information.
MIN_REFERENCE_SECONDS = 1.0
#: Pitch shift is clamped: beyond it the voice stops sounding like the person.
MAX_PITCH_SHIFT = 8.0
#: Takes per chunk before keeping the most plausible one.
MAX_ATTEMPTS = 3
#: Pause between chunks (seconds).
CHUNK_GAP = 0.25
#: Tone style → (exaggeration, cfg_weight). Higher exaggeration = more
#: expressive and a bit faster; lower cfg_weight = slower, calmer pacing.
STYLES = {
    "natural": (0.5, 0.5),
    "warm": (0.45, 0.4),
    "cheerful": (0.7, 0.45),
    "calm": (0.35, 0.3),
    "firm": (0.6, 0.6),
    "whisper": (0.3, 0.35),
}


@dataclass
class SynthesisRequest:
    voice_id: str
    text: str
    language: str = "pt-BR"
    style: str = "natural"
    pitch_shift_semitones: float = 0.0
    volume_db: float = 0.0
    seed: int = 0


def resolve_device(device: str) -> str:
    if device == "auto":
        return "cuda" if torch.cuda.is_available() else "cpu"
    return device


@dataclass
class Engine:
    settings: Settings
    voices: VoiceStore = field(init=False)
    _lock: threading.Lock = field(init=False, default_factory=threading.Lock)

    def __post_init__(self) -> None:
        from chatterbox.mtl_tts import ChatterboxMultilingualTTS

        self.device = resolve_device(self.settings.device)
        self.model = ChatterboxMultilingualTTS.from_pretrained(device=self.device)
        self.voices = VoiceStore(self.settings.voices_dir)

    @property
    def sample_rate(self) -> int:
        return int(self.model.sr)

    # ------------------------------------------------------------- cloning

    def _embed(self, clips16: list[np.ndarray]) -> np.ndarray:
        return np.asarray(self.model.ve.embeds_from_wavs(clips16, sample_rate=refs.ANALYSIS_RATE))

    def enroll(self, name: str, references: list[tuple[np.ndarray, int]], language: str = "pt-BR") -> VoiceMeta:
        model_language(language)  # refuse an unsupported language before the heavy work
        clips16 = []
        total = 0.0
        for audio, sample_rate in references:
            seconds = len(audio) / sample_rate
            if seconds < MIN_REFERENCE_SECONDS:
                continue
            clips16.append(fx.normalize_peak(fx.resample(audio, sample_rate, refs.ANALYSIS_RATE)))
            total += seconds
        if not clips16:
            raise ValueError(f"no reference audio longer than {MIN_REFERENCE_SECONDS:.0f}s")

        with self._lock:
            speaker = self._embed(clips16).mean(axis=0)
            windows = [w for clip in clips16 for w in refs.candidate_windows(clip, refs.ANALYSIS_RATE)]
            window = windows[refs.closest(self._embed(windows), speaker)]
            conditionals = self._conditionals(window, speaker)

        meta = VoiceMeta(
            voice_id=self.voices.new_id(),
            name=name,
            language=language,
            reference_count=len(clips16),
            reference_seconds=round(total, 2),
            median_pitch_hz=round(refs.median_pitch(np.concatenate(clips16), refs.ANALYSIS_RATE), 1),
        )
        self.voices.save(meta, conditionals, fx.encode_wav(window, refs.ANALYSIS_RATE))
        return meta

    def _conditionals(self, window16: np.ndarray, speaker: np.ndarray):
        from chatterbox.models.t3.modules.cond_enc import T3Cond
        from chatterbox.mtl_tts import Conditionals

        m = self.model
        window24 = fx.resample(window16, refs.ANALYSIS_RATE, m.sr)
        gen = m.s3gen.embed_ref(window24[: m.DEC_COND_LEN], m.sr, device=self.device)
        tokens, _ = m.s3gen.tokenizer.forward([window16[: m.ENC_COND_LEN]], max_len=m.t3.hp.speech_cond_prompt_len)
        t3 = T3Cond(
            speaker_emb=torch.from_numpy(speaker.astype(np.float32)).reshape(1, -1),
            cond_prompt_speech_tokens=torch.atleast_2d(tokens),
            emotion_adv=0.5 * torch.ones(1, 1, 1),
        )
        return Conditionals(t3, gen).to(self.device)

    # ----------------------------------------------------------- synthesis

    def synthesize(self, request: SynthesisRequest) -> tuple[np.ndarray, int]:
        language = model_language(request.language)
        pieces = tx.chunks(request.text)
        if not pieces:
            raise ValueError("nothing to say")
        exaggeration, cfg_weight = STYLES.get(request.style, STYLES["natural"])
        _, conditionals = self.voices.load(request.voice_id, self.device)
        sr = self.sample_rate
        out: list[np.ndarray] = []
        with self._lock:
            self.model.conds = conditionals
            for i, piece in enumerate(pieces):
                if out:
                    out.append(fx.silence(CHUNK_GAP, sr))
                out.append(self._take(piece, language, exaggeration, cfg_weight, request.seed + 1000 * i))
        speech = np.concatenate(out)
        shift = float(np.clip(request.pitch_shift_semitones, -MAX_PITCH_SHIFT, MAX_PITCH_SHIFT))
        speech = fx.pitch_shift(speech, sr, shift)
        return fx.apply_gain(fx.normalize_peak(speech, 0.8), request.volume_db), sr

    def _take(self, piece: str, language: str, exaggeration: float, cfg_weight: float, seed: int) -> np.ndarray:
        best: np.ndarray | None = None
        for attempt in range(MAX_ATTEMPTS):
            torch.manual_seed(seed + attempt)
            wav = self.model.generate(
                piece, language_id=language, exaggeration=exaggeration, cfg_weight=cfg_weight
            ).squeeze(0).cpu().numpy()
            seconds = len(wav) / self.sample_rate
            log.info("take %d: %.1fs for %d chars (expected %.1fs)", attempt + 1, seconds, len(piece),
                     tx.expected_seconds(piece))
            if tx.plausible(piece, seconds):
                return wav
            expected = tx.expected_seconds(piece)
            if best is None or abs(seconds - expected) < abs(len(best) / self.sample_rate - expected):
                best = wav
        return best
