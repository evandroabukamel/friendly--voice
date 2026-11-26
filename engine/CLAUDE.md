# engine/ — Python voice server (Chatterbox Multilingual)

Read before touching `engine/`, `crates/infrastructure/src/voice/http.rs` or
the HTTP contract. Contract: [`docs/engine-api.md`](../docs/engine-api.md).

## Which engine runs

| Build / config | Engine | Result |
|---|---|---|
| default build, no URL | local prosody **sketch** | person's rhythm and melody, **not** the voice — for dev/tests |
| `--features http-engine` + `--engine-url` / `FRIENDLY_VOICE_ENGINE_URL` | `HttpVoiceEngine` → `engine/` | cloned voice + language + tone |

The Flutter app takes the URL from the build (`--dart-define`), never from
the user: see [`flutter_app/CLAUDE.md`](../flutter_app/CLAUDE.md).

## Endpoints and environment

`POST /v1/voices`, `POST /v1/synthesize`, `DELETE /v1/voices/{id}`, optional
`POST /v1/transcribe` (faster-whisper `small` on the CPU, extra
`[transcribe]`; 501 when missing → sample stored without transcript),
`GET /health`. Env vars: `FV_VOICES_DIR`, `FV_ENGINE_HOME`, `FV_DEVICE`
(`auto`), `HF_HOME` (weights).

## Gotchas

- `chatterbox-tts==0.1.7` pins torch 2.6 / transformers; install torch from
  the CUDA or CPU index first (see `Dockerfile`, `TORCH_INDEX`).
- GPU: `docker run --gpus all`; ~4 s per sentence on a 6 GB laptop GPU.
  Whisper runs on the CPU so both models fit in 6 GB.
- The model object holds the current voice (`model.conds`): synthesis and
  enrollment are serialized with a lock in `engine.py`.
- Voices from engine 0.1 (OpenVoice) are not compatible: regenerate them.
- Style → (`exaggeration`, `cfg_weight`) table in `engine.py`; there is no
  direct speaking-rate control.
- Licences: Chatterbox is MIT and watermarks its output (Perth);
  `praat-parselmouth` is GPL-3.0. Only commercially usable weights (ADR
  0006/0007 explain why XTTS and CC-BY-NC F5-TTS-style weights are out).

## Commands

```bash
(cd engine && pytest -q)     # integration tests need chatterbox-tts
scripts/engine-e2e.sh        # CLI ⇄ running engine
docker build -t friendly-voice-engine engine/ && docker run --gpus all -p 8020:8020 -v fv-voices:/data friendly-voice-engine
```
