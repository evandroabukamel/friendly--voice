# Friendly Voice engine

Self-hosted voice-cloning server used by the Friendly Voice app (Rust CLI and
Android) through `--features http-engine`. It implements the contract in
[`docs/engine-api.md`](../docs/engine-api.md).

## How it works

| Step | Component | License |
|---|---|---|
| Timbre embedding from the uploaded recordings | OpenVoice V2 tone color encoder | MIT |
| Base speech from the **accented phonemes** sent by the app | Piper `pt_BR-faber-medium` (ONNX, run with `onnxruntime`) | see below |
| Base speech → the person's timbre | OpenVoice V2 tone color converter | MIT |
| Tone of the message (rate, pitch shift, gain) | Piper length scale + librosa | ISC (librosa) |

The engine does **no** grapheme-to-phoneme conversion: the Rust core already
decided how every word is pronounced (that is where the learned accent lives)
and sends the phonemes with stress and pauses. The engine only maps them to
the Piper voice inventory, so the accent reaches the audio unchanged.

OpenVoice's converter also moves the pitch towards the reference speaker, so
the only pitch change applied afterwards is the one requested by the tone.

## Install

Requires Python 3.10+.

```bash
python -m venv .venv && . .venv/bin/activate
pip install torch --index-url https://download.pytorch.org/whl/cpu   # or a CUDA build
pip install -e ".[dev]"
# OpenVoice from a pinned commit, without its (very old) pinned dependencies:
pip install --no-deps -r requirements-openvoice.txt
friendly-voice-engine download      # weights, verified by SHA-256
friendly-voice-engine serve --host 127.0.0.1 --port 8020
```

Docker (CPU, weights baked into the image):

```bash
docker build -t friendly-voice-engine .
docker run -p 8020:8020 -v fv-voices:/data friendly-voice-engine
```

Optional features:

```bash
pip install -e ".[transcribe]"   # POST /v1/transcribe (faster-whisper)
pip install -e ".[phonemes]"     # POST /v1/phonemes (wav2vec2 phone recognizer):
                                 # learns the accent without hand-written IPA
```

Without them, those endpoints answer `501` and the app falls back to the
transcript / IPA files provided by the user.

## Configuration

| Variable | Default | Meaning |
|---|---|---|
| `FV_ENGINE_HOME` | `~/.cache/friendly-voice-engine` | base folder |
| `FV_MODELS_DIR` | `$FV_ENGINE_HOME/models` | model weights |
| `FV_VOICES_DIR` | `$FV_ENGINE_HOME/voices` | cloned voices (one speaker embedding each) |
| `FV_DEVICE` | `cpu` | `cpu`, `cuda`, `cuda:0`... |
| `FV_TAU` | `0.3` | OpenVoice conversion strength (lower = closer to the target timbre) |

On a 4-core CPU (no GPU), about 2 s of speech are produced in about 1 s.

## Tests

```bash
pytest -q                          # integration tests are skipped without the weights
../scripts/engine-e2e.sh           # Rust CLI ⇄ running server, end to end
```

## Recording tips

* 3 to 10 recordings of 5–20 s each, one speaker, little background noise.
* Natural speech (a conversation or a message) carries more of the accent than
  reading aloud.
* Provide what was said (`.txt`) and, ideally, what was heard (`.ipa`) — or
  enable `[phonemes]` — so that the accent can be learned too.

## Licenses of the models

* OpenVoice V1/V2 code and weights: MIT (MyShell.ai, since April 2024).
* Piper `pt_BR-faber-medium`: dataset CC0, fine-tuned from Piper's *lessac*
  voice, whose dataset (Blizzard 2013) has its own license — review it before
  commercial distribution, or replace the base voice.
* XTTS v2 was **not** used: its Coqui Public Model License allows only
  non-commercial use (see ADR 0006).
