# CPU image of the Friendly Voice engine, with model weights baked in.
#   docker build -t friendly-voice-engine engine/
#   docker run -p 8020:8020 -v fv-voices:/data friendly-voice-engine
FROM python:3.11-slim

RUN apt-get update \
 && apt-get install -y --no-install-recommends git libsndfile1 \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY pyproject.toml requirements-openvoice.txt README.md ./
COPY friendly_voice_engine ./friendly_voice_engine

# CPU-only PyTorch first, so that pip does not pull the CUDA build.
RUN pip install --no-cache-dir torch --index-url https://download.pytorch.org/whl/cpu \
 && pip install --no-cache-dir . \
 && pip install --no-cache-dir --no-deps -r requirements-openvoice.txt

ENV FV_MODELS_DIR=/models \
    FV_VOICES_DIR=/data/voices
RUN friendly-voice-engine download

VOLUME ["/data"]
EXPOSE 8020
CMD ["friendly-voice-engine", "serve", "--host", "0.0.0.0", "--port", "8020"]
