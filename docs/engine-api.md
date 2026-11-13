# Neural voice server contract

Used when the app is built with `--features http-engine` and run with
`--engine-url http://host:port` (or `FRIENDLY_VOICE_ENGINE_URL`). The reference
implementation is [`engine/`](../engine) (Chatterbox Multilingual, zero-shot
cloning), but any server implementing these endpoints can be plugged in. All
audio travels as base64-encoded WAV (the app converts other input formats,
such as FLAC, to WAV before sending).

Languages are BCP 47 tags (`pt-BR`, `en`, `es-MX`); the server uses the
primary subtag and answers **422** for a language it cannot speak.

Errors are returned with a non-2xx status and a JSON body
`{"detail": "..."}`, which the app shows to the user. Optional endpoints that
are not available answer **501**.

## `POST /v1/voices` — clone the voice
```json
{ "name": "Vó Lúcia", "language": "pt-BR", "gender": "female",
  "references": [ { "audio_wav_base64": "...", "transcript": "Leite quente..." },
                  { "audio_wav_base64": "...", "transcript": null } ] }
```
Response: `{ "voice_id": "3eb6a366...", "reference_count": 3, "reference_seconds": 17.0, "median_pitch_hz": 231.4 }`

* `references` are all the person's recordings; `transcript` is optional.
* `gender` is detected by the app from the recordings (`female`, `male` or
  `null`); a hint for engines that pick a base voice.
* The reference engine averages the speaker embedding over every recording
  and keeps, as the reference the model imitates, the 10 s window whose own
  embedding is closest to that average.

## `POST /v1/synthesize` — speak a text with the voice and the tone
```json
{ "voice_id": "3eb6a366...", "language": "pt-BR",
  "text": "Bom dia, Evandro! Hoje é dia de coisa boa.",
  "style": "warm", "pitch_shift_semitones": -1.0, "volume_db": -2.0 }
```
Response: `audio/wav` bytes.

* The server reads `text` itself, in `language`.
* `style` (natural, warm, cheerful, calm, firm, whisper) maps to the model's
  expressiveness controls; `pitch_shift_semitones` is the extra shift chosen
  by the user and `volume_db` the tone's gain.
* The reference engine speaks sentence by sentence (≤ 220 characters each)
  and retries a take whose duration shows it was cut off or rambled.

## `POST /v1/transcribe` — optional
```json
{ "language": "pt-BR", "audio_wav_base64": "..." }
```
Response: `{ "text": "leite quente de noite" }`. On 501 the app stores the
sample without a transcript.

## `GET /health`
Response: `{ "status": "ok", "version": "0.2.0", "device": "cuda", "languages": ["ar", ...], "transcribe": true }`
