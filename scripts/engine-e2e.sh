#!/usr/bin/env bash
# End-to-end check: Rust CLI (http-engine) ⇄ voice engine server.
#
# Expects the engine to be running (default http://127.0.0.1:8020) and the
# CLI to be built with `cargo build --features http-engine`.
# Usage: scripts/engine-e2e.sh [engine-url]
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
URL="${1:-http://127.0.0.1:8020}"
FV="$ROOT/target/debug/friendly-voice"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT
export FRIENDLY_VOICE_DATA="$WORK/data" FRIENDLY_VOICE_ENGINE_URL="$URL"

# Reference recordings: voiced, harmonic signals with syllable-like pauses,
# plus the transcript (.txt) and what was heard (.ipa) next to each file.
python3 - "$WORK" <<'PY'
import math, struct, sys, wave
work = sys.argv[1]
samples = {
    "a1": (210, "Leite quente de noite, tudo certo.", "lejtʃi kẽtʃi dʒi nojtʃi tudu sɛhtu"),
    "a2": (205, "Os amigos de verdade vão falar com você.", "uz amiguʃ dʒi vehdadʒi vãw fala kũ vose"),
    "a3": (215, "A festa é mesmo na porta do mar.", "a fɛʃta ɛ meʒmu na pɔhta du ma"),
}
for name, (f0, text, ipa) in samples.items():
    sr, frames = 16000, []
    for i in range(6 * sr):
        t = i / sr
        v = sum(math.sin(2 * math.pi * f0 * k * t) / k for k in range(1, 6)) * 0.15 if (t % 0.5) < 0.35 else 0.0
        frames.append(struct.pack("<h", int(v * 32767)))
    with wave.open(f"{work}/{name}.wav", "wb") as w:
        w.setnchannels(1); w.setsampwidth(2); w.setframerate(sr); w.writeframes(b"".join(frames))
    open(f"{work}/{name}.txt", "w").write(text)
    open(f"{work}/{name}.ipa", "w").write(ipa)
PY

id=$("$FV" person add --name "Vó" --consent-by "Vó" | sed -n 's/.*id: \(.*\))/\1/p')
for f in a1 a2 a3; do "$FV" sample add --person "$id" --audio "$WORK/$f.wav" >/dev/null; done
"$FV" voice generate --person "$id" | grep -q "neural server (HTTP)"
"$FV" speak --person "$id" --theme good-morning --tone warm
"$FV" speak --person "$id" --text "Respira fundo. Vai dar tudo certo." --tone calm --pitch-shift -1

for wav in "$FRIENDLY_VOICE_DATA"/outbox/*.wav; do
  python3 -c "import sys, wave; w = wave.open(sys.argv[1]); d = w.getnframes() / w.getframerate(); assert d > 1.0, d; print(f'{sys.argv[1]}: {d:.1f}s')" "$wav"
done
echo "engine end-to-end: OK"
