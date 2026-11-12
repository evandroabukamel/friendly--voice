"""Orchestrates base speech (Piper) + timbre conversion (OpenVoice) + tone."""

from __future__ import annotations

from dataclasses import dataclass, field

import numpy as np

from . import audio as fx
from .config import Settings
from .download import missing
from .phonemes import Word
from .piper_voice import CALIBRATION, PiperVoice
from .timbre import ToneColorConverter
from .voices import VoiceMeta, VoiceStore

#: References shorter than this carry too little timbre information.
MIN_REFERENCE_SECONDS = 1.0
#: Pitch shift is clamped: beyond it the voice stops sounding like the person.
MAX_PITCH_SHIFT = 8.0


@dataclass
class SynthesisRequest:
    voice_id: str
    words: list[Word]
    syllables_per_second: float = 0.0
    pitch_shift_semitones: float = 0.0
    volume_db: float = 0.0


@dataclass
class Engine:
    settings: Settings
    piper: PiperVoice = field(init=False)
    converter: ToneColorConverter = field(init=False)
    voices: VoiceStore = field(init=False)

    def __post_init__(self) -> None:
        absent = missing(self.settings.models_dir)
        if absent:
            names = ", ".join(m.relative_path for m in absent)
            raise RuntimeError(f"missing model files ({names}); run: friendly-voice-engine download")
        m = self.settings.models_dir
        self.piper = PiperVoice(m / "piper/pt_BR-faber-medium.onnx", m / "piper/pt_BR-faber-medium.onnx.json")
        self.converter = ToneColorConverter(
            m / "openvoice/config.json", m / "openvoice/checkpoint.pth", self.settings.device
        )
        self.voices = VoiceStore(self.settings.voices_dir)
        # Tone color of the base voice, needed as the conversion source.
        base = self.piper.synthesize(CALIBRATION)
        self.base_embedding = self.converter.embedding(
            [self.converter.load(base, self.piper.sample_rate)]
        )

    # ------------------------------------------------------------- cloning

    def enroll(self, name: str, references: list[tuple[np.ndarray, int]], language: str = "pt-BR") -> VoiceMeta:
        clips = []
        total = 0.0
        for audio, sample_rate in references:
            seconds = len(audio) / sample_rate
            if seconds < MIN_REFERENCE_SECONDS:
                continue
            clips.append(self.converter.load(fx.normalize_peak(audio), sample_rate))
            total += seconds
        if not clips:
            raise ValueError(f"no reference audio longer than {MIN_REFERENCE_SECONDS:.0f}s")
        meta = VoiceMeta(
            voice_id=self.voices.new_id(),
            name=name,
            language=language,
            reference_count=len(clips),
            reference_seconds=round(total, 2),
        )
        self.voices.save(meta, self.converter.embedding(clips))
        return meta

    # ----------------------------------------------------------- synthesis

    def length_scale(self, syllables_per_second: float) -> float:
        if syllables_per_second <= 0:
            return 1.0
        return float(np.clip(self.piper.syllables_per_second / syllables_per_second, 0.5, 2.0))

    def synthesize(self, request: SynthesisRequest) -> tuple[np.ndarray, int]:
        _, target = self.voices.load(request.voice_id)
        if not request.words:
            raise ValueError("nothing to say")
        base = self.piper.synthesize(request.words, self.length_scale(request.syllables_per_second))
        sr = self.converter.sample_rate
        base = self.converter.load(base, self.piper.sample_rate)
        converted = self.converter.convert(base, self.base_embedding, target, tau=self.settings.tau)
        shift = float(np.clip(request.pitch_shift_semitones, -MAX_PITCH_SHIFT, MAX_PITCH_SHIFT))
        converted = fx.pitch_shift(converted, sr, shift)
        converted = fx.apply_gain(fx.normalize_peak(converted, 0.8), request.volume_db)
        return converted, sr
