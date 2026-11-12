# Neural voice server contract

Used when the app is built with `--features http-engine` and run with
`--motor-url http://host:port` (or `FRIENDLY_VOICE_ENGINE_URL`). The reference
implementation is [`engine/`](../engine) (Piper pt-BR + OpenVoice V2), but any
server implementing these endpoints can be plugged in. All audio travels as
base64-encoded WAV (the app converts other input formats, such as FLAC, to WAV before sending).

Errors are returned with a non-2xx status and a JSON body
`{"detail": "..."}`, which the app shows to the user. Optional endpoints that
are not available answer **501**.

## `POST /v1/voices` — create the voice (timbre)
```json
{ "name": "Vó Lúcia", "language": "pt-BR",
  "references": [ { "audio_wav_base64": "...", "transcript": "Leite quente..." } ] }
```
Response: `{ "voice_id": "3eb6a366...", "reference_count": 3, "reference_seconds": 17.0 }`

## `POST /v1/synthesize` — speak with the voice, the accent and the tone
```json
{ "voice_id": "3eb6a366...", "language": "pt-BR",
  "text": "Bom dia, Evandro!",
  "words": [
    { "phonemes": ["b", "õ"], "stress": 1, "pause_after": false },
    { "phonemes": ["dʒ", "i", "a"], "stress": 1, "pause_after": true },
    { "phonemes": ["e", "v", "ã", "d", "ɾ", "u"], "stress": 2, "pause_after": false }
  ],
  "ipa_words": ["bõ", "dʒia", "evãdɾu"],
  "ssml": "<speak ...><prosody rate=\"95%\" pitch=\"210Hz\"><phoneme alphabet=\"ipa\" ph=\"bõ\">bom</phoneme> ...",
  "pitch_hz": 210.0, "syllables_per_second": 5.2,
  "style": "carinhoso", "pitch_shift_semitones": -0.5, "volume_db": -2.0 }
```
Response: `audio/wav` bytes.

* `words` is the pronunciation **with the learned accent already applied**,
  one entry per word: phonemes (IPA, affricates as one symbol), index of the
  stressed phoneme and whether a pause follows. The server must speak these
  phonemes rather than re-reading `text`. `ipa_words` and `ssml` carry the same
  pronunciation for engines that only accept IPA strings or SSML.
* `pitch_hz` and `syllables_per_second` already include the effect of the
  chosen tone. `style` (natural, carinhoso, animado, calmo, firme, sussurrado)
  can be mapped to the model's emotion/style control when there is one;
  `pitch_shift_semitones` and `volume_db` are the tone's adjustments relative
  to the person's natural voice.

## `POST /v1/transcribe` — optional
```json
{ "language": "pt-BR", "audio_wav_base64": "...", "transcript": null }
```
Response: `{ "text": "leite quente de noite" }`

## `POST /v1/phonemes` — optional (learn the accent without annotating)
```json
{ "language": "pt-BR", "audio_wav_base64": "...", "transcript": "leite quente" }
```
Response: `{ "ipa": "lejtʃi kẽtʃi" }`. On 501 the app stores the sample
without accent evidence instead of failing.

## `GET /health`
Response: `{ "status": "ok", "transcribe": false, "phonemes": false, ... }`
