# Contrato do servidor de voz neural

Usado quando o app é compilado com `--features http-engine` e executado com
`--motor-url http://host:porta`. Pode ser implementado sobre XTTS v2,
OpenVoice, F5-TTS (voz), Whisper (transcrição) e wav2vec2-phoneme/Allosaurus
(fonemas). Todos os áudios trafegam como WAV em base64.

## `POST /v1/voices` — cria a voz (timbre)
```json
{ "name": "Vó Lúcia", "language": "pt-BR",
  "references": [ { "audio_wav_base64": "...", "transcript": "Leite quente..." } ] }
```
Resposta: `{ "voice_id": "abc123" }`

## `POST /v1/synthesize` — fala com a voz e o sotaque
```json
{ "voice_id": "abc123", "language": "pt-BR",
  "text": "Bom dia, Evandro!",
  "ssml": "<speak ...><prosody rate=\"95%\" pitch=\"210Hz\"><phoneme alphabet=\"ipa\" ph=\"bõ\">bom</phoneme> ...",
  "ipa_words": ["bõ", "dʒia", "evãdɾu"],
  "pitch_hz": 210.0, "syllables_per_second": 5.2 }
```
Resposta: bytes `audio/wav`.

O servidor **deve** priorizar a pronúncia de `ssml`/`ipa_words` (é ali que está
o sotaque aprendido) — p.ex. alimentando o modelo com fonemas em vez de texto.

## `POST /v1/transcribe` — opcional
```json
{ "language": "pt-BR", "audio_wav_base64": "...", "transcript": null }
```
Resposta: `{ "text": "leite quente de noite" }`

## `POST /v1/phonemes` — opcional (habilita aprender o sotaque sem anotar)
```json
{ "language": "pt-BR", "audio_wav_base64": "...", "transcript": "leite quente" }
```
Resposta: `{ "ipa": "lejtʃi kẽtʃi" }`
