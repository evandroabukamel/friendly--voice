# Image of the Friendly Voice engine, with model weights baked in.
#   docker build -t friendly-voice-engine engine/
#   docker run --gpus all -p 8020:8020 -v fv-voices:/data friendly-voice-engine   # NVIDIA GPU
#   docker run -p 8020:8020 -v fv-voices:/data friendly-voice-engine              # CPU (slower)
# For a smaller CPU-only image:
#   docker build --build-arg TORCH_INDEX=https://download.pytorch.org/whl/cpu -t friendly-voice-engine engine/
FROM python:3.11-slim

ARG TORCH_INDEX=https://download.pytorch.org/whl/cu124

RUN apt-get update \
 && apt-get install -y --no-install-recommends libsndfile1 \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY pyproject.toml README.md ./
COPY friendly_voice_engine ./friendly_voice_engine

# PyTorch first, from the chosen index (CUDA or CPU), at the version chatterbox-tts pins.
RUN pip install --no-cache-dir torch==2.6.0 torchaudio==2.6.0 --index-url ${TORCH_INDEX} \
 && pip install --no-cache-dir ".[transcribe]"

ENV HF_HOME=/models \
    FV_VOICES_DIR=/data/voices \
    FV_DEVICE=auto
RUN friendly-voice-engine download

VOLUME ["/data"]
EXPOSE 8020
CMD ["friendly-voice-engine", "serve", "--host", "0.0.0.0", "--port", "8020"]
