"""Tone color (timbre) conversion with OpenVoice V2.

Adapted from ``openvoice.api.ToneColorConverter`` (MIT license, Copyright
2024 MyShell.ai). We use the model directly so that only the converter's
modules are imported (not OpenVoice's multilingual text front-end, which
pulls many unrelated dependencies).
"""

from __future__ import annotations

from pathlib import Path

import librosa
import numpy as np
import torch
from openvoice import utils
from openvoice.mel_processing import spectrogram_torch
from openvoice.models import SynthesizerTrn


class ToneColorConverter:
    def __init__(self, config_path: Path, checkpoint_path: Path, device: str = "cpu"):
        if device.startswith("cuda") and not torch.cuda.is_available():
            raise RuntimeError("FV_DEVICE=cuda but CUDA is not available")
        self.hps = utils.get_hparams_from_file(str(config_path))
        self.device = device
        self.model = SynthesizerTrn(
            len(getattr(self.hps, "symbols", [])),
            self.hps.data.filter_length // 2 + 1,
            n_speakers=self.hps.data.n_speakers,
            **self.hps.model,
        ).to(device)
        self.model.eval()
        checkpoint = torch.load(str(checkpoint_path), map_location=torch.device(device))
        self.model.load_state_dict(checkpoint["model"], strict=False)

    @property
    def sample_rate(self) -> int:
        return int(self.hps.data.sampling_rate)

    def _spectrogram(self, audio: np.ndarray) -> torch.Tensor:
        y = torch.FloatTensor(audio).to(self.device).unsqueeze(0)
        d = self.hps.data
        return spectrogram_torch(
            y, d.filter_length, d.sampling_rate, d.hop_length, d.win_length, center=False
        ).to(self.device)

    def load(self, path_or_audio, sample_rate: int | None = None) -> np.ndarray:
        """Loads a file, or resamples an in-memory clip, to the model rate."""
        if isinstance(path_or_audio, (str, Path)):
            audio, _ = librosa.load(str(path_or_audio), sr=self.sample_rate)
            return audio
        if sample_rate != self.sample_rate:
            return librosa.resample(path_or_audio, orig_sr=sample_rate, target_sr=self.sample_rate)
        return path_or_audio

    @torch.no_grad()
    def embedding(self, clips: list[np.ndarray]) -> torch.Tensor:
        """Speaker embedding ("tone color") averaged over several clips."""
        if not clips:
            raise ValueError("at least one clip is required")
        embeddings = [
            self.model.ref_enc(self._spectrogram(clip).transpose(1, 2)).unsqueeze(-1) for clip in clips
        ]
        return torch.stack(embeddings).mean(0).cpu()

    @torch.no_grad()
    def convert(self, audio: np.ndarray, source: torch.Tensor, target: torch.Tensor, tau: float = 0.3) -> np.ndarray:
        spec = self._spectrogram(audio)
        lengths = torch.LongTensor([spec.size(-1)]).to(self.device)
        out = self.model.voice_conversion(
            spec, lengths, sid_src=source.to(self.device), sid_tgt=target.to(self.device), tau=tau
        )[0]
        return out[0, 0].cpu().float().numpy()
