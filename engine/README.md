# Friendly Voice engine

Self-hosted voice-cloning server used by the Friendly Voice app (Rust CLI and
Android) through `--features http-engine`. It implements the contract in
[`docs/engine-api.md`](../docs/engine-api.md).

## How it works

| Step | Component | License |
|---|---|---|
| Speaker embedding averaged over **all** recordings | Chatterbox voice encoder | MIT |
| Reference: the 10 s window of speech closest to that average (continuous speech, no clipping) | `references.py` | — |
| Text → speech in the person's voice, in one of 23 languages | Chatterbox Multilingual (T3 + S3Gen) | MIT |
| Tone of the message | style → `exaggeration`/`cfg_weight`; pitch shift with Praat PSOLA via `praat-parselmouth` (shifts under 1 semitone skipped); gain | GPL-3.0 (Praat) |
| Transcription *(optional)* | faster-whisper `small`, on the CPU | MIT |

Zero-shot: no training. The model reads the text itself and imitates the
person's timbre, intonation, rhythm and accent from the reference. Long texts
are spoken sentence by sentence (≤ 220 characters); a take whose duration
shows it was cut off or rambled is retried with another seed. Every output
carries Resemble AI's imperceptible Perth watermark.

## Install

Requires Python 3.10 or 3.11.

```bash
python -m venv .venv && . .venv/bin/activate
pip install torch==2.6.0 torchaudio==2.6.0 --index-url https://download.pytorch.org/whl/cu124   # or .../whl/cpu
pip install -e ".[transcribe,dev]"
friendly-voice-engine download      # ~3 GB into the Hugging Face cache (HF_HOME)
friendly-voice-engine serve --host 127.0.0.1 --port 8020
```

Docker (weights baked into the image):

```bash
docker build -t friendly-voice-engine .
docker run --gpus all -p 8020:8020 -v fv-voices:/data friendly-voice-engine   # NVIDIA GPU
docker run -p 8020:8020 -v fv-voices:/data friendly-voice-engine              # CPU
```

Without the `transcribe` extra, `/v1/transcribe` answers `501` and samples are
stored without a transcript (it is not needed to clone the voice).

## Configuration

| Variable | Default | Meaning |
|---|---|---|
| `FV_ENGINE_HOME` | `~/.cache/friendly-voice-engine` | base folder |
| `FV_VOICES_DIR` | `$FV_ENGINE_HOME/voices` | cloned voices (conditionals + reference window + metadata) |
| `FV_DEVICE` | `auto` | `auto` (GPU when there is one), `cpu`, `cuda`, `cuda:0`... |
| `HF_HOME` | `~/.cache/huggingface` | model weights |

Speed: about 4 s per sentence on a 6 GB laptop GPU (RTX 4050); several times
slower on CPU. Voices made by engine 0.1 (OpenVoice) are not compatible:
generate them again.

## Tests

```bash
pytest -q                          # integration tests are skipped without chatterbox-tts
../scripts/engine-e2e.sh           # Rust CLI ⇄ running server, end to end
```

## Recording tips

* 3 to 10 recordings of 10–60 s each, one speaker, little background noise.
* Natural speech (a conversation or a voice message) carries more of the
  person's intonation and accent than reading aloud.
* Register the person with the language they speak in the recordings.

## Licenses of the models

* Chatterbox / Chatterbox Multilingual code and weights: MIT (Resemble AI).
* faster-whisper and the Whisper `small` weights: MIT.
* XTTS v2 was **not** used: its Coqui Public Model License allows only
  non-commercial use (see ADR 0006/0007).
